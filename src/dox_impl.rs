//! Actual implementation of doxing.

use frakti::{
    AsyncTelegramApi,
    client_cyper::Bot,
    methods::{GetChatMemberParams, GetChatParams},
    types::{
        Birthdate, BusinessLocation, Chat, ChatFullInfo, ChatMember, ExternalReplyInfo,
        MessageOrigin, User,
    },
};
use log::warn;
use std::fmt;

/// Identifier for either Telegram user or chat.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SubjectId {
    /// Telegram user ID.
    User(u64),
    /// Telegram chat/channel/group ID.
    Chat(i64),
}

impl SubjectId {
    /// Return user ID if subject is a user.
    #[must_use]
    pub(crate) const fn as_user_id(self) -> Option<u64> {
        match self {
            Self::User(id) => Some(id),
            Self::Chat(_) => None,
        }
    }

    /// Return chat ID usable with `get_chat`.
    #[must_use]
    pub(crate) fn chat_id_for_get_chat(self) -> Option<i64> {
        match self {
            Self::User(id) => i64::try_from(id).ok(),
            Self::Chat(id) => Some(id),
        }
    }
}

/// A report containing all available information about a user.
pub struct DoxReport {
    /// User or chat this report describes.
    pub subject: SubjectId,
    /// The user's username, if any.
    pub username: Option<String>,
    /// The user's title/tag in the chat or signature, if any.
    pub sender_title: Option<String>,
    /// The user's first name or chat name, if any.
    pub display_name: Option<String>,
    /// The user's last name, if any.
    pub last_name: Option<String>,
    /// If the user is premium.
    pub is_premium: Option<bool>,
    /// Birthdate, if any.
    pub birthdate: Option<Birthdate>,
    /// Business location, if any.
    pub business_location: Option<BusinessLocation>,
    /// Personal channel, if any.
    pub personal_chat: Option<Chat>,
}

impl DoxReport {
    /// Create a new [`DoxReport`] from given given user, title and full info.
    #[must_use]
    fn new(user: User, title: Option<String>, full_info: Option<ChatFullInfo>) -> Self {
        let mut report = Self::from_user(user);
        if title.is_some() {
            report = report.with_title(title);
        }
        if let Some(full_info) = full_info {
            report = report.with_full_info(full_info);
        }
        report
    }

    // Helper methods to create a new [`DoxReport`] from different sources of information.
    /// Create a new [`DoxReport`] from given [`User`].
    #[must_use]
    pub(crate) fn from_user(user: User) -> Self {
        Self {
            subject: SubjectId::User(user.id),
            username: user.username,
            sender_title: None,
            display_name: Some(user.first_name),
            last_name: user.last_name,
            is_premium: user.is_premium,
            birthdate: None,
            business_location: None,
            personal_chat: None,
        }
    }
    /// Create a new [`DoxReport`] from given [`Chat`].
    #[must_use]
    fn from_chat(chat: Chat) -> Self {
        Self {
            subject: SubjectId::Chat(chat.id),
            username: chat.username,
            sender_title: None, // Chat doesn't have title/tag, so we leave it empty. It can be filled later from `Message::author_signature` with `with_title`.
            display_name: chat.title, // For chats, the title field is used to store the chat's name, so we put it in first_name.
            last_name: None,
            is_premium: None,
            birthdate: None,
            business_location: None,
            personal_chat: None,
        }
    }
    /// Create a new [`DoxReport`] from message sender fields.
    #[must_use]
    pub(crate) fn from_sender(
        from: Option<Box<User>>,
        sender_chat: Option<Box<Chat>>,
        sender_title: Option<String>,
    ) -> Option<Self> {
        let report = if let Some(chat) = sender_chat {
            Self::from_chat(*chat)
        } else if let Some(user) = from {
            Self::from_user(*user)
        } else {
            return None;
        };
        Some(report.with_title(sender_title))
    }
    /// Create a completed [`DoxReport`] from a forwarded/external message origin.
    pub(crate) async fn from_origin(
        bot: &Bot,
        origin: MessageOrigin,
        chat_id: Option<i64>,
    ) -> Option<Self> {
        match origin {
            MessageOrigin::User(origin_user) => Some(
                Self::from_user(origin_user.sender_user)
                    .complete_title(bot, chat_id)
                    .await
                    .complete_full_info(bot)
                    .await,
            ),
            MessageOrigin::Channel(origin_channel) => Some(
                Self::from_chat(origin_channel.chat).with_title(origin_channel.author_signature),
            ),
            MessageOrigin::Chat(origin_chat) => Some(
                Self::from_chat(origin_chat.sender_chat).with_title(origin_chat.author_signature),
            ),
            MessageOrigin::HiddenUser(_) => None,
        }
    }
    /// Create a completed [`DoxReport`] from an external reply info.
    pub(crate) async fn from_external_reply(
        bot: &Bot,
        external: ExternalReplyInfo,
    ) -> Option<Self> {
        let chat_id = external.chat.map(|chat| chat.id);
        Self::from_origin(bot, external.origin, chat_id).await
    }
    /// Try to create a new completed [`DoxReport`] from given user id and optional chat id, returning None if it fails.
    pub(crate) async fn from_id(bot: &Bot, user_id: u64, chat_id: Option<i64>) -> Option<Self> {
        let (user, title) = get_user_title_by_id(bot, user_id, chat_id).await?;
        let report = Self::new(user, title, None);
        let report = report.complete_full_info(bot).await;
        Some(report)
    }

    // Helper methods to update fields of an existing [`DoxReport`].
    /// Add title/tag to the [`DoxReport`].
    #[must_use]
    fn with_title(mut self, title: Option<String>) -> Self {
        self.sender_title = title;
        self
    }
    /// Update the [`DoxReport`] with given [`ChatFullInfo`], keeping existing fields if the new info doesn't have them.
    fn with_full_info(mut self, full_info: ChatFullInfo) -> Self {
        if self.username.is_none() {
            self.username = full_info.username;
        }
        if self.display_name.is_none() {
            self.display_name = full_info.first_name;
        }
        if self.last_name.is_none() {
            self.last_name = full_info.last_name;
        }
        if self.birthdate.is_none() {
            self.birthdate = full_info.birthdate;
        }
        if self.business_location.is_none() {
            self.business_location = full_info.business_location;
        }
        if self.personal_chat.is_none() {
            self.personal_chat = full_info.personal_chat.map(|c| *c);
        }
        self
    }
    /// Complete the title of the report.
    async fn complete_title(mut self, bot: &Bot, chat_id: Option<i64>) -> Self {
        if self.sender_title.is_none()
            && let Some(user_id) = self.subject.as_user_id()
            && let Some((_, title)) = get_user_title_by_id(bot, user_id, chat_id).await
        {
            self.sender_title = title;
        }
        self
    }
    /// Complete the full info of the report.
    pub(crate) async fn complete_full_info(mut self, bot: &Bot) -> Self {
        if (self.birthdate.is_none()
            || self.business_location.is_none()
            || self.personal_chat.is_none())
            && let Some(chat_id) = self.subject.chat_id_for_get_chat()
            && let Some(full_info) = get_full_info(bot, chat_id).await
        {
            self = self.with_full_info(full_info);
        }
        self
    }

    pub(crate) fn inline_title(&self) -> String {
        format!(
            "开盒 {}",
            self.display_name
                .as_deref()
                .or(self.last_name.as_deref())
                .or(self.username.as_deref())
                .map_or_else(
                    || match self.subject {
                        SubjectId::User(id) => id.to_string(),
                        SubjectId::Chat(id) => id.to_string(),
                    },
                    ToOwned::to_owned,
                )
        )
    }
}

impl fmt::Display for DoxReport {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self.subject {
            SubjectId::User(id) => write!(f, "您好，请问是用户 ID 为 <code>{id}</code>")?,
            SubjectId::Chat(id) => write!(f, "您好，请问是群聊/频道 ID 为 <code>{id}</code>")?,
        }
        if let Some(username) = &self.username {
            write!(f, "，用户名为 <code>@{username}</code>")?;
        }
        if let Some(title) = &self.sender_title {
            write!(f, "，头衔为 <code>{}</code>", escape(title))?;
        }
        if let Some(birthday) = &self.birthdate {
            let Birthdate { year, month, day } = birthday;
            match year {
                None => write!(f, "，生日在 {month:02} 月 {day:02} 日")?,
                Some(year) => write!(f, "，生日在 {year:04} 年 {month:02} 月 {day:02} 日")?,
            }
        }
        if let Some(business_location) = &self.business_location {
            write!(
                f,
                "，位于 <code>{}</code>",
                escape(&business_location.address)
            )?;
            if let Some(location) = &business_location.location {
                write!(
                    f,
                    "（经度：{}，纬度：{}）",
                    location.longitude, location.latitude
                )?;
            }
        }
        if let Some(channel) = &self.personal_chat {
            if let Some(channel_username) = &channel.username {
                write!(f, "，开通了 tg 空间 @{channel_username}")?;
            } else {
                warn!("Cannot get username of personal channel: {}", channel.id);
            }
        }
        match self.subject {
            SubjectId::User(_) => {
                if let Some(display_name) = &self.display_name {
                    write!(f, " 的 <code>{}", escape(display_name))?;
                } else {
                    write!(f, " 的 <code>")?;
                }
                if let Some(last_name) = &self.last_name {
                    write!(f, " {}", escape(last_name))?;
                }
                write!(f, "</code> ")?;
                if fish_cake(self.display_name.as_ref()) || fish_cake(self.last_name.as_ref()) {
                    write!(f, "南梁")?;
                } else if self.is_premium == Some(true) {
                    write!(f, "富哥")?;
                } else {
                    write!(f, "先生")?;
                }
            }
            SubjectId::Chat(_) => {
                let name = self
                    .display_name
                    .as_deref()
                    .or(self.last_name.as_deref())
                    .unwrap_or("");
                write!(f, " 的 <code>{}</code> ", escape(name))?;
            }
        }
        write!(f, "吗？")
    }
}

// TODO: Cache
/// Try to get full info about the user.
async fn get_full_info(bot: &Bot, chat_id: i64) -> Option<ChatFullInfo> {
    let get_params = GetChatParams::builder().chat_id(chat_id).build();
    let Ok(result) = bot
        .get_chat(&get_params)
        .await
        .inspect_err(|e| warn!("Error querying {chat_id}: {e:?}"))
    else {
        return None;
    };
    Some(result.result)
}

// TODO: Cache
/// Try to get [`User`] and his title/tag from given id.
async fn get_user_title_by_id(
    bot: &Bot,
    user_id: u64,
    chat_id: Option<i64>,
) -> Option<(User, Option<String>)> {
    /// Try to convert u64 to i64, returning None and logging a warning if it fails.
    fn try_i64_from_u64(n: u64) -> Option<i64> {
        i64::try_from(n)
            .inspect_err(|e| warn!("[get_user_title_by_id] Cannot convert {n} to i64: {e:?}"))
            .ok()
    }
    let chat_id = match chat_id {
        Some(id) => id,
        None => try_i64_from_u64(user_id)?,
    };
    let get_params = GetChatMemberParams::builder()
        .chat_id(chat_id)
        .user_id(user_id)
        .build();
    let result = match bot.get_chat_member(&get_params).await {
        Ok(result) => result,
        Err(e) => {
            // Fallback to chat_id = user_id
            let fallback_chat_id = try_i64_from_u64(user_id)?;
            if fallback_chat_id == chat_id {
                warn!(
                    "Cannot get user with id {user_id} in chat {chat_id}: {e:?}, no fallback available"
                );
                return None;
            }
            warn!("Cannot get user with id {user_id} in chat {chat_id}: {e:?}, trying fallback...");
            let fallback_params = GetChatMemberParams::builder()
                .chat_id(fallback_chat_id)
                .user_id(user_id)
                .build();
            match bot.get_chat_member(&fallback_params).await {
                Ok(result) => result,
                Err(e) => {
                    warn!("Fallback failed for user_id {user_id}: {e:?}");
                    return None;
                }
            }
        }
    };
    let user_and_title = match result.result {
        ChatMember::Administrator(admin) => (admin.user, admin.custom_title),
        ChatMember::Creator(creator) => (creator.user, creator.custom_title),
        ChatMember::Kicked(kicked) => (kicked.user, None),
        ChatMember::Left(left) => (left.user, None),
        ChatMember::Member(member) => (member.user, member.tag),
        ChatMember::Restricted(restricted) => (restricted.user, restricted.tag),
    };
    Some(user_and_title)
}

/// Whether the given string contains "🍥" or "🏳️‍⚧️".
fn fish_cake(s: Option<&String>) -> bool {
    s.is_some_and(|s| s.contains('🍥') || s.contains("🏳️‍⚧️"))
}

/// Escapes the given string, as mentioned by [the docs](https://core.telegram.org/bots/api#html-style) on Telegram.
fn escape(s: &str) -> String {
    s.replace('&', "&amp;")
        .replace('<', "&lt;")
        .replace('>', "&gt;")
}

#[cfg(test)]
mod tests {
    use super::{DoxReport, SubjectId};

    #[test]
    fn user_subject_converts_to_chat_id_when_possible() {
        assert_eq!(SubjectId::User(123).chat_id_for_get_chat(), Some(123));
    }

    #[test]
    fn chat_subject_never_becomes_user_id() {
        assert_eq!(SubjectId::Chat(-1_000_000_000_000).as_user_id(), None);
    }

    #[test]
    fn chat_display_uses_chat_id_without_wrapping() {
        let report = DoxReport {
            subject: SubjectId::Chat(-1_000_000_000_000),
            username: None,
            sender_title: Some("Test".to_string()),
            display_name: None,
            last_name: None,
            is_premium: None,
            birthdate: None,
            business_location: None,
            personal_chat: None,
        };

        let rendered = report.to_string();
        assert!(rendered.contains("群聊/频道 ID 为 <code>-1000000000000</code>"));
        assert!(!rendered.contains("先生"));
    }
}

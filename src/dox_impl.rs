//! Actual implementation of doxing.

use frakti::{
    AsyncTelegramApi,
    client_cyper::Bot,
    methods::{GetChatMemberParams, GetChatParams},
    types::{Birthdate, BusinessLocation, Chat, ChatFullInfo, ChatMember, User},
};
use log::warn;
use std::fmt;

/// A report containing all available information about a user.
pub struct DoxReport {
    /// The user's ID as u64.
    pub id: u64,
    /// The chat ID as i64.
    pub chat_id: i64,
    /// The user's username, if any.
    pub username: Option<String>,
    /// The user's title/tag in the chat, if any.
    pub title: Option<String>,
    /// The user's first name, if any.
    pub first_name: Option<String>,
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
    pub fn new(user: User, title: Option<String>, full_info: Option<ChatFullInfo>) -> Self {
        let mut report = Self::from_user(user);
        if title.is_some() {
            report = report.with_title(title);
        }
        if let Some(full_info) = full_info {
            report = report.with_full_info(full_info);
        }
        report
    }

    /// Create a new [`DoxReport`] from given [`User`].
    pub fn from_user(user: User) -> Self {
        Self {
            id: user.id,
            chat_id: user.id.try_into().unwrap_or_else(|e| {
                warn!("Cannot convert user id {} to chat id: {e:?}", user.id);
                0
            }),
            username: user.username,
            title: None,
            first_name: Some(user.first_name),
            last_name: user.last_name,
            is_premium: user.is_premium,
            birthdate: None,
            business_location: None,
            personal_chat: None,
        }
    }
    /// Create a new [`DoxReport`] from given [`Chat`].
    pub fn from_chat(chat: Chat) -> Self {
        Self {
            id: chat.id.try_into().unwrap_or_else(|e| {
                warn!("Cannot convert chat id {} to user id: {e:?}", chat.id);
                0
            }),
            chat_id: chat.id,
            username: chat.username,
            title: chat.title,
            first_name: None,
            last_name: None,
            is_premium: None,
            birthdate: None,
            business_location: None,
            personal_chat: None,
        }
    }
    /// Create a new [`DoxReport`] from given [`ChatFullInfo`].
    pub fn from_full_info(full_info: ChatFullInfo) -> Self {
        Self {
            id: full_info.id.try_into().unwrap_or_else(|e| {
                warn!("Cannot convert chat id {} to user id: {e:?}", full_info.id);
                0
            }),
            chat_id: full_info.id,
            username: full_info.username,
            title: None,
            first_name: full_info.first_name,
            last_name: full_info.last_name,
            is_premium: None,
            birthdate: full_info.birthdate,
            business_location: full_info.business_location,
            personal_chat: full_info.personal_chat.map(|c| *c),
        }
    }
    /// Try to create a new completed [`DoxReport`] from given user id and optional chat id, returning None if it fails.
    pub async fn from_id(bot: &Bot, user_id: u64, chat_id: Option<i64>) -> Option<Self> {
        let (user, title) = get_user_title_by_id(bot, user_id, chat_id).await?;
        let report = Self::new(user, title, None);
        let report = report.complete_full_info(bot).await;
        Some(report)
    }

    /// Add title/tag to the [`DoxReport`].
    pub fn with_title(mut self, title: Option<String>) -> Self {
        self.title = title;
        self
    }
    /// Update the [`DoxReport`] with given [`ChatFullInfo`], keeping existing fields if the new info doesn't have them.
    fn with_full_info(mut self, full_info: ChatFullInfo) -> Self {
        if self.username.is_none() {
            self.username = full_info.username;
        }
        if self.first_name.is_none() {
            self.first_name = full_info.first_name;
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
    pub async fn complete_title(mut self, bot: &Bot, chat_id: Option<i64>) -> Self {
        if self.title.is_none() {
            if let Some((_, title)) = get_user_title_by_id(bot, self.id, chat_id).await {
                self.title = title;
            }
        }
        self
    }
    /// Complete the full info of the report.
    pub async fn complete_full_info(mut self, bot: &Bot) -> Self {
        if self.birthdate.is_none()
            || self.business_location.is_none()
            || self.personal_chat.is_none()
        {
            if let Some(full_info) = get_full_info(bot, self.chat_id).await {
                self = self.with_full_info(full_info);
            }
        }
        self
    }
}

impl fmt::Display for DoxReport {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "您好，请问是用户 ID 为 <code>{}</code>", self.id)?;
        if let Some(username) = &self.username {
            write!(f, "，用户名为 <code>@{username}</code>")?;
        }
        if let Some(title) = &self.title {
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
            write!(f, "，位于 {}", escape(&business_location.address))?;
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
        if let Some(first_name) = &self.first_name {
            write!(f, " 的 <code>{}", escape(first_name))?;
        } else {
            write!(f, " 的 <code>")?;
        }
        if let Some(last_name) = &self.last_name {
            write!(f, " {}", escape(last_name))?;
        }
        write!(f, "</code> ")?;
        if fish_cake(&self.first_name) || fish_cake(&self.last_name) {
            write!(f, "南梁")?;
        } else if self.is_premium == Some(true) {
            write!(f, "富哥")?;
        } else {
            write!(f, "先生")?;
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
fn fish_cake(s: &Option<String>) -> bool {
    s.as_ref()
        .map(|s| s.find('🍥').is_some() || s.find("🏳️‍⚧️").is_some())
        .unwrap_or_default()
}

/// Escapes the given string, as mentioned by [the docs](https://core.telegram.org/bots/api#html-style) on Telegram.
fn escape(s: &str) -> String {
    s.replace('&', "&amp;")
        .replace('<', "&lt;")
        .replace('>', "&gt;")
}

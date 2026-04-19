//! Actual implementation of doxing.

use frakti::{
    AsyncTelegramApi,
    client_cyper::Bot,
    methods::{GetChatMemberParams, GetChatParams},
    types::{Birthdate, ChatFullInfo, ChatMember, ChatType, User},
};
use log::warn;
use std::fmt;

/// A report of doxing a user, containing the user, their title/tag and their full info if possible.
pub struct DoxReport {
    /// The user.
    pub user: User,
    /// The user's title/tag in the chat, if any.
    pub title: Option<String>,
    /// The user's full info, if possible to get.
    pub full_info: Option<ChatFullInfo>,
}

impl DoxReport {
    /// Create a new [`DoxInfo`] from given user, title and full info.
    pub fn new(user: User, title: Option<String>, full_info: Option<ChatFullInfo>) -> Self {
        Self {
            user,
            title,
            full_info,
        }
    }
}

impl fmt::Display for DoxReport {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "您好，请问是用户 ID 为 <code>{}</code>", self.user.id)?;
        if let Some(username) = &self.user.username {
            write!(f, "，用户名为 <code>@{username}</code>")?;
        }
        if let Some(title) = &self.title {
            write!(f, "，头衔为 <code>{}</code>", escape(title))?;
        }
        if let Some(full_info) = &self.full_info {
            detailed_doxing(full_info, f)?;
        }
        write!(f, " 的 <code>{}", escape(&self.user.first_name))?;
        if let Some(last_name) = &self.user.last_name {
            write!(f, " {}", escape(last_name))?;
        }
        write!(f, "</code> ")?;
        if fish_cake(&self.user) {
            write!(f, "南梁")?;
        } else if self.user.is_premium == Some(true) {
            write!(f, "富哥")?;
        } else {
            write!(f, "先生")?;
        }
        write!(f, "吗？")
    }
}

/// Detailed doxing.
fn detailed_doxing(full_info: &ChatFullInfo, f: &mut fmt::Formatter<'_>) -> fmt::Result {
    if !matches!(full_info.type_field, ChatType::Private) {
        warn!("Trying to dox a non-private chat: {}", full_info.id);
        return Ok(());
    }
    if let Some(birthday) = &full_info.birthdate {
        let Birthdate { year, month, day } = birthday;
        match year {
            None => write!(f, "，生日在 {month:02} 月 {day:02} 日")?,
            Some(year) => write!(f, "，生日在 {year:04} 年 {month:02} 月 {day:02} 日")?,
        }
    }
    if let Some(business_location) = &full_info.business_location {
        write!(f, "，位于 {}", escape(&business_location.address))?;
        if let Some(location) = &business_location.location {
            write!(
                f,
                "（经度：{}，纬度：{}）",
                location.longitude, location.latitude
            )?;
        }
    }
    if let Some(channel) = &full_info.personal_chat {
        if let Some(channel_username) = &channel.username {
            write!(f, "，开通了 tg 空间 @{channel_username}")?;
        } else {
            warn!("Cannot get username of personal channel: {}", channel.id);
        }
    }

    Ok(())
}

/// Try to get a dox report of the user with given id.
pub async fn get_user_report(bot: &Bot, user_id: u64) -> Option<DoxReport> {
    let (user, title) = get_user_title_by_id(bot, user_id).await?;
    let full_info = get_full_info(bot, user_id).await;
    Some(DoxReport {
        user,
        title,
        full_info,
    })
}

// TODO: Cache
/// Try to get full info about the user.
pub async fn get_full_info(bot: &Bot, user_id: u64) -> Option<ChatFullInfo> {
    let Ok(chat_id) = i64::try_from(user_id).inspect_err(|e| {
        warn!("[get_full_info] Cannot convert user_id {user_id} to chat_id: {e:?}")
    }) else {
        return None;
    };
    let get_params = GetChatParams::builder().chat_id(chat_id).build();
    let Ok(result) = bot
        .get_chat(&get_params)
        .await
        .inspect_err(|e| warn!("Error querying {user_id}: {e:?}"))
    else {
        return None;
    };
    Some(result.result)
}

// TODO: Cache
/// Try to get [`User`] and his title/tag from given id.
async fn get_user_title_by_id(bot: &Bot, user_id: u64) -> Option<(User, Option<String>)> {
    let chat_id = match i64::try_from(user_id) {
        Ok(id) => id,
        Err(e) => {
            warn!("[get_user_by_id] Cannot convert user_id {user_id} to chat_id: {e:?}");
            return None;
        }
    };
    let get_params = GetChatMemberParams::builder()
        .chat_id(chat_id)
        .user_id(user_id)
        .build();
    let Ok(result) = bot
        .get_chat_member(&get_params)
        .await
        .inspect_err(|e| warn!("Cannot get user from id {user_id}: {e:?}"))
    else {
        return None;
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

/// Whether the given [`User`]'s name contains "🍥" or "🏳️‍⚧️".
fn fish_cake(user: &User) -> bool {
    fn has_fish_cake(s: &String) -> bool {
        s.find('🍥').is_some() || s.find("🏳️‍⚧️").is_some()
    }
    has_fish_cake(&user.first_name)
        || user
            .last_name
            .as_ref()
            .map(has_fish_cake)
            .unwrap_or_default()
}

/// Escapes the given string, as mentioned by [the docs](https://core.telegram.org/bots/api#html-style) on Telegram.
fn escape(s: &str) -> String {
    s.replace('&', "&amp;")
        .replace('<', "&lt;")
        .replace('>', "&gt;")
}

//! Actual implementation of doxing.

use frankenstein::{
    AsyncTelegramApi,
    client_reqwest::Bot,
    methods::GetChatParams,
    types::{Birthdate, ChatFullInfo, ChatType, User},
};
use log::warn;
use std::fmt::Write;

/// Dox given [`User`] and optional [`ChatFullInfo`].
pub fn dox(doxee: User, full_info: Option<ChatFullInfo>) -> String {
    // Generate doxing report
    let mut report = String::new();
    // User ID
    let id = doxee.id;
    if let Err(e) = write!(report, "您好，请问是用户 ID 为 <code>{id}</code>") {
        warn!("Cannot write to report: {e}");
    }
    // Username
    if let Some(username) = doxee.username {
        if let Err(e) = write!(report, "，用户名为 <code>@{username}</code>") {
            warn!("Cannot write to report: {e}");
        }
    }
    // Detailed doxing if full info provided
    if let Some(full_info) = full_info
        && let Some(detail) = detailed_doxing(full_info)
    {
        report.push_str(&detail);
    }
    // Names & finish report
    report.push_str(" 的 <code>");
    let first_name = &doxee.first_name;
    report.push_str(&escape(first_name));
    if let Some(last_name) = &doxee.last_name {
        report.push(' ');
        report.push_str(&escape(last_name));
    }
    if doxee.is_premium == Some(true) {
        report.push_str("</code> 富哥吗？");
    } else {
        report.push_str("</code> 先生吗？");
    }

    report
}

/// Detailed doxing, only available if the user has contacted the bot.
fn detailed_doxing(full_info: ChatFullInfo) -> Option<String> {
    let user_id = full_info.id;
    let mut detail = String::new();
    if !matches!(full_info.type_field, ChatType::Private) {
        warn!("Trying to dox a non-private chat: {user_id}");
        return None;
    }
    if let Some(birthday) = full_info.birthdate {
        let Birthdate { year, month, day } = birthday;
        let result = match year {
            None => write!(detail, "，生日在 {month:02} 月 {day:02} 日"),
            Some(year) => write!(detail, "，生日在 {year:04} 年 {month:02} 月 {day:02} 日"),
        };
        if let Err(e) = result {
            warn!("Cannot write to detail: {e}");
        }
    }
    if let Some(channel) = full_info.personal_chat {
        if let Some(channel_username) = channel.username {
            if let Err(e) = write!(detail, "，开通了 tg 空间 @{channel_username}") {
                warn!("Cannot write to detail: {e}");
            }
        } else {
            warn!("Cannot get username of personal channel: {}", channel.id);
        }
    }

    Some(detail)
}

// TODO: Cache the result.
/// Try to get info about the user, only available if the user has contacted the bot.
pub async fn get_info(bot: &Bot, user_id: u64) -> Option<ChatFullInfo> {
    let chat_id = match i64::try_from(user_id) {
        Ok(id) => id,
        Err(e) => {
            warn!("Cannot convert user_id {user_id} to chat_id: {e:?}");
            return None;
        }
    };
    let get_params = GetChatParams::builder().chat_id(chat_id).build();
    match bot.get_chat(&get_params).await {
        Err(e) => {
            warn!("Error querying {user_id}: {e:?}");
            None
        }
        Ok(r) => Some(r.result),
    }
}

/// Escapes the given string, as mentioned by [the docs](https://core.telegram.org/bots/api#html-style) on Telegram.
fn escape(s: &str) -> String {
    s.replace('&', "&amp;")
        .replace('<', "&lt;")
        .replace('>', "&gt;")
    // TODO: More effiency by iterating over chars, estimating resulting size and creating new string
}

/// Try to get [`User`] from given id or username.
pub async fn get_user(bot: &Bot, identifier: String) -> Option<User> {
    if !identifier.is_ascii() {
        None
    } else if identifier.chars().all(|c| c.is_ascii_digit()) {
        let user_id: u64 = identifier.parse().ok()?;
        get_user_by_id(bot, user_id).await
    } else {
        // let username = identifier.trim_start_matches('@');
        let mut username = identifier;
        if username.starts_with('@') {
            username.drain(..1);
        }
        get_user_by_username(bot, username).await
    }
}

// TODO: Cache the result.
/// Try to get [`User`] from given id.
async fn get_user_by_id(bot: &Bot, user_id: u64) -> Option<User> {
    // TODO: Get user by id
    None
}

// TODO: Cache the result.
/// Try to get [`User`] from given username. Note that the provided username mustn't start with `@`.
async fn get_user_by_username(bot: &Bot, username: String) -> Option<User> {
    // TODO: Get user by username
    None
}

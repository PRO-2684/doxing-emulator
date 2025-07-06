//! Actual implementation of doxing.

use cached::proc_macro::cached;
use frankenstein::{
    AsyncTelegramApi,
    client_reqwest::Bot,
    methods::{GetChatMemberParams, GetChatParams},
    types::{Birthdate, ChatFullInfo, ChatMember, ChatType, User},
};
use log::{debug, error, warn};
use std::fmt::Write;

/// Dox given [`User`] and optional [`ChatFullInfo`].
pub fn dox(doxee: &User, full_info: Option<&ChatFullInfo>) -> String {
    // Generate doxing report
    let mut report = String::new();
    // User ID
    let id = doxee.id;
    _ = write!(report, "您好，请问是用户 ID 为 <code>{id}</code>")
        .inspect_err(|e| error!("Cannot write to report: {e}"));
    // Username
    if let Some(username) = &doxee.username {
        _ = write!(report, "，用户名为 <code>@{username}</code>")
            .inspect_err(|e| error!("Cannot write to report: {e}"));
    }
    // Detailed doxing if full info provided
    if let Some(full_info) = full_info
        && let Some(detail) = detailed_doxing(full_info)
    {
        report.push_str(&detail);
    }
    // Names
    report.push_str(" 的 <code>");
    let first_name = &doxee.first_name;
    report.push_str(&escape(first_name));
    if let Some(last_name) = &doxee.last_name {
        report.push(' ');
        report.push_str(&escape(last_name));
    }
    report.push_str("</code> ");
    // Titles
    if fish_cake(&doxee) {
        report.push_str("南梁");
    } else if doxee.is_premium == Some(true) {
        report.push_str("富哥");
    } else {
        report.push_str("先生");
    }
    report.push_str("吗？");

    report
}

/// Detailed doxing.
fn detailed_doxing(full_info: &ChatFullInfo) -> Option<String> {
    let user_id = full_info.id;
    let mut detail = String::new();
    if !matches!(full_info.type_field, ChatType::Private) {
        warn!("Trying to dox a non-private chat: {user_id}");
        return None;
    }
    // TODO: BusinessLocation
    if let Some(birthday) = &full_info.birthdate {
        let Birthdate { year, month, day } = birthday;
        _ = match year {
            None => write!(detail, "，生日在 {month:02} 月 {day:02} 日"),
            Some(year) => write!(detail, "，生日在 {year:04} 年 {month:02} 月 {day:02} 日"),
        }
        .inspect_err(|e| error!("Cannot write to detail: {e}"));
    }
    if let Some(channel) = &full_info.personal_chat {
        if let Some(channel_username) = &channel.username {
            _ = write!(detail, "，开通了 tg 空间 @{channel_username}")
                .inspect_err(|e| warn!("Cannot write to detail: {e}"));
        } else {
            warn!("Cannot get username of personal channel: {}", channel.id);
        }
    }

    Some(detail)
}

/// Try to get full info about the user.
#[cached(
    size = 64,
    time = 60,
    key = "u64",
    convert = r#"{ user_id }"#,
    sync_writes = "by_key"
)]
pub async fn get_full_info(bot: &Bot, user_id: u64) -> Option<ChatFullInfo> {
    let chat_id = match i64::try_from(user_id) {
        Ok(id) => id,
        Err(e) => {
            warn!("[get_full_info] Cannot convert user_id {user_id} to chat_id: {e:?}");
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

/// Try to get [`User`] and its full info from given id ~~or username~~.
pub async fn get_user_full(bot: &Bot, identifier: &str) -> Option<(User, Option<ChatFullInfo>)> {
    if !identifier.is_ascii() {
        None
    } else if identifier.chars().all(|c| c.is_ascii_digit()) {
        let user_id: u64 = identifier.parse().ok()?;
        let user = get_user_by_id(bot, user_id).await;
        if let Some(user) = user {
            Some((user, get_full_info(bot, user_id).await))
        } else {
            None
        }
    } else {
        // let username = identifier.trim_start_matches('@').to_ascii_lowercase();
        // get_user_full_by_username(bot, username).await
        None
    }
}

/// Try to get [`User`] from given id.
#[cached(
    size = 64,
    time = 60,
    key = "u64",
    convert = r#"{ user_id }"#,
    sync_writes = "by_key"
)]
async fn get_user_by_id(bot: &Bot, user_id: u64) -> Option<User> {
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
    match bot.get_chat_member(&get_params).await {
        Ok(result) => {
            let user = match result.result {
                ChatMember::Administrator(admin) => admin.user,
                ChatMember::Creator(creator) => creator.user,
                ChatMember::Kicked(kicked) => kicked.user,
                ChatMember::Left(left) => left.user,
                ChatMember::Member(member) => member.user,
                ChatMember::Restricted(restricted) => restricted.user,
            };
            Some(user)
        }
        Err(e) => {
            debug!("Cannot get user from id {user_id}: {e:?}");
            None
        }
    }
}

/// **Won't work. Kept for reference only.**
///
/// Try to get [`User`] from given username. Note that the provided username mustn't start with `@` and should be lowercased for best caching.
#[allow(dead_code, reason = "Kept for reference")]
async fn get_user_full_by_username(
    bot: &Bot,
    username: String,
) -> Option<(User, Option<ChatFullInfo>)> {
    let get_params = GetChatParams::builder()
        .chat_id(format!("@{username}"))
        .build();
    let full_info = match bot.get_chat(&get_params).await {
        Err(e) => {
            warn!("Error querying @{username}: {e:?}");
            None
        }
        Ok(r) => Some(r.result),
    }?;
    if !matches!(full_info.type_field, ChatType::Private) {
        warn!("Trying to get user full on a non-private chat: @{username}");
        return None;
    }

    let chat_id = full_info.id;
    let user_id = match u64::try_from(chat_id) {
        Ok(id) => id,
        Err(e) => {
            warn!("[get_user_full_by_username] Cannot convert chat_id {chat_id} to user_id: {e:?}");
            return None;
        }
    };
    let Some(user) = get_user_by_id(bot, user_id).await else {
        error!("Cannot get user by id, even we've got chat full info");
        return None;
    };
    Some((user, Some(full_info)))
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

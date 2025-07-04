//! The dox command.

use super::Command;
use frankenstein::{
    AsyncTelegramApi,
    client_reqwest::Bot,
    methods::GetChatParams,
    types::{Birthdate, ChatFullInfo, ChatType, Message},
};
use log::warn;
use std::fmt::Write;

/// The dox command.
pub struct Dox {
    /// The target of doxing. If empty, should be determined from message. Unused for now.
    pub doxee: Option<String>,
}

impl Command for Dox {
    const TRIGGER: &'static str = "dox";
    const HELP: &'static str = "盒盒盒";
    async fn execute(self, bot: &Bot, msg: Message) -> String {
        // TODO: Reject premium users & users that have not contacted the bot
        // Determine doxee
        let doxee = match self.doxee {
            // Target not provided in command
            None => match msg.reply_to_message {
                // Not a reply message
                None => match msg.from {
                    // Can't determine sender
                    None => return include_str!("../messages/invoke-no-sender.html").to_string(),
                    // Fallback to sender
                    Some(sender) => sender,
                },
                // Reply message
                Some(reply) => match reply.from {
                    None => return include_str!("../messages/reply-no-sender.html").to_string(),
                    Some(sender) => sender,
                },
            },
            // Target provided in command
            Some(_doxee) => {
                // TODO: Resolve provided doxee
                return "TBD".to_string();
            }
        };

        // Generate doxing report
        let mut report = String::new();
        // User ID
        let id = doxee.id;
        // report.push_str(&format!("用户 ID 是 {id}"));
        if let Err(e) = write!(report, "用户 ID 是 {id}") {
            warn!("Cannot write to report: {e}");
        }
        // Username
        if let Some(username) = doxee.username {
            report.push_str("，用户名是 ");
            report.push_str(&username);
        }
        // Detailed doxing
        if let Some(detail) = detailed_doxing(bot, id).await {
            report.push_str(&detail);
        }
        // Names & finish report
        report.push_str(" 的 ");
        let first_name = &doxee.first_name;
        report.push_str(&escape(first_name));
        if let Some(last_name) = &doxee.last_name {
            report.push(' ');
            report.push_str(&escape(last_name));
        }
        if doxee.is_premium == Some(true) {
            report.push_str(" 富哥");
        } else {
            report.push_str(" 先生");
        }

        report
    }
}

/// Escapes the given string, as mentioned by [the docs](https://core.telegram.org/bots/api#html-style) on Telegram.
fn escape(s: &str) -> String {
    s.replace('&', "&amp;")
        .replace('<', "&lt;")
        .replace('>', "&gt;")
    // TODO: More effiency by iterating over chars, estimating resulting size and creating new string
}

/// Detailed doxing, only available if the user has contacted the bot.
async fn detailed_doxing(bot: &Bot, user_id: u64) -> Option<String> {
    let info = get_info(bot, user_id).await?;

    let mut detail = String::new();
    if !matches!(info.type_field, ChatType::Private) {
        warn!("Trying to dox a non-private chat: {user_id}");
        return None;
    }
    if let Some(birthday) = info.birthdate {
        let Birthdate {
            year, // Optional???
            month,
            day,
        } = birthday;
        if let Err(e) = write!(detail, "，生日是 {year:04}/{month:02}/{day:02}") {
            warn!("Cannot write to detail: {e}");
        }
    }
    if let Some(channel) = info.personal_chat {
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

/// Try to get info about the user, only available if the user has contacted the bot.
async fn get_info(bot: &Bot, user_id: u64) -> Option<ChatFullInfo> {
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

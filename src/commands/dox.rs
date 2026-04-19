//! The dox command.

use super::{
    Command,
    dox_impl::{DoxReport, get_full_info, get_user_report, get_user_title_by_id},
};
use frakti::{
    client_cyper::Bot,
    types::{Message, MessageOrigin},
};

/// The dox command.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Dox {
    pub doxee: Option<String>,
}

impl Command for Dox {
    const TRIGGER: &'static str = "dox";
    const HELP: &'static str = "盒盒盒";
    async fn execute(self, bot: &Bot, msg: Message, _username: &str) -> String {
        // Reject users that the bot doesn't know
        let Some(doxer) = msg.from else {
            // Can't determine doxer
            return include_str!("../messages/doxer-identification-failed.html").to_string();
        };
        let Some(doxer_info) = get_full_info(bot, doxer.id).await else {
            // Can't determine doxer's full info
            return include_str!("../messages/doxer-identification-failed.html").to_string();
        };
        // Create a report for the doxee
        let report = match self.doxee {
            // Target not provided in command
            None => match msg.reply_to_message {
                // Not a reply message - try external reply
                None => match msg.external_reply {
                    // Not an external reply message - fallback to doxer
                    None => {
                        let title = get_user_title_by_id(bot, doxer.id, Some(msg.chat.id))
                            .await
                            .map(|(_, title)| title)
                            .flatten();
                        DoxReport::new(*doxer, title, Some(doxer_info))
                    }
                    // External reply message
                    Some(external) => match external.origin {
                        MessageOrigin::User(user) => {
                            let chat_id = external.chat.map(|chat| chat.id);
                            let title = get_user_title_by_id(bot, user.sender_user.id, chat_id)
                                .await
                                .map(|(_, title)| title)
                                .flatten();
                            let full_info = get_full_info(bot, user.sender_user.id).await;
                            DoxReport::new(user.sender_user, title, full_info)
                        }
                        _ => {
                            return include_str!("../messages/invalid-origin.html").to_string();
                        }
                    },
                },
                // Reply message
                Some(reply) => {
                    let Some(sender) = reply.from else {
                        return include_str!("../messages/doxee-identification-failed.html")
                            .to_string();
                    };
                    let chat_id = reply.chat.id;
                    let title = get_user_title_by_id(bot, sender.id, Some(chat_id))
                        .await
                        .map(|(_, title)| title)
                        .flatten();
                    let full_info = get_full_info(bot, sender.id).await;
                    DoxReport::new(*sender, title, full_info)
                }
            },
            // Target provided in command
            Some(doxee) => {
                if let Ok(user_id) = doxee.parse() {
                    // Can be parsed as user_id
                    match get_user_report(bot, user_id, Some(msg.chat.id)).await {
                        Some(report) => report,
                        None => {
                            return include_str!("../messages/doxee-identification-failed.html")
                                .to_string();
                        }
                    }
                } else {
                    // Not user id
                    return include_str!("../messages/not-user-id.html").to_string();
                }
            }
        };

        report.to_string()
    }
}

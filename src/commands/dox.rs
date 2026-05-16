//! The dox command.

use super::{Command, dox_impl::DoxReport};
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
    #[allow(clippy::similar_names)]
    async fn execute(self, bot: &Bot, msg: Message, _username: &str) -> String {
        // Reject users that the bot doesn't know
        let mut doxer_report = if let Some(chat) = msg.sender_chat {
            // Chat (e.g. channel or group)
            DoxReport::from_chat(*chat)
        } else if let Some(user) = msg.from {
            // Regular user
            DoxReport::from_user(*user)
        } else {
            // Can't determine doxer
            return include_str!("../messages/doxer-identification-failed.html").to_string();
        };
        doxer_report = doxer_report.with_title(msg.sender_tag.or(msg.author_signature));
        // Create a report for the doxee
        let report = match self.doxee {
            // Target not provided in command
            None => {
                match msg.reply_to_message {
                    // Not a reply message - try external reply
                    None => match msg.external_reply {
                        // Not an external reply message - fallback to doxer
                        None => doxer_report.complete_full_info(bot).await,
                        // External reply message
                        Some(external) => match external.origin {
                            MessageOrigin::User(user) => {
                                let chat_id = external.chat.map(|chat| chat.id);
                                let doxee_report = DoxReport::from_user(user.sender_user);
                                doxee_report
                                    .complete_title(bot, chat_id)
                                    .await
                                    .complete_full_info(bot)
                                    .await
                            }
                            MessageOrigin::Chat(chat) => DoxReport::from_chat(chat.sender_chat)
                                .with_title(chat.author_signature),
                            MessageOrigin::Channel(channel) => DoxReport::from_chat(channel.chat)
                                .with_title(channel.author_signature),
                            MessageOrigin::HiddenUser(_) => {
                                return include_str!("../messages/invalid-origin.html").to_string();
                            }
                        },
                    },
                    // Reply message
                    Some(reply) => {
                        // TODO: Check `forward_origin` and `external_reply`
                        let doxee_report = if let Some(chat) = reply.sender_chat {
                            // Chat (e.g. channel or group)
                            DoxReport::from_chat(*chat)
                        } else if let Some(user) = reply.from {
                            // Regular user
                            DoxReport::from_user(*user)
                        } else {
                            // Can't determine doxee
                            return include_str!("../messages/doxee-identification-failed.html")
                                .to_string();
                        };
                        doxee_report
                            .with_title(reply.sender_tag.or(reply.author_signature))
                            .complete_full_info(bot)
                            .await
                    }
                }
            }
            // Target provided in command
            Some(doxee) => {
                if let Ok(user_id) = doxee.parse() {
                    // Can be parsed as user_id
                    match DoxReport::from_id(bot, user_id, Some(msg.chat.id)).await {
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

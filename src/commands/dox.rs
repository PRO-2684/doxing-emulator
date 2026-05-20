//! The dox command.

use super::{Command, dox_impl::DoxReport};
use frakti::{client_cyper::Bot, types::Message};

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
        let Some(doxer_report) = DoxReport::from_sender(
            msg.from,
            msg.sender_chat,
            msg.sender_tag.or(msg.author_signature),
        ) else {
            // Can't determine doxer
            return include_str!("../messages/doxer-identification-failed.html").to_string();
        };
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
                        Some(external) => {
                            let chat_id = external.chat.map(|chat| chat.id);
                            let Some(report) =
                                DoxReport::from_origin(bot, external.origin, chat_id).await
                            else {
                                return include_str!("../messages/invalid-origin.html").to_string();
                            };
                            report
                        }
                    },
                    // Reply message
                    Some(reply) => {
                        // TODO: Check `forward_origin` and `external_reply`
                        let Some(doxee_report) = DoxReport::from_sender(
                            reply.from,
                            reply.sender_chat,
                            reply.sender_tag.or(reply.author_signature),
                        ) else {
                            // Can't determine doxee
                            return include_str!("../messages/doxee-identification-failed.html")
                                .to_string();
                        };
                        doxee_report.complete_full_info(bot).await
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

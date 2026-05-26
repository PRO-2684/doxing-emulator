//! The dox command.

use super::{Command, dox_impl::DoxReport};
use crate::messages::BotError;
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
            return BotError::DoxerIdentificationFailed.to_string();
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
                            let Some(report) =
                                Box::pin(DoxReport::from_external_reply(bot, *external)).await
                            else {
                                return BotError::InvalidOrigin.to_string();
                            };
                            report
                        }
                    },
                    // Reply message
                    Some(reply) => {
                        if let Some(forward_origin) = reply.forward_origin {
                            // Replied message is forwarded
                            match DoxReport::from_origin(bot, *forward_origin, Some(reply.chat.id))
                                .await
                            {
                                Some(report) => report,
                                None => {
                                    return BotError::InvalidOrigin.to_string();
                                }
                            }
                        } else if let Some(external) = reply.external_reply {
                            // Replied message is an external reply
                            match Box::pin(DoxReport::from_external_reply(bot, *external)).await {
                                Some(report) => report,
                                None => {
                                    return BotError::InvalidOrigin.to_string();
                                }
                            }
                        } else if let Some(doxee_report) = DoxReport::from_sender(
                            // Use sender of the replied message
                            reply.from,
                            reply.sender_chat,
                            reply.sender_tag.or(reply.author_signature),
                        ) {
                            doxee_report.complete_full_info(bot).await
                        } else {
                            // Can't determine doxee
                            return BotError::DoxeeIdentificationFailed.to_string();
                        }
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
                            return BotError::DoxeeIdentificationFailed.to_string();
                        }
                    }
                } else {
                    // Not user id
                    return BotError::NotUserId.to_string();
                }
            }
        };

        report.to_string()
    }
}

//! The dox command.

use super::{
    Command,
    dox_impl::{dox, get_full_info, get_user_full},
};
use frankenstein::{client_reqwest::Bot, types::Message};

/// The dox command.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Dox {
    pub doxee: Option<String>,
}

impl Command for Dox {
    const TRIGGER: &'static str = "dox";
    const HELP: &'static str = "盒盒盒";
    async fn execute(self, bot: &Bot, msg: Message) -> String {
        // TODO: Reject premium users & users that have not contacted the bot
        // Determine doxee
        let (doxee, full_info) = match self.doxee {
            // Target not provided in command
            None => {
                let user = match msg.reply_to_message {
                    // Not a reply message
                    None => match msg.from {
                        // Can't determine sender
                        None => {
                            return include_str!("../messages/invoke-no-sender.html").to_string();
                        }
                        // Fallback to sender
                        Some(sender) => *sender,
                    },
                    // Reply message
                    Some(reply) => match reply.from {
                        None => {
                            return include_str!("../messages/reply-no-sender.html").to_string();
                        }
                        Some(sender) => *sender,
                    },
                };
                let full_info = get_full_info(bot, user.id).await;
                (user, full_info)
            }
            // Target provided in command
            Some(doxee) => {
                // TODO: Resolve provided doxee
                match get_user_full(bot, doxee).await {
                    Some(user_and_info) => user_and_info,
                    None => {
                        return include_str!("../messages/user-not-found.html").to_string();
                    }
                }
            }
        };

        // let full_info = get_info(bot, doxee.id).await;
        dox(doxee, full_info)
    }
}

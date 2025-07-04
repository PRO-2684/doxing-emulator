//! The dox command.

use super::{Command, dox_impl::{dox, get_info}};
use frankenstein::{
    client_reqwest::Bot,
    types::{Message, User},
};

/// The dox command.
pub struct Dox;

impl Command for Dox {
    const TRIGGER: &'static str = "dox";
    const HELP: &'static str = "盒盒盒";
    async fn execute(self, bot: &Bot, msg: Message) -> String {
        // TODO: Reject premium users & users that have not contacted the bot
        // Determine doxee
        let doxee: Option<User> = None;
        let doxee = match doxee {
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

        let full_info = get_info(bot, doxee.id).await;
        dox(*doxee, full_info)
    }
}

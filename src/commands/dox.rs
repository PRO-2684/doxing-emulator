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
        // Reject users that the bot doesn't know
        let doxer = match msg.from {
            // Can't determine doxer
            None => {
                return include_str!("../messages/doxer-identification-failed.html")
                    .to_string();
            }
            Some(doxer) => *doxer,
        };
        let doxer_info = match get_full_info(bot, doxer.id).await {
            // Can't determine doxer's full info
            None => {
                return include_str!("../messages/doxer-identification-failed.html")
                    .to_string();
            }
            Some(full_info) => full_info,
        };
        // Determine doxee and full info
        let (doxee, doxee_info) = match self.doxee {
            // Target not provided in command
            None => match msg.reply_to_message {
                // Not a reply message - fallback to doxer
                None => (doxer, Some(doxer_info)),
                // Reply message
                Some(reply) => match reply.from {
                    None => {
                        return include_str!("../messages/doxee-identification-failed.html")
                            .to_string();
                    }
                    Some(sender) => {
                        let full_info = get_full_info(bot, sender.id).await;
                        (*sender, full_info)
                    },
                },
            },
            // Target provided in command
            Some(doxee) => match get_user_full(bot, &doxee).await {
                Some(user_and_info) => user_and_info,
                None => {
                    return include_str!("../messages/doxee-identification-failed.html").to_string();
                }
            },
        };

        dox(&doxee, doxee_info.as_ref())
    }
}

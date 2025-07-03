//! The dox command.

use super::Command;
use frankenstein::{client_reqwest::Bot, types::Message};

/// The dox command.
pub struct Dox {}

impl Command for Dox {
    const TRIGGER: &'static str = "dox";
    const HELP: &'static str = "盒盒盒";
    async fn execute(self, bot: &Bot, msg: Message) -> String {
        "".to_string()
    }
}

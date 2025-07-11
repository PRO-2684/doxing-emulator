//! The help command.

use super::Command;
use frankenstein::{client_reqwest::Bot, types::Message};

/// The help command.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Help;

impl Command for Help {
    const TRIGGER: &'static str = "help";
    const HELP: &'static str = "查看帮助信息";
    async fn execute(self, _bot: &Bot, _msg: Message) -> String {
        include_str!("../messages/help.html").to_string()
    }
}

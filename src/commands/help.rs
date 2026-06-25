//! The help command.

use super::Command;
use frakti::{client_cyper::Bot, types::Message};

/// The help command.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Help;

impl Command for Help {
    const TRIGGER: &'static str = "help";
    const HELP: &'static str = "查看帮助信息";
    fn execute(self, _bot: &Bot, _msg: Message, username: &str) -> impl Future<Output = String> {
        std::future::ready(format!(
            include_str!("../messages/help.html"),
            username = username
        ))
    }
}

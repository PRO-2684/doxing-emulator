//! Commands for the bot.

mod dox;
mod help;

pub use dox::Dox;
use frankenstein::{client_reqwest::Bot, types::Message};
pub use help::Help;

/// A command.
pub trait Command {
    /// Trigger word.
    const TRIGGER: &'static str;
    /// Help message.
    const HELP: &'static str;
    /// Execute the command.
    async fn execute(bot: &Bot, msg: Message) -> String;
}

/// A list of available commands and descriptions.
pub const LIST: [(&str, &str); 2] = [(Dox::TRIGGER, Dox::HELP), (Help::TRIGGER, Help::HELP)];

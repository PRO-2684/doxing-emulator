//! Extracts commands from messages and execution.

mod dox;
mod help;

use super::dox_impl;
pub use dox::Dox;
use frankenstein::{client_reqwest::Bot, types::Message};
pub use help::Help;
use log::debug;

/// A command.
pub trait Command {
    /// Trigger word.
    const TRIGGER: &'static str;
    /// Help message.
    const HELP: &'static str;
    /// Execute the command.
    fn execute(self, bot: &Bot, msg: Message) -> impl Future<Output = String> + Send;
}

/// Available commands.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Commands {
    /// The help command.
    Help(Help),
    /// The dox command.
    Dox(Dox),
}

impl Commands {
    /// Try to parse the given text to a command.
    ///
    /// # Arguments
    ///
    /// - `text` - The text to check.
    /// - `username` - The username of the bot.
    #[must_use]
    pub fn parse(text: Option<&String>, username: &str) -> Option<Self> {
        let text = text?.trim();
        let (command, arg) = text.split_once(' ').unwrap_or((text, ""));

        // Two possible command formats:
        // 1. /command <arg>
        // 2. /command@bot_username <arg>

        // Trim the leading slash
        let slash = command.starts_with('/');
        if !slash {
            return None;
        }
        let command = &command[1..];

        // Split out the mention and check if it's the bot
        let (command, mention) = command.split_once('@').unwrap_or((command, ""));
        if !mention.is_empty() && mention != username {
            return None;
        }

        // Match the command
        match command {
            Dox::TRIGGER => {
                let arg = arg.trim();
                let doxee = if arg.is_empty() {
                    None
                } else {
                    Some(arg.to_string())
                };
                Some(Self::Dox(Dox { doxee }))
            }
            Help::TRIGGER | "start" => Some(Self::Help(Help)),
            _ => None,
        }
    }

    /// Execute the command.
    pub async fn execute(self, bot: &Bot, msg: Message) -> String {
        debug!("Executing command: {self:?}");
        match self {
            Self::Help(help) => help.execute(bot, msg).await,
            Self::Dox(dox) => dox.execute(bot, msg).await,
        }
    }
}

/// A list of available commands and descriptions.
pub const LIST: [(&str, &str); 2] = [(Dox::TRIGGER, Dox::HELP), (Help::TRIGGER, Help::HELP)];

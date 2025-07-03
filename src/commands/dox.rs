//! The dox command.

use super::Command;
use frankenstein::{client_reqwest::Bot, types::Message};

/// The dox command.
pub struct Dox {
    /// The target of doxing. If empty, should be determined from message. Unused for now.
    pub doxee: Option<String>,
}

impl Command for Dox {
    const TRIGGER: &'static str = "dox";
    const HELP: &'static str = "盒盒盒";
    async fn execute(self, _bot: &Bot, msg: Message) -> String {
        // TODO: Reject premium users
        // Determine doxee
        let doxee = match self.doxee {
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
            },
        };

        // Generate doxing report
        let mut report = String::new();
        // TODO: Data center (getUserProfilePhotos -> file_id -> getFile)
        // TODO: Personal channel
        // TODO: Phone No
        // TODO: Birthday
        // Use getChatMember for more detail?
        // User ID
        let id = doxee.id;
        report.push_str(&format!("用户 ID 是 {id}"));
        // Username
        if let Some(username) = doxee.username {
            report.push_str("，用户名是 ");
            report.push_str(&username);
        };
        // Names & finish report
        report.push_str(" 的 ");
        let first_name = &doxee.first_name;
        report.push_str(first_name);
        if let Some(last_name) = &doxee.last_name {
            report.push(' ');
            report.push_str(last_name);
        };
        report.push_str(" 先生/女士");

        report
    }
}

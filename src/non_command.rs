//! Module for handling non-command messages.

use super::dox_impl::{dox, get_full_info};
use frankenstein::{
    client_reqwest::Bot,
    types::{ChatType, Message, MessageOrigin},
};
use log::debug;

/// Handles non-command messages.
pub async fn handle_non_command(bot: &Bot, msg: Message) -> Option<String> {
    // We only handle those in private chats, to prevent polluting the groups
    if matches!(msg.chat.type_field, ChatType::Private) {
        debug!("Handling non-command message in PM: {msg:?}");
        let reply = if let Some(origin) = msg.forward_origin {
            // The message is forwarded
            // Reject users that the bot doesn't know
            let doxer = match &msg.from {
                // Can't determine doxer
                None => {
                    return Some(
                        include_str!("./messages/doxer-identification-failed.html").to_string(),
                    );
                }
                Some(doxer) => doxer,
            };
            let _doxer_info = match get_full_info(bot, doxer.id).await {
                // Can't determine doxer's full info
                None => {
                    return Some(
                        include_str!("./messages/doxer-identification-failed.html").to_string(),
                    );
                }
                Some(full_info) => full_info,
            };
            if let MessageOrigin::User(origin_user) = *origin {
                // ... from a user
                let doxee = origin_user.sender_user;
                let full_info = get_full_info(bot, doxee.id).await;
                dox(&doxee, full_info.as_ref())
            } else {
                // ... from something else
                debug!("Cannot determine the origin as a user: {origin:?}");
                include_str!("messages/invalid-origin.html").to_string()
            }
        } else {
            // Not forwarded message - incomprehensible
            debug!(
                "Not a command or forwarded message: {:?}",
                msg.text.as_ref()
            );
            include_str!("messages/incomprehensible.html").to_string()
        };
        Some(reply)
    } else {
        None
    }
}

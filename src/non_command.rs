//! Module for handling non-command messages.

use super::dox_impl::DoxReport;
use frakti::{
    client_cyper::Bot,
    types::{ChatType, Message, MessageOrigin},
};
use log::{debug, info};

/// Handles non-command messages.
pub async fn handle_non_command(bot: &Bot, msg: Message) -> Option<String> {
    // We only handle those in private chats, to prevent polluting the groups
    if matches!(msg.chat.type_field, ChatType::Private) {
        info!("Handling non-command message in PM: {msg:?}");
        let reply = if let Some(origin) = msg.forward_origin {
            // The message is forwarded
            if msg.from.is_none() && msg.sender_chat.is_none() {
                // Can't determine doxer
                return Some(
                    include_str!("./messages/doxer-identification-failed.html").to_string(),
                );
            }
            match *origin {
                MessageOrigin::User(origin_user) => {
                    // ... from a user
                    DoxReport::from_user(origin_user.sender_user)
                        .complete_full_info(bot)
                        .await
                        .to_string()
                }
                MessageOrigin::Channel(origin_channel) => {
                    // ... from a channel
                    DoxReport::from_chat(origin_channel.chat)
                        .with_title(origin_channel.author_signature)
                        .to_string()
                }
                MessageOrigin::Chat(origin_chat) => {
                    // ... from a chat
                    DoxReport::from_chat(origin_chat.sender_chat)
                        .with_title(origin_chat.author_signature)
                        .to_string()
                }
                MessageOrigin::HiddenUser(_) => {
                    // ... from something else
                    debug!("Cannot determine the origin as a user: {origin:?}");
                    include_str!("messages/invalid-origin.html").to_string()
                }
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

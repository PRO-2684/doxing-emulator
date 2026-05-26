//! Module for handling non-command messages.

use super::{dox_impl::DoxReport, messages::BotError};
use frakti::{
    client_cyper::Bot,
    types::{ChatType, Message},
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
                return Some(BotError::DoxerIdentificationFailed.to_string());
            }
            debug!("Handling forwarded origin: {origin:?}");
            DoxReport::from_origin(bot, *origin, None)
                .await
                .map_or_else(
                    || {
                        debug!("Cannot determine the origin as a user");
                        BotError::InvalidOrigin.to_string()
                    },
                    |report| report.to_string(),
                )
        } else {
            // Not forwarded message - incomprehensible
            debug!(
                "Not a command or forwarded message: {:?}",
                msg.text.as_ref()
            );
            BotError::Incomprehensible.to_string()
        };
        Some(reply)
    } else {
        None
    }
}

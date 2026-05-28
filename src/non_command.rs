//! Module for handling non-command messages.

use super::{
    doxee_resolution::{DoxeeSource, resolve},
    messages::BotError,
};
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
        if msg.forward_origin.is_none() {
            debug!(
                "Not a command or forwarded message: {:?}",
                msg.text.as_ref()
            );
        }
        let source = DoxeeSource::PrivateForward { message: msg };
        let result = Box::pin(resolve(bot, source)).await?;
        // .expect("private forward resolution should always reply");
        Some(match result {
            Ok(report) => report.to_string(),
            Err(BotError::InvalidOrigin) => {
                debug!("Cannot determine the origin as a user");
                BotError::InvalidOrigin.to_string()
            }
            Err(error) => error.to_string(),
        })
    } else {
        None
    }
}

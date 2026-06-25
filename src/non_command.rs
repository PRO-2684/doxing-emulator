//! Module for handling non-command messages.

use super::{doxee_resolution::DoxeeSource, messages::BotError};
use frakti::{
    client_cyper::Bot,
    types::{ChatType, Message},
};
use futures_util::{FutureExt, future::Either};
use log::{debug, info};

/// Handles non-command messages.
pub fn handle_non_command(bot: &Bot, msg: Message) -> impl Future<Output = Option<String>> {
    // We only handle those in private chats, to prevent polluting the groups
    if matches!(msg.chat.type_field, ChatType::Private) {
        info!("Handling non-command message in PM: {msg:?}");
        if msg.forward_origin.is_none() {
            debug!(
                "Not a command or forwarded message: {:?}",
                msg.text.as_ref()
            );
        }
        let source = DoxeeSource::PrivateMessage { message: msg };
        Either::Left(source.resolve_with(bot).map(|result| {
            result.map(|result| match result {
                Ok(report) => report.to_string(),
                Err(BotError::InvalidOrigin) => {
                    debug!("Cannot determine the origin as a user");
                    BotError::InvalidOrigin.to_string()
                }
                Err(error) => error.to_string(),
            })
        }))
    } else {
        Either::Right(std::future::ready(None))
    }
}

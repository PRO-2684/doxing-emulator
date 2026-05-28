//! Module for handling guest messages.

use super::{
    doxee_resolution::{DoxeeSource, parse_guest_invocation, resolve},
    inline::create_article,
};
use frakti::{client_cyper::Bot, inline_mode::InlineQueryResultArticle, types::Message};
use log::info;

/// Handle guest messages.
pub async fn handle_guest_message(
    bot: &Bot,
    msg: Message,
    bot_username: &str,
) -> Option<InlineQueryResultArticle> {
    info!("Handling guest message: {:?}", msg.text);

    let arg = parse_guest_invocation(msg.text.as_deref(), bot_username)?;
    let source = DoxeeSource::Guest { arg, message: msg };
    let result = Box::pin(resolve(bot, source))
        .await
        .expect("guest mention resolution should always reply");
    let message = match result {
        Ok(report) => report.to_string(),
        Err(error) => error.to_string(),
    };

    Some(create_article(message, "Title", "Description"))
}

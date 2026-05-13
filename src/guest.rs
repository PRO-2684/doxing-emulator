//! Module for handling guest messages.

use frakti::{ParseMode, client_cyper::Bot, inline_mode::{InlineQueryResultArticle, InputMessageContent, InputTextMessageContent}, types::Message};
use super::inline::create_article;
use log::{error, info};

/// Handle guest messages.
pub async fn handle_guest_message(bot: &Bot, msg: Message) -> InlineQueryResultArticle {
    info!("Received guest message: {:?}", msg.text);

    // Dummy impl
    create_article("message", "title", "description")
}

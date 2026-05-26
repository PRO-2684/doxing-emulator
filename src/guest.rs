//! Module for handling guest messages.

use crate::Command;

use super::{commands::Dox, inline::create_article};
use frakti::{client_cyper::Bot, inline_mode::InlineQueryResultArticle, types::Message};
use log::info;

/// Handle guest messages.
pub async fn handle_guest_message(bot: &Bot, msg: Message) -> InlineQueryResultArticle {
    info!("Handling guest message: {:?}", msg.text);

    // Delegate to `dox` command
    let dox = Dox { doxee: None };
    let report = Box::pin(dox.execute(bot, msg, "")).await;

    create_article(report, "Title", "Description")
}

//! Module for handling inline queries.

use super::{
    doxee_resolution::{DoxArg, DoxeeSource},
    messages::BotError,
};
use frakti::{
    ParseMode,
    client_cyper::Bot,
    inline_mode::{
        InlineQuery, InlineQueryResultArticle, InlineQueryResultsButton, InputMessageContent,
        InputTextMessageContent,
    },
};
use futures_util::FutureExt;
use log::info;

/// Handle inline queries.
pub fn handle_inline_query(
    bot: &Bot,
    inline: InlineQuery,
) -> impl Future<Output = InlineQueryResultArticle> {
    info!("Handling inline query: {}", inline.query);
    let arg = DoxArg::parse(Some(&inline.query));
    let source = DoxeeSource::Inline {
        arg,
        from: inline.from,
    };
    source.resolve_with(bot).map(|result| {
        match result.expect("inline query resolution should always reply") {
            Ok(report) => create_article(report.to_string(), report.inline_title(), "盒盒盒"),
            Err(BotError::DoxeeIdentificationFailed) => create_article(
                BotError::DoxeeIdentificationFailed,
                "ERR_DOXEE_IDENTIFICATION_FAILED",
                "马冬什么？马冬梅。什么冬梅啊？马冬梅啊。马什么梅啊？行，大爷，您先凉快吧。",
            ),
            Err(BotError::NotUserId) => {
                create_article(BotError::NotUserId, "ERR_NOT_USER_ID", "发的啥呀这是？")
            }
            Err(error) => create_article(error, "ERR", "发的啥呀这是？"), // Should not happen
        }
    })
}

/// Create an article with given message, title and description.
pub fn create_article(
    message: impl Into<String>,
    title: impl Into<String>,
    description: impl Into<String>,
) -> InlineQueryResultArticle {
    let content = InputTextMessageContent::builder()
        .message_text(message)
        .parse_mode(ParseMode::Html)
        .build();
    InlineQueryResultArticle::builder()
        .id("1")
        .title(title)
        .description(description)
        .input_message_content(InputMessageContent::Text(content))
        .build()
}

/// Create a button that sends `/start help` to the bot.
pub fn help_button() -> InlineQueryResultsButton {
    InlineQueryResultsButton::builder()
        .text("遇到 ERR? 点我查看帮助!")
        .start_parameter("help")
        .build()
}

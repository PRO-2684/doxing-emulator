//! Module for handling inline queries.

use super::dox_impl::{dox, get_user_full};
use frankenstein::{
    ParseMode,
    client_reqwest::Bot,
    inline_mode::{
        InlineQuery, InlineQueryResult, InlineQueryResultArticle, InlineQueryResultsButton,
        InputMessageContent, InputTextMessageContent,
    },
};

/// Handle inline queries.
pub async fn handle_inline_query(bot: &Bot, inline: &InlineQuery) -> InlineQueryResult {
    let doxee = inline.query.trim();
    let article = if let Some((user, full_info)) = get_user_full(bot, doxee).await {
        let report = dox(&user, full_info.as_ref());
        create_article(report, format!("开盒 {}", user.first_name), "盒盒盒")
    } else {
        create_article(
            include_str!("./messages/user-identification-failed.html"),
            "ERR_USER_IDENTIFICATION_FAILED",
            "马冬什么？马冬梅。什么冬梅啊？马冬梅啊。马什么梅啊？行，大爷，您先凉快吧。",
        )
    };
    InlineQueryResult::Article(article)
}

/// Create an article with given message, title and description.
fn create_article(
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

/// Create a button that sends /help to the bot.
pub fn help_button() -> InlineQueryResultsButton {
    InlineQueryResultsButton::builder()
        .text("查看帮助")
        .start_parameter("help")
        .build()
}

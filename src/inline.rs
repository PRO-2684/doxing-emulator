//! Module for handling inline queries.

use super::dox_impl::{dox, get_user_full, get_full_info};
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
    // Reject users that the bot doesn't know
    let doxer = &inline.from;
    let Some(doxer_info) = get_full_info(bot, doxer.id).await else {
        return create_article(include_str!("./messages/doxer-identification-failed.html"), "ERR_DOXER_IDENTIFICATION_FAILED", "谁在说话？滚木吗？");
    };
    // Actual doxing
    let query = inline.query.trim();
    if query.is_empty() {
        let report = dox(&doxer, Some(&doxer_info));
        create_article(report, format!("开盒 {}", doxer.first_name), "盒盒盒")
    } else if let Some((doxee, doxee_info)) = get_user_full(bot, query).await {
        let report = dox(&doxee, doxee_info.as_ref());
        create_article(report, format!("开盒 {}", doxee.first_name), "盒盒盒")
    } else {
        create_article(
            include_str!("./messages/doxee-identification-failed.html"),
            "ERR_DOXEE_IDENTIFICATION_FAILED",
            "马冬什么？马冬梅。什么冬梅啊？马冬梅啊。马什么梅啊？行，大爷，您先凉快吧。",
        )
    }
}

/// Create an article with given message, title and description.
fn create_article(
    message: impl Into<String>,
    title: impl Into<String>,
    description: impl Into<String>,
) -> InlineQueryResult {
    let content = InputTextMessageContent::builder()
        .message_text(message)
        .parse_mode(ParseMode::Html)
        .build();
    let article = InlineQueryResultArticle::builder()
        .id("1")
        .title(title)
        .description(description)
        .input_message_content(InputMessageContent::Text(content))
        .build();
    InlineQueryResult::Article(article)
}

/// Create a button that sends /help to the bot.
pub fn help_button() -> InlineQueryResultsButton {
    InlineQueryResultsButton::builder()
        .text("遇到 ERR? 点我查看帮助!")
        .start_parameter("help")
        .build()
}

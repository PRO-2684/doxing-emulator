//! Module for handling inline queries.

use super::dox_impl::{DoxReport, get_full_info, get_user_report};
use frakti::{
    ParseMode,
    client_cyper::Bot,
    inline_mode::{
        InlineQuery, InlineQueryResultArticle, InlineQueryResultsButton, InputMessageContent,
        InputTextMessageContent,
    },
};
use log::info;

/// Handle inline queries.
pub async fn handle_inline_query(bot: &Bot, inline: &InlineQuery) -> InlineQueryResultArticle {
    info!("Handling inline query: {}", inline.query);
    // Reject users that the bot doesn't know
    let doxer = &inline.from;
    let Some(doxer_info) = get_full_info(bot, doxer.id).await else {
        return create_article(
            include_str!("./messages/doxer-identification-failed.html"),
            "ERR_DOXER_IDENTIFICATION_FAILED",
            "谁在说话？滚木吗？",
        );
    };
    // Actual doxing
    let query = inline.query.trim();
    if query.is_empty() {
        let report = DoxReport::new(doxer.clone(), None, Some(doxer_info));
        create_article(
            report.to_string(),
            format!("开盒 {}", doxer.first_name),
            "盒盒盒",
        )
    } else if let Ok(user_id) = query.parse() {
        // Can be parsed as user_id
        match get_user_report(bot, user_id, None).await {
            Some(report) => create_article(
                report.to_string(),
                format!("开盒 {}", report.user.first_name),
                "盒盒盒",
            ),
            None => create_article(
                include_str!("./messages/doxee-identification-failed.html"),
                "ERR_DOXEE_IDENTIFICATION_FAILED",
                "马冬什么？马冬梅。什么冬梅啊？马冬梅啊。马什么梅啊？行，大爷，您先凉快吧。",
            ),
        }
    } else {
        // Not user id
        create_article(
            include_str!("./messages/not-user-id.html"),
            "ERR_NOT_USER_ID",
            "发的啥呀这是？",
        )
    }
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

/// Create a button that sends `/start help` to the bot.
pub fn help_button() -> InlineQueryResultsButton {
    InlineQueryResultsButton::builder()
        .text("遇到 ERR? 点我查看帮助!")
        .start_parameter("help")
        .build()
}

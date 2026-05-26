//! Module for handling inline queries.

use super::{
    dox_impl::{DoxReport, SubjectId},
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
use log::info;

/// Handle inline queries.
pub async fn handle_inline_query(bot: &Bot, inline: InlineQuery) -> InlineQueryResultArticle {
    info!("Handling inline query: {}", inline.query);
    // Reject users that the bot doesn't know
    let mut doxer_report = DoxReport::from_user(inline.from);
    // Actual doxing
    let query = inline.query.trim();
    if query.is_empty() {
        doxer_report = doxer_report.complete_full_info(bot).await;
        create_article(
            doxer_report.to_string(),
            format!("开盒 {}", doxer_report.display_name.unwrap_or_default()),
            "盒盒盒",
        )
    } else if let Ok(user_id) = query.parse() {
        // Can be parsed as user_id
        match DoxReport::from_id(bot, user_id, None).await {
            Some(report) => create_article(
                report.to_string(),
                format!(
                    "开盒 {}",
                    report
                        .display_name
                        .or(report.last_name)
                        .or(report.username)
                        .unwrap_or_else(|| match report.subject {
                            SubjectId::User(id) => id.to_string(),
                            SubjectId::Chat(id) => id.to_string(),
                        })
                ),
                "盒盒盒",
            ),
            None => create_article(
                BotError::DoxeeIdentificationFailed,
                "ERR_DOXEE_IDENTIFICATION_FAILED",
                "马冬什么？马冬梅。什么冬梅啊？马冬梅啊。马什么梅啊？行，大爷，您先凉快吧。",
            ),
        }
    } else {
        // Not user id
        create_article(BotError::NotUserId, "ERR_NOT_USER_ID", "发的啥呀这是？")
    }
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

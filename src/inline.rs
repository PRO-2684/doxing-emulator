//! Module for handling inline queries.

use super::{
    dox_impl::{DoxReport, SubjectId},
    doxee_resolution::{DoxArg, DoxeeSource, resolve},
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
    let arg = DoxArg::parse(Some(&inline.query));
    let source = DoxeeSource::Inline {
        arg,
        from: inline.from,
    };
    let result = Box::pin(resolve(bot, source))
        .await
        .expect("inline query resolution should always reply");
    match result {
        Ok(report) => create_article(report.to_string(), title(&report), "盒盒盒"),
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
}

fn title(report: &DoxReport) -> String {
    format!(
        "开盒 {}",
        report
            .display_name
            .as_deref()
            .or(report.last_name.as_deref())
            .or(report.username.as_deref())
            .map_or_else(
                || match report.subject {
                    SubjectId::User(id) => id.to_string(),
                    SubjectId::Chat(id) => id.to_string(),
                },
                ToOwned::to_owned,
            )
    )
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

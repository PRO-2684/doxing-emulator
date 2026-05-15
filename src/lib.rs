//! # `doxing-emulator` library crate
//!
//! If you are reading this, you are reading the documentation for the `doxing-emulator` library crate. For the cli, kindly refer to the README file.

#![deny(missing_docs)]
#![warn(clippy::all, clippy::nursery, clippy::pedantic, clippy::cargo)]
#![allow(clippy::multiple_crate_versions, reason = "Dependency")]

mod commands;
mod dox_impl;
mod guest;
mod inline;
mod non_command;
mod setup;

use anyhow::{Result, bail};
pub use commands::{Command, Commands};
pub use dox_impl::DoxReport;
use frakti::{
    AsyncTelegramApi, BASE_API_URL, ParseMode,
    client_cyper::Bot,
    cyper::{Client, proxy::Proxy},
    inline_mode::{InlineQuery, InlineQueryResult},
    methods::{
        AnswerGuestQueryParams, AnswerInlineQueryParams, GetUpdatesParams, SendMessageParams,
    },
    types::{Message, ReplyParameters},
    updates::UpdateContent,
};
use log::{error, info, trace};
use non_command::handle_non_command;
use serde::Deserialize;
use setup::{setup_commands, setup_rights};

/// Configuration for the bot.
#[derive(Deserialize, Debug, Clone, PartialEq, Eq)]
pub struct Config {
    /// The token for the bot.
    pub token: String,

    /// Optional proxy URL.
    pub proxy: Option<String>,
}

/// Runs the bot.
///
/// ## Errors
///
/// Errors if setting up or determining username failed.
pub async fn run(config: Config) -> Result<()> {
    let token = config.token;
    let api_url = format!("{BASE_API_URL}{token}");

    let mut builder = Client::builder();
    if let Some(proxy) = config.proxy {
        let proxy = Proxy::all(proxy)?;
        builder = builder.proxy(proxy);
    };
    let client = builder.build();

    let bot = Bot { api_url, client };

    setup_commands(&bot).await?;
    setup_rights(&bot).await?;

    let me = bot.get_me().await?.result;
    let Some(username) = me.username else {
        bail!("Cannot get username of the bot");
    };
    info!("Bot started: @{username}");

    // Handle incoming messages
    let mut update_params = GetUpdatesParams::builder().build();
    loop {
        match bot.get_updates(&update_params).await {
            Ok(updates) => {
                // Update offset
                let Some(last) = updates.result.last() else {
                    continue;
                };
                update_params.offset.replace((last.update_id + 1).into());
                // Process each update
                for update in updates.result {
                    trace!("Received update: {update:?}");
                    match update.content {
                        UpdateContent::Message(msg) => {
                            // Handling messages
                            let bot = bot.clone();
                            let username = username.clone();
                            compio::runtime::spawn(async move {
                                handle_message(bot, *msg, username).await;
                            })
                            .detach();
                        }
                        UpdateContent::GuestMessage(msg) => {
                            // Handling guest messages
                            let bot = bot.clone();
                            compio::runtime::spawn(async move {
                                handle_guest(bot, *msg).await;
                            })
                            .detach();
                        }
                        UpdateContent::InlineQuery(inline) => {
                            // Handling inline queries
                            let bot = bot.clone();
                            compio::runtime::spawn(async move {
                                handle_inline(bot, inline).await;
                            })
                            .detach();
                        }
                        _ => {}
                    }
                }
            }
            Err(err) => {
                error!("Error getting updates: {err}");
            }
        }
    }
}

async fn handle_message(bot: Bot, msg: Message, username: String) {
    let parsed = Commands::parse(msg.text.as_ref(), &username);
    let chat_id = msg.chat.id;
    let message_id = msg.message_id;
    let reply = match parsed {
        // Commands
        Some(command) => Some(command.execute(&bot, msg, &username).await),
        // Non-commands, can be forwarded messages or others
        None => handle_non_command(&bot, msg).await,
    };
    if let Some(reply) = reply {
        info!("Reply: {reply}");
        let reply_param = ReplyParameters::builder().message_id(message_id).build();
        let send_message_param = SendMessageParams::builder()
            .chat_id(chat_id)
            .text(reply)
            .reply_parameters(reply_param)
            .parse_mode(ParseMode::Html)
            .build();
        _ = bot
            .send_message(&send_message_param)
            .await
            .inspect_err(|e| error!("Failed to send message: {e}"));
    }
}

async fn handle_guest(bot: Bot, msg: Message) {
    let Some(guest_id) = msg.guest_query_id.clone() else {
        error!("Guest message without guest_query_id: {msg:?}");
        return;
    };
    let article = guest::handle_guest_message(&bot, msg).await;
    info!("Answer: {:?}", article.input_message_content);
    let result = InlineQueryResult::Article(article);
    let answer_param = AnswerGuestQueryParams::builder()
        .guest_query_id(guest_id)
        .result(result)
        .build();
    _ = bot
        .answer_guest_query(&answer_param)
        .await
        .inspect_err(|e| error!("Failed to answer guest query: {e}"));
}

async fn handle_inline(bot: Bot, inline: InlineQuery) {
    let inline_id = inline.id.clone();
    let article = inline::handle_inline_query(&bot, inline).await;
    info!("Answer: {:?}", article.input_message_content);
    let result = InlineQueryResult::Article(article);
    let answer_param = AnswerInlineQueryParams::builder()
        .inline_query_id(inline_id)
        .results(vec![result])
        .button(inline::help_button())
        .cache_time(60)
        .build();
    _ = bot
        .answer_inline_query(&answer_param)
        .await
        .inspect_err(|e| error!("Failed to answer inline query: {e}"));
}

//! # `doxing-emulator` library crate
//!
//! If you are reading this, you are reading the documentation for the `doxing-emulator` library crate. For the cli, kindly refer to the README file.

#![deny(missing_docs)]
#![warn(clippy::all, clippy::nursery, clippy::pedantic, clippy::cargo)]
#![allow(clippy::multiple_crate_versions, reason = "Dependency")]

mod commands;
mod dox_impl;
mod inline;
mod non_command;
mod setup;

use anyhow::{Result, bail};
pub use commands::{Command, Commands};
use frankenstein::{
    AsyncTelegramApi, ParseMode,
    client_reqwest::Bot,
    methods::{AnswerInlineQueryParams, GetUpdatesParams, SendMessageParams},
    types::ReplyParameters,
    updates::UpdateContent,
};
use log::{error, info, trace};
use non_command::handle_non_command;
use serde::Deserialize;
use setup::{setup_commands, setup_rights};

/// Configuration for the bot.
#[derive(Deserialize, Clone, PartialEq, Eq)]
pub struct Config {
    /// The token for the bot.
    pub token: String,
}

/// Runs the bot.
///
/// ## Errors
///
/// Errors if setting up or determining username failed.
pub async fn run(config: Config) -> Result<()> {
    let token = config.token;
    let bot = Bot::new(&token);
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
                            let parsed = Commands::parse(msg.text.as_ref(), &username);
                            tokio::spawn(async move {
                                let chat_id = msg.chat.id;
                                let message_id = msg.message_id;
                                let reply = match parsed {
                                    // Commands
                                    Some(command) => Some(command.execute(&bot, *msg).await),
                                    // Non-commands, can be forwarded messages or others
                                    None => handle_non_command(&bot, *msg).await,
                                };
                                if let Some(reply) = reply {
                                    let reply_param =
                                        ReplyParameters::builder().message_id(message_id).build();
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
                            });
                        }
                        UpdateContent::InlineQuery(inline) => {
                            // Handling inline queries
                            let bot = bot.clone();
                            tokio::spawn(async move {
                                let result = inline::handle_inline_query(&bot, &inline).await;
                                let answer_param = AnswerInlineQueryParams::builder()
                                    .inline_query_id(inline.id)
                                    .results(vec![result])
                                    .button(inline::help_button())
                                    .cache_time(60)
                                    .build();
                                _ = bot
                                    .answer_inline_query(&answer_param)
                                    .await
                                    .inspect_err(|e| error!("Failed to answer inline query: {e}"));
                            });
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

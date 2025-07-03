//! # `doxing-emulator` library crate
//!
//! If you are reading this, you are reading the documentation for the `doxing-emulator` library crate. For the cli, kindly refer to the README file.

#![deny(missing_docs)]
#![warn(clippy::all, clippy::nursery, clippy::pedantic, clippy::cargo)]

mod commands;
mod setup;

use anyhow::{bail, Result};
use frankenstein::{client_reqwest::Bot, methods::{GetUpdatesParams, SendMessageParams}, types::ReplyParameters, updates::UpdateContent, AsyncTelegramApi};
use serde::Deserialize;
use setup::{setup_commands, setup_rights};
use log::{debug, error, info};
pub use commands::{Command, Commands};

/// Configuration for the bot.
#[derive(Deserialize)]
pub struct Config {
    /// The token for the bot.
    pub token: String,
}

/// Runs the bot.
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
                    debug!("Received update: {update:?}");
                    let UpdateContent::Message(msg) = update.content else {
                        continue;
                    };

                    let text = msg.text.as_ref();
                    let Some(command) = Commands::parse(text, &username) else {
                        debug!("Not a command: {text:?}");
                        continue;
                    };
                    let bot = bot.clone();
                    tokio::spawn(async move {
                        let chat_id = msg.chat.id;
                        let message_id = msg.message_id;
                        let reply = command.execute(&bot, *msg).await;
                        let reply_param = ReplyParameters::builder().message_id(message_id).build();
                        let send_message_param = SendMessageParams::builder()
                            .chat_id(chat_id)
                            .text(reply)
                            .reply_parameters(reply_param)
                            .build();
                        if let Err(err) = bot.send_message(&send_message_param).await {
                            error!("Failed to send message: {err}");
                        }
                    });
                }
            }
            Err(err) => {
                error!("Error getting updates: {err}");
            }
        }
    }
}

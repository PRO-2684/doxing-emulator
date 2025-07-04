//! # `doxing-emulator` library crate
//!
//! If you are reading this, you are reading the documentation for the `doxing-emulator` library crate. For the cli, kindly refer to the README file.

#![deny(missing_docs)]
#![warn(clippy::all, clippy::nursery, clippy::pedantic, clippy::cargo)]
#![allow(clippy::multiple_crate_versions, reason = "Dependency")]

mod commands;
mod setup;
mod dox_impl;

use anyhow::{Result, bail};
pub use commands::{Command, Commands};
use frankenstein::{
    client_reqwest::Bot, methods::{GetUpdatesParams, SendMessageParams}, types::{ChatType, MessageOrigin, ReplyParameters}, updates::UpdateContent, AsyncTelegramApi, ParseMode
};
use log::{debug, error, info};
use serde::Deserialize;
use setup::{setup_commands, setup_rights};

/// Configuration for the bot.
#[derive(Deserialize, Clone, PartialEq, Eq)]
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
                        // TODO: Handle inline
                        continue;
                    };

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
                            None => {
                                // We only handle those in private chats, to prevent polluting the groups
                                if matches!(msg.chat.type_field, ChatType::Private) {
                                    let reply = if let Some(origin) = msg.forward_origin {
                                        // The message is forwarded
                                        if let MessageOrigin::User(origin_user) = *origin {
                                            // ... from a user
                                            let user = origin_user.sender_user;
                                            format!("TODO: Dox user {}", user.id)
                                        } else {
                                            // ... from something else
                                            debug!("Cannot determine the origin as a user: {origin:?}");
                                            include_str!("messages/invalid-origin.html").to_string()
                                        }
                                    } else {
                                        // Not forwarded message - incomprehensible
                                        debug!("Not a command or forwarded message: {text:?}", text = msg.text.as_ref());
                                        include_str!("messages/incomprehensible.html").to_string()
                                    };
                                    Some(reply)
                                } else {
                                    None
                                }
                            },
                        };
                        if let Some(reply) = reply {
                            let reply_param = ReplyParameters::builder().message_id(message_id).build();
                            let send_message_param = SendMessageParams::builder()
                                .chat_id(chat_id)
                                .text(reply)
                                .reply_parameters(reply_param)
                                .parse_mode(ParseMode::Html)
                                .build();
                            if let Err(err) = bot.send_message(&send_message_param).await {
                                error!("Failed to send message: {err}");
                            }
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

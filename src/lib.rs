//! # `doxing-emulator` library crate
//!
//! If you are reading this, you are reading the documentation for the `doxing-emulator` library crate. For the cli, kindly refer to the README file.

#![deny(missing_docs)]
#![warn(clippy::all, clippy::nursery, clippy::pedantic, clippy::cargo)]

use anyhow::Result;
use serde::Deserialize;

/// Configuration for the bot.
#[derive(Deserialize)]
pub struct Config {
    /// The token for the bot.
    pub token: String,
}

/// Runs the bot.
pub async fn run(config: Config) -> Result<()> {
    Ok(())
}

#![warn(clippy::all, clippy::nursery, clippy::pedantic, clippy::cargo)]

use anyhow::{Ok, Result};
use doxing_emulator::{Config, run};
use env_logger::Env;
use std::io::Write;
use tokio::fs::read_to_string;
use toml::from_str;

#[tokio::main]
async fn main() -> Result<()> {
    // Logging
    env_logger::Builder::from_env(Env::default().default_filter_or("info"))
        .format(|buf, record| {
            let level = record.level();
            let style = buf.default_level_style(level);
            writeln!(buf, "[{style}{level}{style:#}] {}", record.args())
        })
        .init();

    // Running
    let config = read_config().await?;
    run(config).await
}

async fn read_config() -> Result<Config> {
    let path = std::env::args()
        .nth(1)
        .unwrap_or_else(|| "config.toml".to_string());
    let config = read_to_string(path).await?;
    let config = from_str(&config)?;

    Ok(config)
}

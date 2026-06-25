#![warn(clippy::all, clippy::nursery, clippy::pedantic, clippy::cargo)]
#![allow(clippy::multiple_crate_versions, reason = "Dependency")]
#![allow(clippy::future_not_send, reason = "Single-threaded runtime")]

use anyhow::Result;
use compio::fs::read;
use doxing_emulator::{Config, run};
use env_logger::Env;
use futures_util::FutureExt;
use log::info;
use std::io::Write;
use toml::from_slice;

#[compio::main]
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
    info!("Starting doxing emulator...");
    let config = read_config().await?;
    run(config).await
}

fn read_config() -> impl Future<Output = Result<Config>> {
    let path = std::env::args()
        .nth(1)
        .unwrap_or_else(|| "config.toml".to_string());
    read(path).map(|config| {
        let config = config?;
        info!("Config read complete, parsing...");
        Ok(from_slice(&config)?)
    })
}

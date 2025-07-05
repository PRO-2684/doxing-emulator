# doxing-emulator

[![GitHub License](https://img.shields.io/github/license/PRO-2684/doxing-emulator?logo=opensourceinitiative)](https://github.com/PRO-2684/doxing-emulator/blob/main/LICENSE)
[![GitHub Workflow Status](https://img.shields.io/github/actions/workflow/status/PRO-2684/doxing-emulator/release.yml?logo=githubactions)](https://github.com/PRO-2684/doxing-emulator/blob/main/.github/workflows/release.yml)
[![GitHub Release](https://img.shields.io/github/v/release/PRO-2684/doxing-emulator?logo=githubactions)](https://github.com/PRO-2684/doxing-emulator/releases)
[![GitHub Downloads (all assets, all releases)](https://img.shields.io/github/downloads/PRO-2684/doxing-emulator/total?logo=github)](https://github.com/PRO-2684/doxing-emulator/releases)
[![Crates.io Version](https://img.shields.io/crates/v/doxing-emulator?logo=rust)](https://crates.io/crates/doxing-emulator)
[![Crates.io Total Downloads](https://img.shields.io/crates/d/doxing-emulator?logo=rust)](https://crates.io/crates/doxing-emulator)
[![docs.rs](https://img.shields.io/docsrs/doxing-emulator?logo=rust)](https://docs.rs/doxing-emulator)

Telegram doxing emulator.

## 📥 Installation

### Using [`binstall`](https://github.com/cargo-bins/cargo-binstall)

```shell
cargo binstall doxing-emulator
```

### Downloading from Releases

Navigate to the [Releases page](https://github.com/PRO-2684/doxing-emulator/releases) and download respective binary for your platform. Make sure to give it execute permissions.

### Compiling from Source

```shell
cargo install doxing-emulator
```

## 🛠️ Setup

### 🤖 Bot

1. Go to [@BotFather](https://t.me/BotFather) and create a bot.
2. Enable [Inline Mode](https://core.telegram.org/bots/inline) and disable [Privacy Mode](https://core.telegram.org/bots/features#privacy-mode).
    - By enabling inline mode, you can dox in any chat via inline queries.
    - By disabling privacy mode, the bot can know who you've replied with a `/dox` command, so it can dox him for you.
3. Customize name, description etc. as u wish.

### ⚙️ Configuration

The configuration file is in the format of TOML, with the following key(s):

- `token`: The token for the bot.

### ▶️ Running

```shell
doxing-emulator path/to/config.toml
```

If the path is not specified, defaults to `config.toml`.

## 📖 Usage

- `/dox <user_id>` - doxes the user with provided user id
- Reply a message with `/dox` - doxes the sender of the replied message
- Forward a message to the bot - doxes the sender of the forwarded message
- Inline query - doxes the user with provided user id

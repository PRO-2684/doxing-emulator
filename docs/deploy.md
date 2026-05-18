# Deployment

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
3. Enable [Guest Mode](https://core.telegram.org/bots/features#guest-bots).
4. Customize name, description etc. as u wish.

### ⚙️ Configuration

The configuration file is in the format of TOML, with the following key(s):

- `token`: The token for the bot.
- `proxy`: Optional proxy URL. Uses [cyper::proxy::Proxy](https://docs.rs/cyper/0.9.0-rc.2/cyper/proxy/struct.Proxy.html).

### ▶️ Running

```shell
doxing-emulator path/to/config.toml
```

If the path is not specified, defaults to `config.toml`.

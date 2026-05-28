# doxing-emulator

[![GitHub License](https://img.shields.io/github/license/PRO-2684/doxing-emulator?logo=opensourceinitiative)](https://github.com/PRO-2684/doxing-emulator/blob/main/LICENSE)
[![GitHub Workflow Status](https://img.shields.io/github/actions/workflow/status/PRO-2684/doxing-emulator/release.yml?logo=githubactions)](https://github.com/PRO-2684/doxing-emulator/blob/main/.github/workflows/release.yml)
[![GitHub Release](https://img.shields.io/github/v/release/PRO-2684/doxing-emulator?logo=githubactions)](https://github.com/PRO-2684/doxing-emulator/releases)
[![GitHub Downloads (all assets, all releases)](https://img.shields.io/github/downloads/PRO-2684/doxing-emulator/total?logo=github)](https://github.com/PRO-2684/doxing-emulator/releases)
[![Crates.io Version](https://img.shields.io/crates/v/doxing-emulator?logo=rust)](https://crates.io/crates/doxing-emulator)
[![Crates.io Total Downloads](https://img.shields.io/crates/d/doxing-emulator?logo=rust)](https://crates.io/crates/doxing-emulator)
[![docs.rs](https://img.shields.io/docsrs/doxing-emulator?logo=rust)](https://docs.rs/doxing-emulator)

Telegram doxing **emulator**. You can find me at [@DoxingEmulatorBot](https://t.me/DoxingEmulatorBot?start=help).

## 📖 Usage

- `/dox`
    - `/dox <user_id>`: doxes the user with provided user id
    - Reply a message with `/dox` - doxes the sender of the replied message
    - Otherwise, doxes the sender of the command
- Forward a message to the bot - doxes the sender of the forwarded message
- Inline query - doxes the user with provided user id
- [Guest interaction](https://telegram.org/blog/ai-bot-revolution-11-new-features#guest-bots)
    - Mention the bot in the message (do not select any inline results)
    - If user id provided, doxes the user with provided user id
    - If replied to a message, doxes the sender of the replied message
    - Otherwise, doxes the sender of the mentioning message

A sample response could be:

您好，请问是用户 ID 为 `0000000000`，用户名为 `@username`，生日在 01 月 01 日，位于 `地球`，开通了 tg 空间 @channel 的 `FirstName` 富哥吗？

## 📚 Documentation

- [Deployment](./docs/deploy.md)
- [Privacy](./docs/privacy.md)

## ✅ TODO

- Avatar DC.
- Cache for `get_full_info` and `get_user_by_id`.

# doxing-emulator

[![GitHub License](https://img.shields.io/github/license/PRO-2684/doxing-emulator?logo=opensourceinitiative)](https://github.com/PRO-2684/doxing-emulator/blob/main/LICENSE)
[![GitHub Workflow Status](https://img.shields.io/github/actions/workflow/status/PRO-2684/doxing-emulator/release.yml?logo=githubactions)](https://github.com/PRO-2684/doxing-emulator/blob/main/.github/workflows/release.yml)
[![GitHub Release](https://img.shields.io/github/v/release/PRO-2684/doxing-emulator?logo=githubactions)](https://github.com/PRO-2684/doxing-emulator/releases)
[![GitHub Downloads (all assets, all releases)](https://img.shields.io/github/downloads/PRO-2684/doxing-emulator/total?logo=github)](https://github.com/PRO-2684/doxing-emulator/releases)
[![Crates.io Version](https://img.shields.io/crates/v/doxing-emulator?logo=rust)](https://crates.io/crates/doxing-emulator)
[![Crates.io Total Downloads](https://img.shields.io/crates/d/doxing-emulator?logo=rust)](https://crates.io/crates/doxing-emulator)
[![docs.rs](https://img.shields.io/docsrs/doxing-emulator?logo=rust)](https://docs.rs/doxing-emulator)

Telegram 开盒模拟器。

## ⚙️ Automatic Releases Setup

1. [Create a new GitHub repository](https://github.com/new) with the name `doxing-emulator` and push this generated project to it.
2. Enable Actions for the repository, and grant "Read and write permissions" to the workflow [here](https://github.com/PRO-2684/doxing-emulator/settings/actions).
3. [Generate an API token on crates.io](https://crates.io/settings/tokens/new), with the following setup:

    - `Name`: `doxing-emulator`
    - `Expiration`: `No expiration`
    - `Scopes`: `publish-new`, `publish-update`
    - `Crates`: `doxing-emulator`

4. [Add a repository secret](https://github.com/PRO-2684/doxing-emulator/settings/secrets/actions/new) named `CARGO_TOKEN` with the generated token as its value.
5. Consider removing this section and updating this README with your own project information.

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

## 💡 Examples

TODO

## 📖 Usage

TODO

## 🎉 Credits

TODO

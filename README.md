# Rust CLI Template

CLI starter template for Rust.

## Features

- Simple CLI starer application
  - Argument parsing with [_clap_](https://docs.rs/clap/latest/clap/)
  - Error handling with [_anyhow_](https://docs.rs/anyhow/latest/anyhow/)
  - Integration testing with [_assert_cmd_](https://docs.rs/assert_cmd/latest/assert_cmd/)
- Multi-platform builds and packaging with [_cargo-dist_](https://axodotdev.github.io/cargo-dist/book/)
  - macOS and Linux builds
  - Homebrew and shell installers
- Release automation with [_cargo-release_](https://github.com/crate-ci/cargo-release)
- Dependabot keeps Cargo dependencies and Rust toolchain up-to-date
- CI workflow (GitHub Actions)
  - Format check
  - Install check
  - Lint with Clippy
  - Run tests with [_cargo-nextest_](https://nexte.st)
  - Link checking with (_lychee_)(https://lychee.cli.rs)

## Installation

The example application included here is distributed like a real CLI tool would be. There are several installation methods available:

### Homebrew
```
brew install ladvoc/tap/rust-cli-template
```

### Install script
```
curl --proto '=https' --tlsv1.2 -LsSf https://github.com/ladvoc/rust-cli-template/releases/download/v0.1.0/rust-cli-template-installer.sh | sh
```

### Cargo (pre-built binary)
```
cargo binstall rust-cli-template
```

### Cargo (build from source)
```
cargo install rust-cli-template
```

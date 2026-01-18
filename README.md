# Rust CLI Template

CLI starter template for Rust.

## Features

- Argument parsing with [_clap_](https://docs.rs/clap/latest/clap/)
- Error handling with [_anyhow_](https://crates.io/crates/anyhow)
- Multi-platform builds and packaging with [_cargo-dist_](https://axodotdev.github.io/cargo-dist/book/)
  - macOS and Linux
  - Homebrew and shell installer
- Dependabot keeps Cargo dependencies and Rust toolchain up-to-date
- CI workflow (GitHub Actions)
  - Format check
  - Install check
  - Lint with Clippy
  - Run tests with [_cargo-nextest_](https://nexte.st)

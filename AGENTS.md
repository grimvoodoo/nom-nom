# AGENTS.md

This file provides guidance to WARP (warp.dev) when working with code in this repository.

## Project Overview

nom-nom is a web-based cookbook platform built with Rust (2024 edition). Users can manage recipes, track their ingredient inventory ("larder"), and filter recipes by what they have on hand. The project includes an AI microservice for extracting ingredients from uploaded photos.

See [GOALS.md](GOALS.md) for the roadmap and [README.md](README.md) for an overview.

## Security Requirements

**All code changes must follow the security policy in [docs/SECURITY.md](docs/SECURITY.md).**

Key rules:
- **No hardcoded secrets** — use environment variables
- **No sensitive data in localStorage** — only UI preferences (e.g., theme)
- **Parameterized queries only** — never concatenate user input into SQL
- **TLS required** — all database and API connections must be encrypted
- **Validate all input** — server-side validation is mandatory
- **rustls only** — no OpenSSL (see below)

## TLS & Dependencies

**This app uses rustls for TLS and deploys to scratch containers.**

Critical rules:
- **Use rustls** — Never use OpenSSL or native-tls
- **No C dependencies** — Scratch containers have no system libraries
- **Check transitive deps** — Run `cargo tree -i openssl` to verify no OpenSSL
- **Use `*-rustls` features** — When adding crates, prefer rustls variants (e.g., `reqwest/rustls-tls`)

The app terminates TLS directly without a sidecar proxy.

## Configuration

**All configuration is via environment variables** — see [docs/CONFIGURATION.md](docs/CONFIGURATION.md).

This app is deployed via Kubernetes. Never hardcode configuration values:
- Read config from `std::env::var()` at startup
- Provide sensible defaults for optional settings
- Fail fast with clear errors for missing required settings
- Update `.env.example` when adding new config options

## Build Commands

```bash
# Run development server (with hot reload)
dx serve

# Build for web (WASM)
dx build --release

# Build for desktop
dx build --release --platform desktop

# Run without Dioxus CLI
cargo run
```

## Frontend (Tailwind CSS + DaisyUI)

```bash
# Install dependencies
npm install

# Build CSS once
npm run build:css

# Watch CSS during development
npm run watch:css
```

See [docs/STYLING.md](docs/STYLING.md) for theming and component patterns.

## Testing

```bash
# Run all tests
cargo test

# Run a specific test by name
cargo test test_name

# Run tests with output shown
cargo test -- --nocapture
```

## Code Quality

```bash
# Check for compilation errors without building
cargo check

# Run the linter
cargo clippy

# Format code
cargo fmt

# Format check (CI-friendly, doesn't modify files)
cargo fmt --check
```

# AGENTS.md

This file provides guidance to WARP (warp.dev) when working with code in this repository.

## Project Overview

nom-nom is a web-based cookbook platform built with Rust (2024 edition). Users can manage recipes, track their ingredient inventory ("larder"), and filter recipes by what they have on hand. The project includes an AI microservice for extracting ingredients from uploaded photos.

See [GOALS.md](GOALS.md) for the roadmap and [README.md](README.md) for an overview.

## Build Commands

```bash
# Build the project
cargo build

# Build with optimizations
cargo build --release

# Run the project
cargo run

# Run with release optimizations
cargo run --release
```

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

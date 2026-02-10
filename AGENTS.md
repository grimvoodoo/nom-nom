# AGENTS.md

This file provides guidance to WARP (warp.dev) when working with code in this repository.

## Project Overview

nom-nom is a Rust project using the 2024 edition.

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

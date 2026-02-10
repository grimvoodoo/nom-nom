# nom-nom 🍳

A smart cookbook that knows what's in your kitchen.

## What is nom-nom?

nom-nom is a web-based recipe platform that connects your recipes to your actual ingredients. Instead of browsing recipes and then checking if you have what you need, nom-nom flips the script—tell it what's in your kitchen, and it tells you what you can make.

## Key Features

- **Recipe Management** — Add, organize, and search your personal recipe collection
- **Larder Tracking** — Maintain an inventory of ingredients you have on hand
- **Smart Filtering** — Find recipes you can make right now with what's in your larder
- **AI-Powered Inventory** — Upload photos of your pantry, fridge, or shopping receipts to automatically update your ingredient list

## Architecture

nom-nom is built as a collection of microservices:

- **Core API** (Rust) — Handles recipes, user data, and larder management
- **AI Service** — Processes images to extract ingredient information

## Project Status

🚧 **Early Development** — This project is just getting started.

See [GOALS.md](GOALS.md) for the roadmap and planned features.

## Development

```bash
# Build
cargo build

# Run
cargo run

# Test
cargo test

# Lint
cargo clippy

# Format
cargo fmt
```

## License

This project is licensed under the [PolyForm Noncommercial License 1.0.0](LICENSE). You may use it for personal, educational, and noncommercial purposes. Commercial use requires a separate license from the author.

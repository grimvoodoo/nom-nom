# nom-nom 🍳

A smart cookbook that knows what's in your kitchen.

## What is nom-nom?

nom-nom is a web-based recipe platform that connects your recipes to your actual ingredients. Instead of browsing recipes and then checking if you have what you need, nom-nom flips the script—tell it what's in your kitchen, and it tells you what you can make.

## Key Features

- **Recipe Management** — Add, organize, and search your personal recipe collection
- **Larder Tracking** — Maintain an inventory of ingredients you have on hand
- **Smart Filtering** — Find recipes you can make right now with what's in your larder
- **AI-Powered Inventory** — Upload photos of your pantry, fridge, or shopping receipts to automatically update your ingredient list
- **Security by Default** — Encrypted data, secure authentication, no sensitive data in client storage

## Architecture

nom-nom uses [Dioxus](https://dioxuslabs.com/) for a unified Rust codebase that targets:

- **Web** (WASM) — Primary platform
- **Desktop** — Native apps via WebView
- **Mobile** — iOS and Android (planned)

Additional services:

- **AI Service** — Processes images to extract ingredient information

**Deployment:** Kubernetes-native with environment variable configuration. See [docs/CONFIGURATION.md](docs/CONFIGURATION.md).

## Project Status

🚧 **Early Development** — This project is just getting started.

See [GOALS.md](GOALS.md) for the roadmap and planned features.

## Development

### Prerequisites

- Rust (2024 edition)
- [Dioxus CLI](https://dioxuslabs.com/learn/0.6/getting_started): `cargo install dioxus-cli`
- Node.js (for Tailwind CSS)

### Getting Started

```bash
# Install CSS dependencies
npm install

# Build CSS
npm run build:css

# Run development server (with hot reload)
dx serve

# Or run without Dioxus CLI
cargo run
```

### Build for Production

```bash
# Web (WASM)
dx build --release

# Desktop
dx build --release --platform desktop
```

### Testing & Quality

```bash
cargo test           # Run tests
cargo clippy         # Lint
cargo fmt            # Format
```

## License

This project is licensed under the [PolyForm Noncommercial License 1.0.0](LICENSE). You may use it for personal, educational, and noncommercial purposes. Commercial use requires a separate license from the author.

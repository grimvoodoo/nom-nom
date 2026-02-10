# Contributing to nom-nom

Thank you for considering contributing to nom-nom!

## Contributor License Agreement

**Before contributing, please read and understand our [Contributor License Agreement (CLA)](CLA.md).**

By submitting a pull request or patch, you agree to the terms of the CLA. This grants the project maintainer the ability to relicense contributions, which is necessary because this project uses a noncommercial license while the maintainer retains commercial rights.

## How to Contribute

1. Fork the repository
2. Create a feature branch (`git checkout -b feature/your-feature`)
3. Make your changes
4. Run tests and linting:
   ```bash
   cargo test
   cargo clippy
   cargo fmt --check
   ```
5. Commit your changes with a clear message
6. Push to your fork and open a pull request

## Code Style

- Run `cargo fmt` before committing
- Ensure `cargo clippy` passes without warnings
- Write tests for new functionality

## Questions?

Open an issue if you have questions or want to discuss a feature before implementing it.

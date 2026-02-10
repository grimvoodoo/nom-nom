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

## Dependencies

**This app deploys to scratch containers.** Before adding or updating dependencies:

```bash
# Verify no OpenSSL dependency
cargo tree -i openssl
cargo tree -i openssl-sys
```

- Use `rustls` variants (e.g., `reqwest = { features = ["rustls-tls"], default-features = false }`)
- Avoid crates with C dependencies
- If unsure, ask before adding

## Security

**Read [docs/SECURITY.md](docs/SECURITY.md) before contributing.**

Your PR will be reviewed for:
- No hardcoded secrets or credentials
- Proper input validation
- Parameterized database queries
- No sensitive data in client-side storage
- Appropriate error handling (no sensitive data in error messages)

## Questions?

Open an issue if you have questions or want to discuss a feature before implementing it.

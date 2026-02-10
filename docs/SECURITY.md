# Security Policy

Security is a core design principle of nom-nom, not an afterthought. All code contributions and architectural decisions must consider security implications.

## Core Principles

1. **Defense in Depth** — Multiple layers of security; never rely on a single control
2. **Least Privilege** — Grant minimum permissions required for functionality
3. **Secure by Default** — Default configurations must be secure; insecure options require explicit opt-in
4. **Zero Trust** — Verify everything; trust nothing implicitly

## Secrets Management

### Hard Rules

- **NEVER** commit secrets, API keys, tokens, or credentials to git
- **NEVER** hardcode sensitive values in source code
- **NEVER** log sensitive data (tokens, passwords, PII)
- **NEVER** include secrets in error messages

### How to Handle Secrets

**Development:**
- Use environment variables loaded from `.env` files (gitignored)
- Use a local secrets manager or vault

**Production:**
- Use a secrets manager (e.g., HashiCorp Vault, cloud provider secrets)
- Inject secrets via environment variables at runtime
- Rotate secrets regularly

**Example `.env` structure:**
```
DATABASE_URL=postgresql://...
SESSION_SECRET=...
API_KEY=...
```

Ensure `.env` is in `.gitignore` (it already is).

## Data Protection

### Encryption in Transit

- All external communications **MUST** use TLS 1.3 (minimum TLS 1.2)
- Database connections **MUST** use SSL/TLS
- API endpoints **MUST** be HTTPS-only in production
- Reject plaintext HTTP connections
- **TLS is terminated in-app using rustls** — no sidecar proxy required

### rustls Requirement

This application **MUST** use rustls for all TLS operations. OpenSSL is prohibited because:

1. **Scratch container deployment** — No system libraries available
2. **Pure Rust** — No C dependencies, better memory safety
3. **Simpler builds** — No cross-compilation issues with OpenSSL

When adding dependencies:

```bash
# Check for OpenSSL in dependency tree
cargo tree -i openssl

# If openssl appears, find alternative crates or use rustls features
```

Common crate configurations:

```toml
# Use rustls variants
reqwest = { version = "...", default-features = false, features = ["rustls-tls"] }
tokio-postgres = { version = "...", features = ["with-rustls"] }
sqlx = { version = "...", features = ["runtime-tokio-rustls", "postgres"] }
```

### Encryption at Rest

- Database storage is encrypted (CockroachDB handles this)
- Sensitive fields (if stored locally) must be encrypted before storage
- Use authenticated encryption (e.g., AES-GCM, ChaCha20-Poly1305)

### Client-Side Storage

**Do NOT store in browser localStorage/sessionStorage:**
- Authentication tokens with long expiry
- Refresh tokens
- User credentials
- Decryption keys
- Sensitive PII

**Acceptable for localStorage:**
- UI preferences (theme, language)
- Non-sensitive cached data
- Short-lived, non-critical session identifiers (with appropriate expiry)

**For sensitive client-side state:**
- Use HTTP-only, Secure, SameSite cookies for auth tokens
- Use in-memory storage that clears on page close when possible
- If persistence is required, encrypt before storing and manage keys carefully

## Authentication & Sessions

- Passwords must be hashed with Argon2id (preferred) or bcrypt
- Session tokens must be cryptographically random (256-bit minimum)
- Implement proper session expiration and rotation
- Support secure logout (invalidate server-side session)
- Consider implementing:
  - Rate limiting on auth endpoints
  - Account lockout after failed attempts
  - Multi-factor authentication

## Database (CockroachDB)

### Connection Security

```rust path=null start=null
// Always use SSL for database connections
// Connection string must include sslmode=verify-full in production
let db_url = std::env::var("DATABASE_URL")
    .expect("DATABASE_URL must be set");
```

### Query Safety

- **ALWAYS** use parameterized queries — never string concatenation
- Validate and sanitize all user input
- Use ORM/query builder with built-in protections when possible

### Access Control

- Use database roles with minimal required permissions
- Application user should NOT have schema modification rights in production
- Separate read/write roles if applicable

## Input Validation

- Validate all input on the server side (client validation is UX, not security)
- Use allowlists over denylists
- Validate type, length, format, and range
- Sanitize output to prevent XSS

## API Security

- Authenticate all non-public endpoints
- Implement authorization checks for every action
- Use CSRF protection for state-changing operations
- Set appropriate CORS policies
- Rate limit API endpoints
- Validate Content-Type headers

## Security Headers

Production deployments should include:

```
Strict-Transport-Security: max-age=31536000; includeSubDomains
Content-Security-Policy: default-src 'self'; ...
X-Content-Type-Options: nosniff
X-Frame-Options: DENY
Referrer-Policy: strict-origin-when-cross-origin
Permissions-Policy: geolocation=(), camera=(), microphone=()
```

## Dependency Security

- Regularly audit dependencies: `cargo audit`
- Keep dependencies updated
- Review new dependencies before adding
- Prefer well-maintained, widely-used crates

## Reporting Vulnerabilities

If you discover a security vulnerability, please report it privately rather than opening a public issue. Contact the maintainer directly.

## Checklist for Code Review

- [ ] No hardcoded secrets or credentials
- [ ] Sensitive data not logged
- [ ] User input validated and sanitized
- [ ] Parameterized queries used (no SQL injection risk)
- [ ] Authentication/authorization checks in place
- [ ] Sensitive data not exposed to client unnecessarily
- [ ] Error messages don't leak sensitive information
- [ ] New dependencies reviewed for security

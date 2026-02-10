# Configuration Guide

nom-nom is configured entirely via **environment variables**. This enables deployment across different environments (development, test, production) without code changes — the same container image works everywhere with different configurations.

## Design Principles

1. **No hardcoded configuration** — All configurable values come from environment variables
2. **Sensible defaults** — Where safe, provide defaults for optional settings
3. **Fail fast** — Required variables with no default should cause immediate startup failure with clear error messages
4. **No config files in container** — Configuration is injected, not baked in

## Environment Variables

### Required

These must be set or the application will not start:

| Variable | Description | Example |
|----------|-------------|---------|
| `DATABASE_URL` | CockroachDB connection string (with SSL) | `postgresql://user:pass@host:26257/db?sslmode=verify-full` |
| `SESSION_SECRET` | Secret for signing sessions (min 32 bytes) | `openssl rand -base64 32` |

### Optional (with defaults)

| Variable | Description | Default |
|----------|-------------|---------|
| `HOST` | Address to bind | `0.0.0.0` |
| `PORT` | Port to listen on | `8080` |
| `LOG_LEVEL` | Logging verbosity | `info` |
| `ENVIRONMENT` | Runtime environment | `development` |

### TLS Configuration

The app terminates TLS directly using rustls (no sidecar needed).

**TLS is required by default.** The app will not start without valid certificate paths unless explicitly running in insecure mode.

| Variable | Description | Required |
|----------|-------------|----------|
| `TLS_CERT_PATH` | Path to TLS certificate (PEM) | Yes (unless `INSECURE_NO_TLS=true`) |
| `TLS_KEY_PATH` | Path to TLS private key (PEM) | Yes (unless `INSECURE_NO_TLS=true`) |
| `INSECURE_NO_TLS` | **Development only.** Disable TLS requirement | No (default: `false`) |

⚠️ **Warning:** `INSECURE_NO_TLS=true` should **never** be used in production. The app will log a warning on startup if this is set.

In Kubernetes, mount certificates from a Secret:

```yaml
volumes:
  - name: tls-certs
    secret:
      secretName: nom-nom-tls
containers:
  - name: nom-nom
    volumeMounts:
      - name: tls-certs
        mountPath: /certs
        readOnly: true
```

### Feature Flags

| Variable | Description | Default |
|----------|-------------|---------|
| `ENABLE_AI_SERVICE` | Enable AI ingredient recognition | `false` |
| `AI_SERVICE_URL` | AI microservice endpoint | — |

## Kubernetes Deployment

### ConfigMap (non-sensitive)

```yaml
apiVersion: v1
kind: ConfigMap
metadata:
  name: nom-nom-config
data:
  HOST: "0.0.0.0"
  PORT: "8080"
  LOG_LEVEL: "info"
  ENVIRONMENT: "production"
```

### Secret (sensitive)

```yaml
apiVersion: v1
kind: Secret
metadata:
  name: nom-nom-secrets
type: Opaque
stringData:
  DATABASE_URL: "postgresql://..."
  SESSION_SECRET: "..."
```

### Deployment

```yaml
apiVersion: apps/v1
kind: Deployment
metadata:
  name: nom-nom
spec:
  template:
    spec:
      containers:
        - name: nom-nom
          image: nom-nom:latest
          envFrom:
            - configMapRef:
                name: nom-nom-config
            - secretRef:
                name: nom-nom-secrets
          ports:
            - containerPort: 8080
```

## Local Development

Copy the example file and fill in your values:

```bash
cp .env.example .env
# Edit .env with your local settings
```

The `.env` file is gitignored and should never be committed.

## Adding New Configuration

When adding a new configurable value:

1. **Add to `.env.example`** — Document the variable with a placeholder or example
2. **Update this document** — Add to the appropriate table above
3. **Use `std::env::var`** — Read from environment at startup, not on each request
4. **Provide defaults where safe** — Use `var("NAME").unwrap_or_else(|_| "default".into())`
5. **Fail fast for required vars** — Use `var("NAME").expect("NAME must be set")`
6. **Never log secret values** — Log that config was loaded, not what it contains

### Example Pattern

```rust path=null start=null
use std::env;

pub struct Config {
    pub database_url: String,
    pub host: String,
    pub port: u16,
    pub log_level: String,
}

impl Config {
    pub fn from_env() -> Self {
        Self {
            // Required - fail if missing
            database_url: env::var("DATABASE_URL")
                .expect("DATABASE_URL must be set"),
            
            // Optional with defaults
            host: env::var("HOST")
                .unwrap_or_else(|_| "0.0.0.0".into()),
            port: env::var("PORT")
                .unwrap_or_else(|_| "8080".into())
                .parse()
                .expect("PORT must be a valid number"),
            log_level: env::var("LOG_LEVEL")
                .unwrap_or_else(|_| "info".into()),
        }
    }
}
```

## Environment Differences

| Setting | Development | Test | Production |
|---------|-------------|------|------------|
| `LOG_LEVEL` | `debug` | `info` | `warn` |
| `ENVIRONMENT` | `development` | `test` | `production` |
| `INSECURE_NO_TLS` | `true` | `false` | `false` |
| `DATABASE_URL` | Local CockroachDB | Test cluster | Production cluster |
| SSL Mode | `require` | `verify-full` | `verify-full` |

## Build & Deployment

### Scratch Container

This app is deployed in a **scratch container** (no OS, no system libraries). This requires:

- **rustls only** — OpenSSL requires system libraries and will not work
- **Static binary** — Compile with `RUSTFLAGS="-C target-feature=+crt-static"`
- **No C dependencies** — Pure Rust crates only

### Dockerfile Example

```dockerfile
# Build stage
FROM rust:1.75 AS builder
WORKDIR /app
COPY . .
RUN rustup target add x86_64-unknown-linux-musl
RUN cargo build --release --target x86_64-unknown-linux-musl

# Runtime stage (scratch = empty container)
FROM scratch
COPY --from=builder /app/target/x86_64-unknown-linux-musl/release/nom-nom /nom-nom
COPY --from=builder /etc/ssl/certs/ca-certificates.crt /etc/ssl/certs/
ENTRYPOINT ["/nom-nom"]
```

### Verifying No OpenSSL

Before merging any dependency changes:

```bash
# This should return nothing if clean
cargo tree -i openssl
cargo tree -i openssl-sys
```

If OpenSSL appears in the tree, find the crate pulling it in and either:
1. Use a different crate
2. Enable rustls features and disable default features
3. If unavoidable, discuss alternatives

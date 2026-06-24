# AGENTS.md - NDID High-Performance Blockchain Banking System (Rust)

## Project Overview

ระบบ National Digital ID (NDID) ความเร็วสูงบนบล็อกเชนสำหรับธนาคารพาณิชย์ไทย
- **Language:** Rust
- **Runtime:** Tokio async
- **Network Protocol:** QUIC (quinn) + TCP/TLS 1.3 Auto-Fallback
- **API Layer:** GraphQL (async-graphql) over Axum
- **Blockchain:** Substrate (Private Permissioned Ledger)
- **Crypto:** ED25519 (signing), AES-GCM (encryption), SHA-256 (hashing)

## Architecture

```
Mobile/Web Client
    │  HTTPS / GraphQL Query+Subscription
    ▼
GraphQL API Gateway (axum + async-graphql)
    │  QUIC (quinn) / TCP auto-switch
    ├── Bank Node Service (P2P KYC data exchange)
    └── Blockchain Adapter (Substrate tx recording)
```

## Project Structure

```
ndid-highspeed-system/
├── Cargo.toml
├── src/
│   ├── main.rs              # Axum + GraphQL Gateway bootstrap
│   ├── schema.rs            # GraphQL Query, Mutation, Subscription definitions
│   ├── identity.rs          # NDID identity data model + SHA-256 hashing
│   ├── blockchain.rs        # Substrate node client / smart contract calls
│   ├── crypto.rs            # ED25519 signing + AES-GCM encryption
│   ├── p2p_quic.rs          # QUIC connection pool + TCP fallback
│   └── network/
│       ├── mod.rs
│       ├── quic_channel.rs  # Quinn QUIC implementation
│       └── tcp_channel.rs   # Tokio-native TCP+TLS fallback
├── tests/
│   ├── integration/
│   └── fixtures/
├── benches/
└── config/
    └── default.toml
```

## Coding Conventions

### Rust Style

- **Rust 2024 edition** - use latest stable features
- Follow `rustfmt` default style (run `rtk cargo fmt` before commit)
- Use `rtk cargo clippy` - zero warnings allowed in CI
- Prefer `thiserror` for domain errors, `anyhow` for application-level errors
- Never use `unwrap()` in library code - use proper error propagation with `?`
- Use `#[must_use]` on functions returning important values
- Prefer `Arc<T>` for shared immutable state, `Arc<RwLock<T>>` for mutable shared state
- Use `tokio::select!` for concurrent operations, not `futures::join!` unless truly parallel

### Naming

- `snake_case` for functions, variables, and modules
- `PascalCase` for types, traits, and enums
- `SCREAMING_SNAKE_CASE` for constants
- Module names: singular (e.g., `identity`, `blockchain`, not `identities`)
- Error types: `XxxError` (e.g., `BlockchainError`, `CryptoError`)

### Error Handling

- Define domain errors in each module using `thiserror::Error`
- Chain errors: `map_err(|e| MyError::Context { source: e.into() })?`
- Log at `error!` level before returning error in service layer
- Never panic in production code paths - use `catch_unwind` for boundary points

### Async Patterns

- All I/O must be async (tokio runtime)
- Use `tokio::spawn` for fire-and-forget background tasks
- Use `tokio::sync::broadcast` for event broadcasting
- Use `tokio::sync::mpsc` for channel communication
- Always use `tokio::time::timeout` for external calls (QUIC, blockchain, HSM)
- Never block on async code (`block_on` only in tests/benchmarks)

### Security-First Rules

- **No raw strings for secrets** - use `secrecy::SecretString` or `zeroize::Zeroize`
- **No logging of PII/keys** - sanitize before writing to structured logs
- **Use `constant_time_eq`** for all comparison of sensitive data
- **No `unsafe`** without explicit review and justification in comments
- **Audit every new crate** - run `cargo vet` before adding dependencies

### Testing

- Unit tests: `#[cfg(test)]` in same file, integration tests in `tests/`
- Always test both QUIC and TCP paths (mock both)
- Test error paths, not just happy paths
- Use `rstest` for parameterized tests
- Test fixtures in `tests/fixtures/` for shared test data

## Build & Quality Commands

```bash
# Build
rtk cargo build --release          # Production build
rtk cargo check                    # Quick type check

# Lint & Format
rtk cargo clippy -- -D warnings    # Zero warnings
rtk cargo fmt --all -- --check     # Format check

# Test
rtk cargo test                     # Unit + integration tests
rtk cargo test --test '*'          # Integration tests only

# Audit
cargo vet                          # Supply chain audit
cargo audit                        # Security vulnerability scan
```

## Git Conventions

- **Branch naming:** `feat/`, `fix/`, `chore/`, `docs/`
- **Commit messages:** `<type>: <description>` (e.g., `feat: implement QUIC/TCP fallback`)
- Never commit secrets, keys, or PII
- Run `rtk cargo clippy` and `rtk cargo test` before pushing
- PR description must reference issue/task number

## Security Checklist (Every PR)

- [ ] No `unwrap()` in non-test code
- [ ] No secrets/keys in code or logs
- [ ] Error types defined and propagated correctly
- [ ] New crate added with `cargo vet` approved
- [ ] `unsafe` block justified (if any)
- [ ] `timeout()` applied to all external calls
- [ ] Structured logging with PII sanitized

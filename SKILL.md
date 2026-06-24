# SKILL.md - NDID Banking System Development Skill

## Trigger Words

- `build`, `compile`, `lint`, `test`, `deploy`
- `quic`, `tcp`, `fallback`, `network`
- `graphql`, `schema`, `resolver`, `subscription`
- `blockchain`, `substrate`, `consensus`
- `crypto`, `sign`, `encrypt`, `hash`
- `identity`, `kyc`, `ndid`
- `hsm`, `certificate`, `mtls`
- `release`, `version`, `changelog`

## Core Workflow

### 1. Start Development

```bash
rtk cargo check                           # Quick type check first
rtk cargo build --release                 # Full build
rtk cargo clippy -- -D warnings           # Lint
rtk cargo test                            # Run tests
```

### 2. Code Changes Checklist

Before any commit:
1. `rtk cargo fmt --all -- --check` - formatting
2. `rtk cargo clippy -- -D warnings` - zero warnings
3. `rtk cargo test` - all tests pass
4. `cargo vet` - supply chain audit (if new crate added)

### 3. Common Tasks

#### Add new module
```
1. Create src/<module>.rs with #[cfg(test)] mod tests
2. Add mod declaration in src/main.rs
3. Add module-specific error type using thiserror
4. Write tests in same file
5. Run: rtk cargo test --lib <module>
```

#### Add new GraphQL endpoint
```
1. Define type in schema.rs (#[derive(SimpleObject)])
2. Add resolver method with #[Object]
3. Use Context<'_> for dependency injection
4. Return Result<T, Error> - never unwrap
5. Add subscription for real-time if needed
```

#### Add QUIC/TCP connection
```
1. Create channel in network/quic_channel.rs or tcp_channel.rs
2. Implement trait ConnectionChannel
3. Add timeout wrapper: tokio::time::timeout(Duration, op)
4. Handle fallback in connect_with_fallback()
5. Test both paths explicitly
```

#### Add blockchain transaction
```
1. Define call in blockchain.rs
2. Use ED25519 signing via crypto.rs
3. Record hash (SHA-256) on chain, NOT PII
4. Implement compensating transaction for rollback
5. Add audit log with timestamp
```

## Security Patterns

### Never do this
```rust
// ❌ unwrap() in library code
let key = fs::read_to_string("key.pem").unwrap();

// ❌ Logging secrets
println!("Private key: {:?}", secret_key);

// ❌ Timing-dependent comparison
if token_a == token_b { ... }

// ❌ Missing timeout on external call
quic_send(data).await?;
```

### Always do this
```rust
// ✅ Proper error propagation
let key = fs::read_to_string("key.pem")
    .map_err(|e| CryptoError::KeyLoad { source: e.into() })?;

// ✅ Sanitized logging
tracing::info!(request_id = %sanitize(&req_id), "Processing KYC request");

// ✅ Constant-time comparison
if constant_time_eq(&token_a, &token_b) { ... }

// ✅ Timeout on all external calls
tokio::time::timeout(Duration::from_secs(5), quic_send(data)).await??
```

## File Organization Rules

### Module file structure
```rust
// src/<module>.rs

// 1. Imports
use crate::crypto::SigningKey;
use thiserror::Error;

// 2. Public types
#[derive(Debug, Clone)]
pub struct IdentityRecord { ... }

// 3. Error type (one per module)
#[derive(Debug, Error)]
pub enum IdentityError {
    #[error("Hash computation failed: {0}")]
    HashFailed(String),
}

// 4. Public API
pub async fn create_identity(...) -> Result<IdentityRecord, IdentityError> { ... }

// 5. Private helpers
fn compute_hash(data: &[u8]) -> String { ... }

// 6. Unit tests
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_compute_hash() { ... }
}
```

## Dependency Management

### Adding new crates
```bash
cargo add <crate_name>
cargo vet                                    # Audit before commit
```

### Approved crates (security vetted)
- `tokio` - async runtime
- `axum` - HTTP framework
- `async-graphql` - GraphQL
- `quinn` - QUIC
- `rustls` - TLS
- `ed25519-dalek` - signing
- `aes-gcm` - encryption
- `sha2` - hashing
- `thiserror` - error handling
- `tracing` - structured logging
- `serde` / `serde_json` - serialization
- `secrecy` - secret handling
- `zeroize` - memory zeroing

## Release Process

```bash
# 1. Update version in Cargo.toml
# 2. Run full quality suite
rtk cargo clippy -- -D warnings
rtk cargo test
cargo vet
cargo audit

# 3. Build release
rtk cargo build --release

# 4. Tag
git tag -a v<version> -m "Release <version>"
git push origin v<version>
```

## Emergency Procedures

### HSM connection failure
```rust
// Circuit breaker pattern
match hsm.sign(&tx_hash).await {
    Ok(sig) => sig,
    Err(e) => {
        tracing::error!(error = %e, "HSM signing failed - triggering circuit breaker");
        circuit_breaker.open();
        return Err(TxError::HsmUnavailable);
    }
}
```

### Blockchain node offline
```rust
// Graceful degradation - queue and retry
match blockchain.submit(&tx).await {
    Ok(hash) => hash,
    Err(BlockchainError::NodeUnavailable) => {
        tx_queue.push(tx.clone()).await;
        tracing::warn!(tx_id = %tx.id, "Blockchain node offline - queued for retry");
        return Ok(TxStatus::Queued);
    }
    Err(e) => return Err(e.into()),
}
```

## Performance Benchmarks

Target metrics (measured in `benches/`):
- QUIC connection: < 50ms (0-RTT)
- TCP fallback: < 200ms
- GraphQL query: < 100ms P99
- Blockchain tx: < 500ms finality
- HSM signing: < 10ms per operation
- SHA-256 hash: < 1ms per KB

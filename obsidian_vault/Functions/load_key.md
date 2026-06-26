---
type: function
module: "network/tls.rs"
parent: ""
tags: [rust, function]
---

# Function: load_key

**Defined in:** [network/tls.rs](file:///home/lokis/Documents/banksystemrust/src/network/tls.rs#L210)

## Signature
```rust
fn load_key(path: &str) -> Result<PrivateKeyDer<'static>, TlsError>
```

## Implementation
```rust
fn load_key(path: &str) -> Result<PrivateKeyDer<'static>, TlsError> {
    let bytes = std::fs::read(path)
        .map_err(|e| TlsError::CertLoading(format!("cannot read {path}: {e}")))?;
    let mut reader = &bytes[..];
    rustls_pemfile::private_key(&mut reader)
        .map_err(|e| TlsError::CertLoading(e.to_string()))?
        .ok_or_else(|| TlsError::CertLoading("no private key found".into()))
}
```

## Calls & References
- [[TlsError|TlsError]]

## Called By
- [[RedisCache_set_transaction_status|RedisCache::set_transaction_status]]
- [[TlsContext_load|TlsContext::load]]


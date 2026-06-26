---
type: function
module: "network/tls.rs"
parent: ""
tags: [rust, function]
---

# Function: load_certs

**Defined in:** [network/tls.rs](file:///home/lokis/Documents/banksystemrust/src/network/tls.rs#L200)

## Signature
```rust
fn load_certs(path: &str) -> Result<Vec<CertificateDer<'static>>, TlsError>
```

## Implementation
```rust
fn load_certs(path: &str) -> Result<Vec<CertificateDer<'static>>, TlsError> {
    let bytes = std::fs::read(path)
        .map_err(|e| TlsError::CertLoading(format!("cannot read {path}: {e}")))?;
    let certs = rustls_pemfile::certs(&mut bytes.as_slice())
        .collect::<Result<Vec<_>, _>>()
        .map_err(|e| TlsError::CertLoading(e.to_string()))?;
    Ok(certs)
}
```

## Calls & References
- [[TlsError|TlsError]]

## Called By
- [[TlsContext_load|TlsContext::load]]
- [[TlsContext_add_ca_cert|TlsContext::add_ca_cert]]


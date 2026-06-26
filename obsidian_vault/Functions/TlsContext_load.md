---
type: function
module: "network/tls.rs"
parent: "TlsContext"
tags: [rust, function]
---

# Function: TlsContext::load

**Defined in:** [network/tls.rs](file:///home/lokis/Documents/banksystemrust/src/network/tls.rs#L74)
**Impl Block:** [[TlsContext]]

## Signature
```rust
pub fn load(cert_path: &str, key_path: &str) -> Result<Self, TlsError>
```

## Implementation
```rust
pub fn load(cert_path: &str, key_path: &str) -> Result<Self, TlsError> {
        let certs = load_certs(cert_path)?;
        let key = load_key(key_path)?;
        Ok(Self {
            certs,
            key,
            ca_certs: Vec::new(),
        })
    }
```

## Calls & References
- [[load_key|load::key]]
- [[TlsError|TlsError]]
- [[load_certs|load::certs]]

## Called By
- [[main|main]]


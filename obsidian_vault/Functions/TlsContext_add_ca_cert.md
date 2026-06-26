---
type: function
module: "network/tls.rs"
parent: "TlsContext"
tags: [rust, function]
---

# Function: TlsContext::add_ca_cert

**Defined in:** [network/tls.rs](file:///home/lokis/Documents/banksystemrust/src/network/tls.rs#L85)
**Impl Block:** [[TlsContext]]

## Signature
```rust
pub fn add_ca_cert(&mut self, path: &str) -> Result<(), TlsError>
```

## Implementation
```rust
pub fn add_ca_cert(&mut self, path: &str) -> Result<(), TlsError> {
        let certs = load_certs(path)?;
        self.ca_certs.extend(certs);
        Ok(())
    }
```

## Calls & References
- [[TlsError|TlsError]]
- [[load_certs|load::certs]]

## Called By
- [[main|main]]


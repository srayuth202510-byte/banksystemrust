---
type: function
module: "network/tls.rs"
parent: ""
tags: [rust, function]
---

# Function: fmt

**Defined in:** [network/tls.rs](file:///home/lokis/Documents/banksystemrust/src/network/tls.rs#L42)

## Signature
```rust
fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result
```

## Implementation
```rust
fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("TlsContext")
            .field("cert_count", &self.certs.len())
            .field("ca_count", &self.ca_certs.len())
            .finish()
    }
```

## Calls & References
- [[TlsContext|TlsContext]]

## Called By
- [[percent_encode_userinfo|percent::encode_userinfo]]
- [[main|main]]
- [[TlsContext|TlsContext]]
- [[Protocol_fmt|Protocol::fmt]]
- [[Protocol_fmt|Protocol::fmt]]


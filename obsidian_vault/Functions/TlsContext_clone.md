---
type: function
module: "network/tls.rs"
parent: "TlsContext"
tags: [rust, function]
---

# Function: TlsContext::clone

**Defined in:** [network/tls.rs](file:///home/lokis/Documents/banksystemrust/src/network/tls.rs#L32)
**Impl Block:** [[TlsContext]]

## Signature
```rust
fn clone(&self) -> Self
```

## Implementation
```rust
fn clone(&self) -> Self {
        Self {
            certs: self.certs.clone(),
            key: self.key.clone_key(),
            ca_certs: self.ca_certs.clone(),
        }
    }
```

## Called By
- [[main|main]]
- [[TlsContext|TlsContext]]
- [[start_tcp_server|start::tcp_server]]


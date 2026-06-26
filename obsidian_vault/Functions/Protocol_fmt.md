---
type: function
module: "network/mod.rs"
parent: "Protocol"
tags: [rust, function]
---

# Function: Protocol::fmt

**Defined in:** [network/mod.rs](file:///home/lokis/Documents/banksystemrust/src/network/mod.rs#L65)
**Impl Block:** [[Protocol]]

## Signature
```rust
fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result
```

## Implementation
```rust
fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Quic { .. } => write!(f, "Quic(..)"),
            Self::TcpTls(_) => write!(f, "TcpTls(..)"),
        }
    }
```

## Calls & References
- [[fmt|fmt]]


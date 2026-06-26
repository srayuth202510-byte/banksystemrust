---
type: function
module: "network/mod.rs"
parent: ""
tags: [rust, function]
---

# Function: test_protocol_display

**Defined in:** [network/mod.rs](file:///home/lokis/Documents/banksystemrust/src/network/mod.rs#L299)

## Signature
```rust
fn test_protocol_display()
```

## Implementation
```rust
fn test_protocol_display() {
        assert!(format!("{}", Protocol::Quic).contains("QUIC"));
        assert!(format!("{}", Protocol::Tcp).contains("TCP"));
    }
```

## Calls & References
- [[Protocol|Protocol]]


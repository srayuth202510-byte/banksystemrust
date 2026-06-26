---
type: struct
module: "crypto.rs"
tags: [rust, type/struct]
---

# Struct: SignedPayload

**Defined in:** [crypto.rs](file:///home/lokis/Documents/banksystemrust/src/crypto.rs#L96)

## Definition
```rust
pub struct SignedPayload {
    pub payload: Vec<u8>,
    pub signature: Vec<u8>,
    pub public_key: Vec<u8>,
}
```

## Used By
- [[sign|sign]]
- [[verify|verify]]
- [[process_p2p_message|process::p2p_message]]


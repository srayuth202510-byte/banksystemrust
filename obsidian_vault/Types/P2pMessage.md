---
type: struct
module: "p2p_quic.rs"
tags: [rust, type/struct]
---

# Struct: P2pMessage

**Defined in:** [p2p_quic.rs](file:///home/lokis/Documents/banksystemrust/src/p2p_quic.rs#L36)

## Definition
```rust
pub struct P2pMessage {
    pub from_bank: String,
    pub to_bank: String,
    pub payload: Vec<u8>,
    pub signature: Vec<u8>,
    pub public_key: Vec<u8>,
    pub timestamp: i64,
}
```

## Used By
- [[P2pNode_send_kyc_inner|P2pNode::send_kyc_inner]]
- [[process_p2p_message|process::p2p_message]]


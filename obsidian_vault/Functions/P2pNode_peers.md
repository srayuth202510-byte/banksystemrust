---
type: function
module: "p2p_quic.rs"
parent: "P2pNode"
tags: [rust, function]
---

# Function: P2pNode::peers

**Defined in:** [p2p_quic.rs](file:///home/lokis/Documents/banksystemrust/src/p2p_quic.rs#L168)
**Impl Block:** [[P2pNode]]

## Signature
```rust
pub fn peers(&self) -> &[String]
```

## Implementation
```rust
pub fn peers(&self) -> &[String] {
        &self.peers
    }
```

## Called By
- [[submit_kyc|submit::kyc]]


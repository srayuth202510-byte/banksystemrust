---
type: function
module: "p2p_quic.rs"
parent: "P2pNode"
tags: [rust, function]
---

# Function: P2pNode::add_peer

**Defined in:** [p2p_quic.rs](file:///home/lokis/Documents/banksystemrust/src/p2p_quic.rs#L95)
**Impl Block:** [[P2pNode]]

## Signature
```rust
pub fn add_peer(&mut self, addr: String)
```

## Implementation
```rust
pub fn add_peer(&mut self, addr: String) {
        self.peers.push(addr);
    }
```

## Called By
- [[main|main]]


---
type: function
module: "p2p_quic.rs"
parent: "P2pNode"
tags: [rust, function]
---

# Function: P2pNode::with_timeouts

**Defined in:** [p2p_quic.rs](file:///home/lokis/Documents/banksystemrust/src/p2p_quic.rs#L82)
**Impl Block:** [[P2pNode]]

## Signature
```rust
pub fn with_timeouts(mut self, quic: u64, tcp: u64) -> Self
```

## Implementation
```rust
pub fn with_timeouts(mut self, quic: u64, tcp: u64) -> Self {
        self.quic_timeout_ms = quic;
        self.tcp_timeout_ms = tcp;
        self
    }
```

## Called By
- [[main|main]]


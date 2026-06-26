---
type: function
module: "p2p_quic.rs"
parent: ""
tags: [rust, function]
---

# Function: test_add_peer

**Defined in:** [p2p_quic.rs](file:///home/lokis/Documents/banksystemrust/src/p2p_quic.rs#L207)

## Signature
```rust
fn test_add_peer()
```

## Implementation
```rust
fn test_add_peer() {
        let mut node = test_node("KBANK");
        node.add_peer("10.0.1.50:4433".into());
        assert_eq!(node.peers().len(), 1);
    }
```

## Calls & References
- [[test_node|test::node]]

## Called By
- [[test_round_robin_peer_selection|test::round_robin_peer_selection]]
- [[test_fanout_peer_selection|test::fanout_peer_selection]]


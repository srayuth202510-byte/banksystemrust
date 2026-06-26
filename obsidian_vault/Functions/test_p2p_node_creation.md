---
type: function
module: "p2p_quic.rs"
parent: ""
tags: [rust, function]
---

# Function: test_p2p_node_creation

**Defined in:** [p2p_quic.rs](file:///home/lokis/Documents/banksystemrust/src/p2p_quic.rs#L200)

## Signature
```rust
fn test_p2p_node_creation()
```

## Implementation
```rust
fn test_p2p_node_creation() {
        let node = test_node("BBL");
        assert_eq!(node.bank_code, "BBL");
        assert!(node.peers().is_empty());
    }
```

## Calls & References
- [[test_node|test::node]]


---
type: function
module: "p2p_quic.rs"
parent: ""
tags: [rust, function]
---

# Function: test_fanout_peer_selection

**Defined in:** [p2p_quic.rs](file:///home/lokis/Documents/banksystemrust/src/p2p_quic.rs#L225)

## Signature
```rust
fn test_fanout_peer_selection()
```

## Implementation
```rust
fn test_fanout_peer_selection() {
        let mut node = test_node("KTB").with_load_balancer(LoadBalancerStrategy::Fanout);
        node.add_peer("10.0.1.50:4433".into());
        node.add_peer("10.0.1.51:4433".into());

        assert_eq!(
            node.select_peers(),
            vec!["10.0.1.50:4433".to_string(), "10.0.1.51:4433".to_string()]
        );
    }
```

## Calls & References
- [[test_node|test::node]]
- [[LoadBalancerStrategy|LoadBalancerStrategy]]
- [[test_add_peer|test::add_peer]]


---
type: enum
module: "config.rs"
tags: [rust, type/enum]
---

# Enum: LoadBalancerStrategy

**Defined in:** [config.rs](file:///home/lokis/Documents/banksystemrust/src/config.rs#L43)

## Definition
```rust
pub enum LoadBalancerStrategy {
    #[default]
    RoundRobin,
    Fanout,
}
```

## Used By
- [[P2pNode|P2pNode]]
- [[P2pNode_new|P2pNode::new]]
- [[P2pNode_with_load_balancer|P2pNode::with_load_balancer]]
- [[P2pNode_select_peers|P2pNode::select_peers]]
- [[test_round_robin_peer_selection|test::round_robin_peer_selection]]
- [[test_fanout_peer_selection|test::fanout_peer_selection]]
- [[LoadBalancerConfig|LoadBalancerConfig]]
- [[test_load_balancer_defaults|test::load_balancer_defaults]]


---
type: function
module: "p2p_quic.rs"
parent: "P2pNode"
tags: [rust, function]
---

# Function: P2pNode::with_load_balancer

**Defined in:** [p2p_quic.rs](file:///home/lokis/Documents/banksystemrust/src/p2p_quic.rs#L89)
**Impl Block:** [[P2pNode]]

## Signature
```rust
pub fn with_load_balancer(mut self, load_balancer: LoadBalancerStrategy) -> Self
```

## Implementation
```rust
pub fn with_load_balancer(mut self, load_balancer: LoadBalancerStrategy) -> Self {
        self.load_balancer = load_balancer;
        self
    }
```

## Calls & References
- [[LoadBalancerStrategy|LoadBalancerStrategy]]

## Called By
- [[main|main]]


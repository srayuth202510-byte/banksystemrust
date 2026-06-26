---
type: function
module: "p2p_quic.rs"
parent: "P2pNode"
tags: [rust, function]
---

# Function: P2pNode::select_peers

**Defined in:** [p2p_quic.rs](file:///home/lokis/Documents/banksystemrust/src/p2p_quic.rs#L173)
**Impl Block:** [[P2pNode]]

## Signature
```rust
pub fn select_peers(&self) -> Vec<String>
```

## Implementation
```rust
pub fn select_peers(&self) -> Vec<String> {
        if self.peers.is_empty() {
            return Vec::new();
        }

        match self.load_balancer {
            LoadBalancerStrategy::Fanout => self.peers.clone(),
            LoadBalancerStrategy::RoundRobin => {
                let index = self.next_peer_index.fetch_add(1, Ordering::Relaxed) % self.peers.len();
                vec![self.peers[index].clone()]
            }
        }
    }
```

## Calls & References
- [[LoadBalancerStrategy|LoadBalancerStrategy]]

## Called By
- [[submit_kyc|submit::kyc]]


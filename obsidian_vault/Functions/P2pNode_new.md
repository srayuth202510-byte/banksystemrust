---
type: function
module: "p2p_quic.rs"
parent: "P2pNode"
tags: [rust, function]
---

# Function: P2pNode::new

**Defined in:** [p2p_quic.rs](file:///home/lokis/Documents/banksystemrust/src/p2p_quic.rs#L68)
**Impl Block:** [[P2pNode]]

## Signature
```rust
pub fn new(bank_code: String, keypair: crypto::KeyPair, tls: TlsContext) -> Self
```

## Implementation
```rust
pub fn new(bank_code: String, keypair: crypto::KeyPair, tls: TlsContext) -> Self {
        Self {
            bank_code,
            keypair,
            tls,
            peers: Vec::new(),
            load_balancer: LoadBalancerStrategy::RoundRobin,
            next_peer_index: AtomicUsize::new(0),
            quic_timeout_ms: 500,
            tcp_timeout_ms: 2000,
        }
    }
```

## Calls & References
- [[LoadBalancerStrategy|LoadBalancerStrategy]]
- [[TlsContext|TlsContext]]
- [[KeyPair|KeyPair]]

## Called By
- [[test_node|test::node]]
- [[submit_kyc|submit::kyc]]
- [[main|main]]


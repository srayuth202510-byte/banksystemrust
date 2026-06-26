---
type: struct
module: "p2p_quic.rs"
tags: [rust, type/struct]
---

# Struct: P2pNode

**Defined in:** [p2p_quic.rs](file:///home/lokis/Documents/banksystemrust/src/p2p_quic.rs#L45)

## Definition
```rust
pub struct P2pNode {
    pub bank_code: String,
    pub keypair: crypto::KeyPair,
    pub tls: TlsContext,
    peers: Vec<String>,
    load_balancer: LoadBalancerStrategy,
    next_peer_index: AtomicUsize,
    quic_timeout_ms: u64,
    tcp_timeout_ms: u64,
}
```

## Associated Functions & Methods
- [[P2pNode_new|P2pNode::new]]
- [[P2pNode_with_timeouts|P2pNode::with_timeouts]]
- [[P2pNode_with_load_balancer|P2pNode::with_load_balancer]]
- [[P2pNode_add_peer|P2pNode::add_peer]]
- [[P2pNode_send_kyc|P2pNode::send_kyc]]
- [[P2pNode_send_kyc_inner|P2pNode::send_kyc_inner]]
- [[P2pNode_peers|P2pNode::peers]]
- [[P2pNode_select_peers|P2pNode::select_peers]]

## References
- [[LoadBalancerStrategy|LoadBalancerStrategy]]
- [[TlsContext|TlsContext]]
- [[KeyPair|KeyPair]]

## Used By
- [[fmt|fmt]]
- [[test_node|test::node]]
- [[submit_kyc|submit::kyc]]
- [[main|main]]


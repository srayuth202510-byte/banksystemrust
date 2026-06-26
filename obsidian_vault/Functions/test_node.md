---
type: function
module: "p2p_quic.rs"
parent: ""
tags: [rust, function]
---

# Function: test_node

**Defined in:** [p2p_quic.rs](file:///home/lokis/Documents/banksystemrust/src/p2p_quic.rs#L193)

## Signature
```rust
fn test_node(bank: &str) -> P2pNode
```

## Implementation
```rust
fn test_node(bank: &str) -> P2pNode {
        let kp = KeyPair::generate().unwrap();
        let tls = TlsContext::generate_self_signed().unwrap();
        P2pNode::new(bank.into(), kp, tls)
    }
```

## Calls & References
- [[TlsContext|TlsContext]]
- [[KeyPair_generate|KeyPair::generate]]
- [[KeyPair|KeyPair]]
- [[P2pNode_new|P2pNode::new]]
- [[P2pNode|P2pNode]]
- [[TlsContext_generate_self_signed|TlsContext::generate_self_signed]]
- [[test_generate_self_signed|test::generate_self_signed]]

## Called By
- [[test_p2p_node_creation|test::p2p_node_creation]]
- [[test_add_peer|test::add_peer]]
- [[test_round_robin_peer_selection|test::round_robin_peer_selection]]
- [[test_fanout_peer_selection|test::fanout_peer_selection]]
- [[test_send_kyc_fallback|test::send_kyc_fallback]]
- [[test_submit_transaction_queued_on_no_node|test::submit_transaction_queued_on_no_node]]


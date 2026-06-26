---
type: module
path: "p2p_quic.rs"
tags: [rust, module]
---

# Module: p2p_quic.rs

**File Link:** [p2p_quic.rs](file:///home/lokis/Documents/banksystemrust/src/p2p_quic.rs)

## Types Defined
- [[P2pMessage]] (struct)
- [[P2pNode]] (struct)
- [[P2pError]] (enum)

## Standalone Functions
- [[fmt|fmt]]
- [[test_node|test_node]]
- [[test_p2p_node_creation|test_p2p_node_creation]]
- [[test_add_peer|test_add_peer]]
- [[test_round_robin_peer_selection|test_round_robin_peer_selection]]
- [[test_fanout_peer_selection|test_fanout_peer_selection]]
- [[test_send_kyc_fallback|test_send_kyc_fallback]]

## Implementation Methods
- [[P2pNode_new|P2pNode::new]] (impl for [[P2pNode]])
- [[P2pNode_with_timeouts|P2pNode::with_timeouts]] (impl for [[P2pNode]])
- [[P2pNode_with_load_balancer|P2pNode::with_load_balancer]] (impl for [[P2pNode]])
- [[P2pNode_add_peer|P2pNode::add_peer]] (impl for [[P2pNode]])
- [[P2pNode_send_kyc|P2pNode::send_kyc]] (impl for [[P2pNode]])
- [[P2pNode_send_kyc_inner|P2pNode::send_kyc_inner]] (impl for [[P2pNode]])
- [[P2pNode_peers|P2pNode::peers]] (impl for [[P2pNode]])
- [[P2pNode_select_peers|P2pNode::select_peers]] (impl for [[P2pNode]])


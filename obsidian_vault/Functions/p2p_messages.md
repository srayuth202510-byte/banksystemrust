---
type: function
module: "metrics.rs"
parent: ""
tags: [rust, function]
---

# Function: p2p_messages

**Defined in:** [metrics.rs](file:///home/lokis/Documents/banksystemrust/src/metrics.rs#L35)

## Signature
```rust
pub fn p2p_messages() -> &'static IntCounterVec
```

## Implementation
```rust
pub fn p2p_messages() -> &'static IntCounterVec {
    static P2P_MESSAGES: OnceLock<IntCounterVec> = OnceLock::new();
    P2P_MESSAGES.get_or_init(|| {
        register_int_counter_vec_with_registry!(
            "ndid_p2p_messages_total",
            "Total number of P2P messages",
            &["direction", "bank_code", "status"],
            registry()
        )
        .unwrap()
    })
}
```

## Calls & References
- [[registry|registry]]

## Called By
- [[P2pNode_send_kyc|P2pNode::send_kyc]]
- [[gather_metrics|gather::metrics]]
- [[test_metrics_registration_and_gathering|test::metrics_registration_and_gathering]]
- [[process_p2p_message|process::p2p_message]]


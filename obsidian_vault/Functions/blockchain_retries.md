---
type: function
module: "metrics.rs"
parent: ""
tags: [rust, function]
---

# Function: blockchain_retries

**Defined in:** [metrics.rs](file:///home/lokis/Documents/banksystemrust/src/metrics.rs#L49)

## Signature
```rust
pub fn blockchain_retries() -> &'static IntCounterVec
```

## Implementation
```rust
pub fn blockchain_retries() -> &'static IntCounterVec {
    static BLOCKCHAIN_RETRIES: OnceLock<IntCounterVec> = OnceLock::new();
    BLOCKCHAIN_RETRIES.get_or_init(|| {
        register_int_counter_vec_with_registry!(
            "ndid_blockchain_retries_total",
            "Total number of Substrate blockchain retries",
            &["status"],
            registry()
        )
        .unwrap()
    })
}
```

## Calls & References
- [[registry|registry]]

## Called By
- [[gather_metrics|gather::metrics]]
- [[test_metrics_registration_and_gathering|test::metrics_registration_and_gathering]]
- [[BlockchainClient_retry_all_queued|BlockchainClient::retry_all_queued]]


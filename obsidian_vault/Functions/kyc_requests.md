---
type: function
module: "metrics.rs"
parent: ""
tags: [rust, function]
---

# Function: kyc_requests

**Defined in:** [metrics.rs](file:///home/lokis/Documents/banksystemrust/src/metrics.rs#L21)

## Signature
```rust
pub fn kyc_requests() -> &'static IntCounterVec
```

## Implementation
```rust
pub fn kyc_requests() -> &'static IntCounterVec {
    static KYC_REQUESTS: OnceLock<IntCounterVec> = OnceLock::new();
    KYC_REQUESTS.get_or_init(|| {
        register_int_counter_vec_with_registry!(
            "ndid_kyc_requests_total",
            "Total number of KYC requests submitted",
            &["bank_code", "status"],
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
- [[submit_kyc|submit::kyc]]


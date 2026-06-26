---
type: function
module: "metrics.rs"
parent: ""
tags: [rust, function]
---

# Function: gather_metrics

**Defined in:** [metrics.rs](file:///home/lokis/Documents/banksystemrust/src/metrics.rs#L63)

## Signature
```rust
pub fn gather_metrics() -> Result<String, String>
```

## Implementation
```rust
pub fn gather_metrics() -> Result<String, String> {
    // Access metrics to ensure they are registered
    let _ = kyc_requests();
    let _ = p2p_messages();
    let _ = blockchain_retries();

    let encoder = TextEncoder::new();
    let metric_families = registry().gather();
    let mut buffer = Vec::new();
    encoder
        .encode(&metric_families, &mut buffer)
        .map_err(|e| format!("failed to encode metrics: {e}"))?;

    String::from_utf8(buffer).map_err(|e| format!("failed to convert metrics to UTF-8: {e}"))
}
```

## Calls & References
- [[p2p_messages|p2p::messages]]
- [[kyc_requests|kyc::requests]]
- [[registry|registry]]
- [[blockchain_retries|blockchain::retries]]

## Called By
- [[test_metrics_registration_and_gathering|test::metrics_registration_and_gathering]]
- [[metrics_handler|metrics::handler]]


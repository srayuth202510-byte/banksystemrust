---
type: function
module: "metrics.rs"
parent: ""
tags: [rust, function]
---

# Function: registry

**Defined in:** [metrics.rs](file:///home/lokis/Documents/banksystemrust/src/metrics.rs#L15)

## Signature
```rust
pub fn registry() -> &'static Registry
```

## Implementation
```rust
pub fn registry() -> &'static Registry {
    static REGISTRY: OnceLock<Registry> = OnceLock::new();
    REGISTRY.get_or_init(Registry::new)
}
```

## Called By
- [[kyc_requests|kyc::requests]]
- [[p2p_messages|p2p::messages]]
- [[blockchain_retries|blockchain::retries]]
- [[gather_metrics|gather::metrics]]


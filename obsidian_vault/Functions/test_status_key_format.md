---
type: function
module: "redis_cache.rs"
parent: ""
tags: [rust, function]
---

# Function: test_status_key_format

**Defined in:** [redis_cache.rs](file:///home/lokis/Documents/banksystemrust/src/redis_cache.rs#L249)

## Signature
```rust
fn test_status_key_format()
```

## Implementation
```rust
fn test_status_key_format() {
        assert_eq!(
            transaction_status_key("tx-123"),
            "ndid:tx_status:tx-123".to_string()
        );
    }
```

## Calls & References
- [[transaction_status_key|transaction::status_key]]


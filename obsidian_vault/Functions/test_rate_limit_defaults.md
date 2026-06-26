---
type: function
module: "config.rs"
parent: ""
tags: [rust, function]
---

# Function: test_rate_limit_defaults

**Defined in:** [config.rs](file:///home/lokis/Documents/banksystemrust/src/config.rs#L403)

## Signature
```rust
fn test_rate_limit_defaults()
```

## Implementation
```rust
fn test_rate_limit_defaults() {
        let rl = RateLimitConfig::default();
        assert_eq!(rl.requests_per_second, 1000);
        assert_eq!(rl.burst, 2000);
        assert_eq!(rl.per_ip_limit, 100);
    }
```

## Calls & References
- [[RateLimitConfig|RateLimitConfig]]
- [[RateLimitConfig_default|RateLimitConfig::default]]


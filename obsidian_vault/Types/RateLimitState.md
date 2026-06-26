---
type: struct
module: "main.rs"
tags: [rust, type/struct]
---

# Struct: RateLimitState

**Defined in:** [main.rs](file:///home/lokis/Documents/banksystemrust/src/main.rs#L56)

## Definition
```rust
struct RateLimitState {
    redis: std::sync::Arc<RedisCache>,
    fallback: std::sync::Arc<TokioMutex<std::collections::HashMap<std::net::IpAddr, u64>>>,
    limit: u64,
}
```

## References
- [[RedisCache|RedisCache]]

## Used By
- [[per_ip_rate_limiter|per::ip_rate_limiter]]
- [[main|main]]


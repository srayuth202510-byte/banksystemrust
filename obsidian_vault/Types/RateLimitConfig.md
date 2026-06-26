---
type: struct
module: "config.rs"
tags: [rust, type/struct]
---

# Struct: RateLimitConfig

**Defined in:** [config.rs](file:///home/lokis/Documents/banksystemrust/src/config.rs#L24)

## Definition
```rust
pub struct RateLimitConfig {
    pub requests_per_second: u64,
    pub burst: usize,
    pub per_ip_limit: u64,
}
```

## Associated Functions & Methods
- [[RateLimitConfig_default|RateLimitConfig::default]]

## Used By
- [[ServerConfig|ServerConfig]]
- [[AppConfig_default|AppConfig::default]]
- [[test_rate_limit_defaults|test::rate_limit_defaults]]


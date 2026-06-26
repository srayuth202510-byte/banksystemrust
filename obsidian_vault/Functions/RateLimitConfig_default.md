---
type: function
module: "config.rs"
parent: "RateLimitConfig"
tags: [rust, function]
---

# Function: RateLimitConfig::default

**Defined in:** [config.rs](file:///home/lokis/Documents/banksystemrust/src/config.rs#L31)
**Impl Block:** [[RateLimitConfig]]

## Signature
```rust
fn default() -> Self
```

## Implementation
```rust
fn default() -> Self {
        Self {
            requests_per_second: 1000,
            burst: 2000,
            per_ip_limit: 100,
        }
    }
```

## Called By
- [[AppConfig_default|AppConfig::default]]
- [[test_rate_limit_defaults|test::rate_limit_defaults]]


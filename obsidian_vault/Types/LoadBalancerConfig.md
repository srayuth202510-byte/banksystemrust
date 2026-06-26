---
type: struct
module: "config.rs"
tags: [rust, type/struct]
---

# Struct: LoadBalancerConfig

**Defined in:** [config.rs](file:///home/lokis/Documents/banksystemrust/src/config.rs#L50)

## Definition
```rust
pub struct LoadBalancerConfig {
    #[serde(default)]
    pub strategy: LoadBalancerStrategy,
}
```

## References
- [[LoadBalancerStrategy|LoadBalancerStrategy]]

## Used By
- [[NetworkConfig|NetworkConfig]]
- [[AppConfig_default|AppConfig::default]]


---
type: struct
module: "config.rs"
tags: [rust, type/struct]
---

# Struct: NetworkConfig

**Defined in:** [config.rs](file:///home/lokis/Documents/banksystemrust/src/config.rs#L72)

## Definition
```rust
pub struct NetworkConfig {
    pub quic_port: u16,
    pub tcp_port: u16,
    pub quic_timeout_ms: u64,
    #[serde(default = "default_tcp_timeout")]
    pub tcp_timeout_ms: u64,
    pub fallback_enabled: bool,
    pub peers: Vec<String>,
    #[serde(default)]
    pub load_balancer: LoadBalancerConfig,
    #[serde(default)]
    pub cert_path: Option<String>,
    #[serde(default)]
    pub key_path: Option<String>,
    #[serde(default)]
    pub ca_cert_path: Option<String>,
}
```

## References
- [[LoadBalancerConfig|LoadBalancerConfig]]

## Used By
- [[AppConfig|AppConfig]]
- [[AppConfig_default|AppConfig::default]]


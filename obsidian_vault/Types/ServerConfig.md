---
type: struct
module: "config.rs"
tags: [rust, type/struct]
---

# Struct: ServerConfig

**Defined in:** [config.rs](file:///home/lokis/Documents/banksystemrust/src/config.rs#L61)

## Definition
```rust
pub struct ServerConfig {
    pub host: String,
    pub port: u16,
    pub graphql_endpoint: String,
    pub graphql_playground: bool,
    #[serde(default)]
    pub rate_limit: RateLimitConfig,
}
```

## References
- [[RateLimitConfig|RateLimitConfig]]

## Used By
- [[AppConfig|AppConfig]]
- [[AppConfig_default|AppConfig::default]]
- [[TlsContext_to_quic_server_config|TlsContext::to_quic_server_config]]
- [[TlsContext_to_rustls_server_config|TlsContext::to_rustls_server_config]]
- [[start_quic_server|start::quic_server]]


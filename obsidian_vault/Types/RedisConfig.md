---
type: struct
module: "config.rs"
tags: [rust, type/struct]
---

# Struct: RedisConfig

**Defined in:** [config.rs](file:///home/lokis/Documents/banksystemrust/src/config.rs#L124)

## Definition
```rust
pub struct RedisConfig {
    #[serde(default)]
    pub enabled: bool,
    pub url: String,
    #[serde(default)]
    pub username: Option<String>,
    #[serde(default)]
    pub password_file: Option<PathBuf>,
    #[serde(default = "default_redis_ttl_secs")]
    pub ttl_secs: u64,
    #[serde(default = "default_redis_timeout_ms")]
    pub timeout_ms: u64,
}
```

## Associated Functions & Methods
- [[RedisConfig_default|RedisConfig::default]]

## Used By
- [[AppConfig|AppConfig]]
- [[AppConfig_default|AppConfig::default]]
- [[RedisCache|RedisCache]]
- [[RedisCache_new|RedisCache::new]]
- [[build_client_url|build::client_url]]
- [[test_disabled_cache|test::disabled_cache]]
- [[test_build_client_url_with_password_file|test::build_client_url_with_password_file]]


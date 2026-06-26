---
type: struct
module: "config.rs"
tags: [rust, type/struct]
---

# Struct: BlockchainConfig

**Defined in:** [config.rs](file:///home/lokis/Documents/banksystemrust/src/config.rs#L92)

## Definition
```rust
pub struct BlockchainConfig {
    pub endpoint: String,
    pub timeout_secs: u64,
    pub max_retries: u32,
    pub db_path: Option<String>,
}
```

## Used By
- [[AppConfig|AppConfig]]
- [[AppConfig_default|AppConfig::default]]
- [[BlockchainClient|BlockchainClient]]
- [[BlockchainClient_new|BlockchainClient::new]]
- [[test_config|test::config]]


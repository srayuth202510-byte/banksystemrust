---
type: enum
module: "config.rs"
tags: [rust, type/enum]
---

# Enum: AppConfigError

**Defined in:** [config.rs](file:///home/lokis/Documents/banksystemrust/src/config.rs#L15)

## Definition
```rust
pub enum AppConfigError {
    #[error("{0}")]
    Message(String),
    #[error("config error: {0}")]
    Config(#[from] config::ConfigError),
}
```

## Used By
- [[AppConfig_load|AppConfig::load]]


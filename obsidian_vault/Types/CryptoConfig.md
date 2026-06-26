---
type: struct
module: "config.rs"
tags: [rust, type/struct]
---

# Struct: CryptoConfig

**Defined in:** [config.rs](file:///home/lokis/Documents/banksystemrust/src/config.rs#L101)

## Definition
```rust
pub struct CryptoConfig {
    #[serde(default)]
    pub hsm_enabled: bool,
    #[serde(default)]
    pub hsm_library_path: Option<String>,
    #[serde(default)]
    pub hsm_slot: Option<u32>,
    #[serde(default)]
    pub hsm_pin_file: Option<PathBuf>,
    pub signing_algorithm: String,
    pub encryption_algorithm: String,
}
```

## Used By
- [[AppConfig|AppConfig]]
- [[AppConfig_default|AppConfig::default]]


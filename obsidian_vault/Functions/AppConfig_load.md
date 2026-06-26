---
type: function
module: "config.rs"
parent: "AppConfig"
tags: [rust, function]
---

# Function: AppConfig::load

**Defined in:** [config.rs](file:///home/lokis/Documents/banksystemrust/src/config.rs#L313)
**Impl Block:** [[AppConfig]]

## Signature
```rust
pub fn load(path: Option<PathBuf>) -> Result<Self, AppConfigError>
```

## Implementation
```rust
pub fn load(path: Option<PathBuf>) -> Result<Self, AppConfigError> {
        let cfg_path = path.unwrap_or_else(|| PathBuf::from("config/default.toml"));
        let settings = config::Config::builder()
            .add_source(config::File::from(cfg_path).required(false))
            .add_source(
                config::Environment::with_prefix("NDID")
                    .prefix_separator("_")
                    .separator("__"),
            )
            .build()?;
        let config: Self = settings.try_deserialize()?;
        config.validate().map_err(AppConfigError::Message)?;
        Ok(config)
    }
```

## Calls & References
- [[AppConfig_validate|AppConfig::validate]]
- [[AppConfigError|AppConfigError]]

## Called By
- [[main|main]]


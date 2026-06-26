---
type: function
module: "config.rs"
parent: ""
tags: [rust, function]
---

# Function: is_production_mode

**Defined in:** [config.rs](file:///home/lokis/Documents/banksystemrust/src/config.rs#L330)

## Signature
```rust
fn is_production_mode() -> bool
```

## Implementation
```rust
fn is_production_mode() -> bool {
    matches!(
        std::env::var("NDID_ENV"),
        Ok(value) if value.eq_ignore_ascii_case("production")
    )
}
```

## Called By
- [[AppConfig_validate|AppConfig::validate]]


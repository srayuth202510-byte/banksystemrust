---
type: struct
module: "main.rs"
tags: [rust, type/struct]
---

# Struct: Cli

**Defined in:** [main.rs](file:///home/lokis/Documents/banksystemrust/src/main.rs#L44)

## Definition
```rust
struct Cli {
    #[arg(short, long, default_value = "config/default.toml")]
    config: String,
}
```

## Used By
- [[main|main]]


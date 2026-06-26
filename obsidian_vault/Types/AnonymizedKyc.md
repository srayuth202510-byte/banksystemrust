---
type: struct
module: "identity.rs"
tags: [rust, type/struct]
---

# Struct: AnonymizedKyc

**Defined in:** [identity.rs](file:///home/lokis/Documents/banksystemrust/src/identity.rs#L79)

## Definition
```rust
pub struct AnonymizedKyc {
    pub identity_hash: String,
    pub bank_code: String,
    pub timestamp: i64,
}
```

## Used By
- [[KycData_anonymize|KycData::anonymize]]


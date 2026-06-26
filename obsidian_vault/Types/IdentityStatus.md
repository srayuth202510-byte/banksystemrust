---
type: enum
module: "identity.rs"
tags: [rust, type/enum]
---

# Enum: IdentityStatus

**Defined in:** [identity.rs](file:///home/lokis/Documents/banksystemrust/src/identity.rs#L31)

## Definition
```rust
pub enum IdentityStatus {
    Pending,
    Approved,
    Rejected,
    Revoked,
}
```

## Used By
- [[IdentityRecord|IdentityRecord]]
- [[create_identity_record|create::identity_record]]
- [[test_create_identity_record|test::create_identity_record]]


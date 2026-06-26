---
type: struct
module: "schema.rs"
tags: [rust, type/struct]
---

# Struct: IdentityStatusGql

**Defined in:** [schema.rs](file:///home/lokis/Documents/banksystemrust/src/schema.rs#L18)

## Definition
```rust
pub struct IdentityStatusGql {
    pub request_id: String,      // รหัสคำขอ
    pub status: String,          // สถานะ (Pending, Approved, Rejected)
    pub active_protocol: String, // โปรโตคอลที่ใช้งาน (QUIC/TCP)
}
```

## Used By
- [[QueryRoot|QueryRoot]]
- [[QueryRoot_verify_ndid_record|QueryRoot::verify_ndid_record]]
- [[QueryRoot_get_identity|QueryRoot::get_identity]]


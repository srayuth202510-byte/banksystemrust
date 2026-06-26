---
type: struct
module: "identity.rs"
tags: [rust, type/struct]
---

# Struct: IdentityRecord

**Defined in:** [identity.rs](file:///home/lokis/Documents/banksystemrust/src/identity.rs#L40)

## Definition
```rust
pub struct IdentityRecord {
    pub request_id: String,      // รหัสอ้างอิงคำขอ
    pub status: IdentityStatus,  // สถานะปัจจุบัน
    pub identity_hash: String,   // ค่าแฮช SHA-256 ของข้อมูล
    pub timestamp: i64,          // เวลาที่สร้าง (Unix timestamp)
    pub bank_code: String,       // รหัสธนาคารเจ้าของข้อมูล
    pub active_protocol: String, // โปรโตคอลที่ใช้ส่งข้อมูล (QUIC/TCP)
}
```

## References
- [[IdentityStatus|IdentityStatus]]

## Used By
- [[create_identity_record|create::identity_record]]


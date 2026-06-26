---
type: struct
module: "schema.rs"
tags: [rust, type/struct]
---

# Struct: KycResponse

**Defined in:** [schema.rs](file:///home/lokis/Documents/banksystemrust/src/schema.rs#L26)

## Definition
```rust
pub struct KycResponse {
    pub request_id: String,    // รหัสธุรกรรม
    pub identity_hash: String, // ค่าแฮชของข้อมูลประจำตัว
    pub bank_code: String,     // รหัสธนาคาร
    pub message: String,       // ข้อความสถานะ
}
```

## Used By
- [[submit_kyc|submit::kyc]]


---
type: function
module: "schema.rs"
parent: ""
tags: [rust, function]
---

# Function: verify_status_label

**Defined in:** [schema.rs](file:///home/lokis/Documents/banksystemrust/src/schema.rs#L266)

## Signature
```rust
fn verify_status_label(status: &crate::blockchain::TxStatus) -> String
```

## Implementation
```rust
fn verify_status_label(status: &crate::blockchain::TxStatus) -> String {
    match status {
        crate::blockchain::TxStatus::Finalized => "Approved".to_string(),
        crate::blockchain::TxStatus::Queued => "Queued".to_string(),
        crate::blockchain::TxStatus::Pending => "Pending".to_string(),
        crate::blockchain::TxStatus::Failed => "Rejected".to_string(),
    }
}
```

## Calls & References
- [[TxStatus|TxStatus]]

## Called By
- [[QueryRoot|QueryRoot]]
- [[QueryRoot_verify_ndid_record|QueryRoot::verify_ndid_record]]


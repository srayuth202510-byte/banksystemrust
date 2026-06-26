---
type: function
module: "schema.rs"
parent: ""
tags: [rust, function]
---

# Function: get_identity_status_label

**Defined in:** [schema.rs](file:///home/lokis/Documents/banksystemrust/src/schema.rs#L256)

## Signature
```rust
fn get_identity_status_label(status: &crate::blockchain::TxStatus) -> String
```

## Implementation
```rust
fn get_identity_status_label(status: &crate::blockchain::TxStatus) -> String {
    match status {
        crate::blockchain::TxStatus::Pending => "Pending".to_string(),
        crate::blockchain::TxStatus::Finalized => "Finalized".to_string(),
        crate::blockchain::TxStatus::Failed => "Failed".to_string(),
        crate::blockchain::TxStatus::Queued => "Queued".to_string(),
    }
}
```

## Calls & References
- [[TxStatus|TxStatus]]

## Called By
- [[QueryRoot|QueryRoot]]
- [[QueryRoot_get_identity|QueryRoot::get_identity]]


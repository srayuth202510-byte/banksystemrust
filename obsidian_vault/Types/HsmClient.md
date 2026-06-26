---
type: struct
module: "crypto.rs"
tags: [rust, type/struct]
---

# Struct: HsmClient

**Defined in:** [crypto.rs](file:///home/lokis/Documents/banksystemrust/src/crypto.rs#L186)

## Definition
```rust
pub struct HsmClient {
        ctx: Ctx,
        _slot: CK_SLOT_ID,
        session: CK_SESSION_HANDLE,
    }
```

## Associated Functions & Methods
- [[HsmClient_new|HsmClient::new]]
- [[HsmClient_sign_ed25519|HsmClient::sign_ed25519]]
- [[HsmClient_find_key|HsmClient::find_key]]
- [[HsmClient_drop|HsmClient::drop]]


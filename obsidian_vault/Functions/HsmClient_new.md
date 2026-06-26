---
type: function
module: "crypto.rs"
parent: "HsmClient"
tags: [rust, function]
---

# Function: HsmClient::new

**Defined in:** [crypto.rs](file:///home/lokis/Documents/banksystemrust/src/crypto.rs#L193)
**Impl Block:** [[HsmClient]]

## Signature
```rust
pub fn new(
            library_path: &str,
            pin: &str,
            slot_id: Option<CK_SLOT_ID>,
        ) -> Result<Self, CryptoError>
```

## Implementation
```rust
pub fn new(
            library_path: &str,
            pin: &str,
            slot_id: Option<CK_SLOT_ID>,
        ) -> Result<Self, CryptoError> {
            let mut ctx =
                Ctx::new(library_path).map_err(|e| CryptoError::HsmError(e.to_string()))?;
            ctx.initialize(None)
                .map_err(|e| CryptoError::HsmError(e.to_string()))?;

            let slots = ctx
                .get_slot_list(true)
                .map_err(|e| CryptoError::HsmError(e.to_string()))?;
            let slot = match slot_id {
                Some(id) => id,
                None => *slots
                    .first()
                    .ok_or_else(|| CryptoError::HsmError("no slots available".into()))?,
            };

            let session = ctx
                .open_session(slot, CKF_SERIAL_SESSION | CKF_RW_SESSION, None, None)
                .map_err(|e| CryptoError::HsmError(e.to_string()))?;
            ctx.login(session, CKU_USER, Some(pin))
                .map_err(|e| CryptoError::HsmError(e.to_string()))?;

            Ok(Self {
                ctx,
                _slot: slot,
                session,
            })
        }
```

## Calls & References
- [[CryptoError|CryptoError]]


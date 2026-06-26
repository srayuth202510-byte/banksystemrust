---
type: function
module: "crypto.rs"
parent: "HsmClient"
tags: [rust, function]
---

# Function: HsmClient::find_key

**Defined in:** [crypto.rs](file:///home/lokis/Documents/banksystemrust/src/crypto.rs#L246)
**Impl Block:** [[HsmClient]]

## Signature
```rust
fn find_key(
            &self,
            label: &str,
            class: CK_OBJECT_CLASS,
            key_type: CK_KEY_TYPE,
        ) -> Result<CK_OBJECT_HANDLE, CryptoError>
```

## Implementation
```rust
fn find_key(
            &self,
            label: &str,
            class: CK_OBJECT_CLASS,
            key_type: CK_KEY_TYPE,
        ) -> Result<CK_OBJECT_HANDLE, CryptoError> {
            let template = [
                CK_ATTRIBUTE::new(CKA_CLASS).with_ck_ulong(&class),
                CK_ATTRIBUTE::new(CKA_KEY_TYPE).with_ck_ulong(&key_type),
                CK_ATTRIBUTE::new(CKA_LABEL).with_string(label),
            ];

            self.ctx
                .find_objects_init(self.session, &template)
                .map_err(|e| CryptoError::HsmError(e.to_string()))?;

            let objects = self
                .ctx
                .find_objects(self.session, 1)
                .map_err(|e| CryptoError::HsmError(e.to_string()))?;

            self.ctx
                .find_objects_final(self.session)
                .map_err(|e| CryptoError::HsmError(e.to_string()))?;

            objects
                .first()
                .copied()
                .ok_or_else(|| CryptoError::HsmError(format!("key not found: {}", label)))
        }
```

## Calls & References
- [[CryptoError|CryptoError]]


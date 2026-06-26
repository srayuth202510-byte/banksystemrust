---
type: function
module: "crypto.rs"
parent: "HsmClient"
tags: [rust, function]
---

# Function: HsmClient::sign_ed25519

**Defined in:** [crypto.rs](file:///home/lokis/Documents/banksystemrust/src/crypto.rs#L226)
**Impl Block:** [[HsmClient]]

## Signature
```rust
pub fn sign_ed25519(&self, data: &[u8], key_label: &str) -> Result<Vec<u8>, CryptoError>
```

## Implementation
```rust
pub fn sign_ed25519(&self, data: &[u8], key_label: &str) -> Result<Vec<u8>, CryptoError> {
            let key = self.find_key(key_label, CKO_PRIVATE_KEY, CKK_EDDSA)?;
            let mech = CK_MECHANISM {
                mechanism: CKM_EDDSA,
                pParameter: std::ptr::null_mut(),
                ulParameterLen: 0,
            };

            self.ctx
                .sign_init(self.session, &mech, key)
                .map_err(|e| CryptoError::HsmError(e.to_string()))?;

            let signature = self
                .ctx
                .sign(self.session, data)
                .map_err(|e| CryptoError::HsmError(e.to_string()))?;

            Ok(signature)
        }
```

## Calls & References
- [[CryptoError|CryptoError]]
- [[sign|sign]]


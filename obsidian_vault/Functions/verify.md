---
type: function
module: "crypto.rs"
parent: ""
tags: [rust, function]
---

# Function: verify

**Defined in:** [crypto.rs](file:///home/lokis/Documents/banksystemrust/src/crypto.rs#L120)

## Signature
```rust
pub fn verify(signed: &SignedPayload) -> Result<bool, CryptoError>
```

## Implementation
```rust
pub fn verify(signed: &SignedPayload) -> Result<bool, CryptoError> {
    let verifying_key = VerifyingKey::from_bytes(
        signed
            .public_key
            .as_slice()
            .try_into()
            .map_err(|_| CryptoError::InvalidKey("invalid public key".into()))?,
    )?;
    let signature = Signature::from_slice(&signed.signature)
        .map_err(|e| CryptoError::VerificationFailed(e.to_string()))?;
    Ok(verifying_key.verify(&signed.payload, &signature).is_ok())
}
```

## Calls & References
- [[SignedPayload|SignedPayload]]
- [[CryptoError|CryptoError]]

## Called By
- [[test_sign_and_verify|test::sign_and_verify]]
- [[process_p2p_message|process::p2p_message]]


---
type: function
module: "crypto.rs"
parent: "KeyPair"
tags: [rust, function]
---

# Function: KeyPair::generate

**Defined in:** [crypto.rs](file:///home/lokis/Documents/banksystemrust/src/crypto.rs#L63)
**Impl Block:** [[KeyPair]]

## Signature
```rust
pub fn generate() -> Result<Self, CryptoError>
```

## Implementation
```rust
pub fn generate() -> Result<Self, CryptoError> {
        let mut secret_bytes = [0u8; 32];
        use rand::RngCore;
        OsRng.fill_bytes(&mut secret_bytes);
        let signing_key = SigningKey::from_bytes(&secret_bytes);
        secret_bytes.zeroize();
        let verifying_key = signing_key.verifying_key();
        Ok(Self {
            public_key: verifying_key.to_bytes().to_vec(),
            secret_key: signing_key.to_bytes().to_vec(),
        })
    }
```

## Calls & References
- [[CryptoError|CryptoError]]

## Called By
- [[test_node|test::node]]
- [[test_create_transaction|test::create_transaction]]
- [[test_submit_transaction_queued_on_no_node|test::submit_transaction_queued_on_no_node]]
- [[test_get_transaction_status|test::get_transaction_status]]
- [[test_keypair_generation|test::keypair_generation]]
- [[test_sign_and_verify|test::sign_and_verify]]
- [[main|main]]
- [[TlsContext_generate_self_signed|TlsContext::generate_self_signed]]


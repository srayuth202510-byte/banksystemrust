---
type: function
module: "crypto.rs"
parent: ""
tags: [rust, function]
---

# Function: sign

**Defined in:** [crypto.rs](file:///home/lokis/Documents/banksystemrust/src/crypto.rs#L103)

## Signature
```rust
pub fn sign(payload: &[u8], keypair: &KeyPair) -> Result<SignedPayload, CryptoError>
```

## Implementation
```rust
pub fn sign(payload: &[u8], keypair: &KeyPair) -> Result<SignedPayload, CryptoError> {
    let signing_key = SigningKey::from_bytes(
        keypair
            .secret_key
            .as_slice()
            .try_into()
            .map_err(|_| CryptoError::InvalidKey("invalid secret key".into()))?,
    );
    let signature = signing_key.sign(payload);
    Ok(SignedPayload {
        payload: payload.to_vec(),
        signature: signature.to_bytes().to_vec(),
        public_key: keypair.public_key.clone(),
    })
}
```

## Calls & References
- [[SignedPayload|SignedPayload]]
- [[CryptoError|CryptoError]]
- [[KeyPair|KeyPair]]
- [[KeyPair_from_bytes|KeyPair::from_bytes]]

## Called By
- [[P2pNode_send_kyc_inner|P2pNode::send_kyc_inner]]
- [[BlockchainClient_create_transaction|BlockchainClient::create_transaction]]
- [[HsmClient_sign_ed25519|HsmClient::sign_ed25519]]
- [[test_sign_and_verify|test::sign_and_verify]]


---
type: module
path: "crypto.rs"
tags: [rust, module]
---

# Module: crypto.rs

**File Link:** [crypto.rs](file:///home/lokis/Documents/banksystemrust/src/crypto.rs)

## Types Defined
- [[KeyPair]] (struct)
- [[SignedPayload]] (struct)
- [[EncryptedPayload]] (struct)
- [[HsmClient]] (struct)
- [[CryptoError]] (enum)

## Standalone Functions
- [[sign|sign]]
- [[verify|verify]]
- [[hash_hex|hash_hex]]
- [[test_keypair_generation|test_keypair_generation]]
- [[test_sign_and_verify|test_sign_and_verify]]
- [[test_hash_consistency|test_hash_consistency]]
- [[test_encrypt_decrypt|test_encrypt_decrypt]]
- [[test_hash_hex_format|test_hash_hex_format]]

## Implementation Methods
- [[From_from|From::from]] (impl for [[From]])
- [[KeyPair_drop|KeyPair::drop]] (impl for [[KeyPair]])
- [[KeyPair_generate|KeyPair::generate]] (impl for [[KeyPair]])
- [[KeyPair_from_bytes|KeyPair::from_bytes]] (impl for [[KeyPair]])
- [[HsmClient_new|HsmClient::new]] (impl for [[HsmClient]])
- [[HsmClient_sign_ed25519|HsmClient::sign_ed25519]] (impl for [[HsmClient]])
- [[HsmClient_find_key|HsmClient::find_key]] (impl for [[HsmClient]])
- [[HsmClient_drop|HsmClient::drop]] (impl for [[HsmClient]])


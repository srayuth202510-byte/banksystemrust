---
type: function
module: "crypto.rs"
parent: ""
tags: [rust, function]
---

# Function: test_encrypt_decrypt

**Defined in:** [crypto.rs](file:///home/lokis/Documents/banksystemrust/src/crypto.rs#L314)

## Signature
```rust
fn test_encrypt_decrypt()
```

## Implementation
```rust
fn test_encrypt_decrypt() {
        let key: [u8; 32] = rand::random();
        let plain = b"sensitive identity data";
        let encrypted = encrypt(plain, &key).unwrap();
        let decrypted = decrypt(&encrypted, &key).unwrap();
        assert_eq!(plain.to_vec(), decrypted);
    }
```


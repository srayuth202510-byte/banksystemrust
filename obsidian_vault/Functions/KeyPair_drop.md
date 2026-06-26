---
type: function
module: "crypto.rs"
parent: "KeyPair"
tags: [rust, function]
---

# Function: KeyPair::drop

**Defined in:** [crypto.rs](file:///home/lokis/Documents/banksystemrust/src/crypto.rs#L55)
**Impl Block:** [[KeyPair]]

## Signature
```rust
fn drop(&mut self)
```

## Implementation
```rust
fn drop(&mut self) {
        self.secret_key.zeroize();
        self.public_key.zeroize();
    }
```


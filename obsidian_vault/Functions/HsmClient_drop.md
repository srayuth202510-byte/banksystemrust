---
type: function
module: "crypto.rs"
parent: "HsmClient"
tags: [rust, function]
---

# Function: HsmClient::drop

**Defined in:** [crypto.rs](file:///home/lokis/Documents/banksystemrust/src/crypto.rs#L279)
**Impl Block:** [[HsmClient]]

## Signature
```rust
fn drop(&mut self)
```

## Implementation
```rust
fn drop(&mut self) {
            let _ = self.ctx.logout(self.session);
            let _ = self.ctx.close_session(self.session);
            let _ = self.ctx.finalize();
        }
```


---
type: function
module: "network/tls.rs"
parent: ""
tags: [rust, function]
---

# Function: test_generate_self_signed

**Defined in:** [network/tls.rs](file:///home/lokis/Documents/banksystemrust/src/network/tls.rs#L224)

## Signature
```rust
fn test_generate_self_signed()
```

## Implementation
```rust
fn test_generate_self_signed() {
        let ctx = TlsContext::generate_self_signed().unwrap();
        assert!(!ctx.certs.is_empty());
    }
```

## Calls & References
- [[TlsContext_generate_self_signed|TlsContext::generate_self_signed]]
- [[TlsContext|TlsContext]]

## Called By
- [[test_node|test::node]]
- [[test_quic_configs|test::quic_configs]]
- [[test_fallback_on_unreachable|test::fallback_on_unreachable]]


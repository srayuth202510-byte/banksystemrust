---
type: function
module: "network/mod.rs"
parent: ""
tags: [rust, function]
---

# Function: test_fallback_on_unreachable

**Defined in:** [network/mod.rs](file:///home/lokis/Documents/banksystemrust/src/network/mod.rs#L292)

## Signature
```rust
async fn test_fallback_on_unreachable()
```

## Implementation
```rust
async fn test_fallback_on_unreachable() {
        let tls = tls::TlsContext::generate_self_signed().unwrap();
        let (_channel, proto) = connect_with_fallback("127.0.0.1:19999", &tls, 500, 2000).await;
        assert_eq!(proto, Protocol::Tcp);
    }
```

## Calls & References
- [[TlsContext|TlsContext]]
- [[TlsContext_generate_self_signed|TlsContext::generate_self_signed]]
- [[Protocol|Protocol]]
- [[test_generate_self_signed|test::generate_self_signed]]
- [[connect_with_fallback|connect::with_fallback]]


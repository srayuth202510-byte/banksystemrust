---
type: function
module: "network/tls.rs"
parent: ""
tags: [rust, function]
---

# Function: test_quic_configs

**Defined in:** [network/tls.rs](file:///home/lokis/Documents/banksystemrust/src/network/tls.rs#L230)

## Signature
```rust
fn test_quic_configs()
```

## Implementation
```rust
fn test_quic_configs() {
        let ctx = TlsContext::generate_self_signed().unwrap();
        assert!(ctx.to_quic_server_config().is_ok());
        assert!(ctx.to_quic_client_config().is_ok());
        assert!(ctx.to_rustls_client_config().is_ok());
    }
```

## Calls & References
- [[TlsContext|TlsContext]]
- [[TlsContext_to_rustls_client_config|TlsContext::to_rustls_client_config]]
- [[test_config|test::config]]
- [[TlsContext_to_quic_client_config|TlsContext::to_quic_client_config]]
- [[TlsContext_generate_self_signed|TlsContext::generate_self_signed]]
- [[test_generate_self_signed|test::generate_self_signed]]
- [[TlsContext_to_quic_server_config|TlsContext::to_quic_server_config]]


---
type: enum
module: "network/tls.rs"
tags: [rust, type/enum]
---

# Enum: TlsError

**Defined in:** [network/tls.rs](file:///home/lokis/Documents/banksystemrust/src/network/tls.rs#L15)

## Definition
```rust
pub enum TlsError {
    #[error("certificate generation failed: {0}")]
    CertGeneration(String),
    #[error("certificate loading failed: {0}")]
    CertLoading(String),
    #[error("invalid key: {0}")]
    InvalidKey(String),
}
```

## Used By
- [[P2pError|P2pError]]
- [[P2pNode_send_kyc_inner|P2pNode::send_kyc_inner]]
- [[TlsContext_generate_self_signed|TlsContext::generate_self_signed]]
- [[TlsContext_load|TlsContext::load]]
- [[TlsContext_add_ca_cert|TlsContext::add_ca_cert]]
- [[TlsContext_to_quic_server_config|TlsContext::to_quic_server_config]]
- [[TlsContext_to_quic_client_config|TlsContext::to_quic_client_config]]
- [[TlsContext_to_rustls_client_config|TlsContext::to_rustls_client_config]]
- [[TlsContext_to_rustls_server_config|TlsContext::to_rustls_server_config]]
- [[load_certs|load::certs]]
- [[load_key|load::key]]
- [[connect_tcp_tls|connect::tcp_tls]]
- [[start_tcp_server|start::tcp_server]]
- [[NetworkError|NetworkError]]


---
type: struct
module: "network/tls.rs"
tags: [rust, type/struct]
---

# Struct: TlsContext

**Defined in:** [network/tls.rs](file:///home/lokis/Documents/banksystemrust/src/network/tls.rs#L25)

## Definition
```rust
pub struct TlsContext {
    pub certs: Vec<CertificateDer<'static>>, // ใบรับรองของเซิร์ฟเวอร์
    key: PrivateKeyDer<'static>,             // กุญแจส่วนตัว (ไม่เปิดเผย)
    pub ca_certs: Vec<CertificateDer<'static>>, // ใบรับรอง CA สำหรับตรวจสอบ
}

impl Clone for TlsContext {
    fn clone(&self) -> Self {
        Self {
            certs: self.certs.clone(),
            key: self.key.clone_key(),
            ca_certs: self.ca_certs.clone(),
        }
    }
}

impl std::fmt::Debug for TlsContext {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("TlsContext")
            .field("cert_count", &self.certs.len())
            .field("ca_count", &self.ca_certs.len())
            .finish()
    }
}
```

## Associated Functions & Methods
- [[TlsContext_clone|TlsContext::clone]]
- [[TlsContext_generate_self_signed|TlsContext::generate_self_signed]]
- [[TlsContext_load|TlsContext::load]]
- [[TlsContext_add_ca_cert|TlsContext::add_ca_cert]]
- [[TlsContext_to_quic_server_config|TlsContext::to_quic_server_config]]
- [[TlsContext_to_quic_client_config|TlsContext::to_quic_client_config]]
- [[TlsContext_to_rustls_client_config|TlsContext::to_rustls_client_config]]
- [[TlsContext_to_rustls_server_config|TlsContext::to_rustls_server_config]]

## References
- [[TlsContext_clone|TlsContext::clone]]
- [[fmt|fmt]]

## Used By
- [[P2pNode|P2pNode]]
- [[P2pNode_new|P2pNode::new]]
- [[test_node|test::node]]
- [[start_quic_server|start::quic_server]]
- [[main|main]]
- [[fmt|fmt]]
- [[test_generate_self_signed|test::generate_self_signed]]
- [[test_quic_configs|test::quic_configs]]
- [[connect_tcp_tls|connect::tcp_tls]]
- [[start_tcp_server|start::tcp_server]]
- [[ConnectionChannel|ConnectionChannel]]
- [[NetworkChannel_connect|NetworkChannel::connect]]
- [[connect_with_fallback|connect::with_fallback]]
- [[fallback_to_tcp|fallback::to_tcp]]
- [[test_fallback_on_unreachable|test::fallback_on_unreachable]]


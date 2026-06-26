---
type: module
path: "network/tls.rs"
tags: [rust, module]
---

# Module: network/tls.rs

**File Link:** [network/tls.rs](file:///home/lokis/Documents/banksystemrust/src/network/tls.rs)

## Types Defined
- [[TlsContext]] (struct)
- [[TlsError]] (enum)

## Standalone Functions
- [[fmt|fmt]]
- [[load_certs|load_certs]]
- [[load_key|load_key]]
- [[test_generate_self_signed|test_generate_self_signed]]
- [[test_quic_configs|test_quic_configs]]

## Implementation Methods
- [[TlsContext_clone|TlsContext::clone]] (impl for [[TlsContext]])
- [[TlsContext_generate_self_signed|TlsContext::generate_self_signed]] (impl for [[TlsContext]])
- [[TlsContext_load|TlsContext::load]] (impl for [[TlsContext]])
- [[TlsContext_add_ca_cert|TlsContext::add_ca_cert]] (impl for [[TlsContext]])
- [[TlsContext_to_quic_server_config|TlsContext::to_quic_server_config]] (impl for [[TlsContext]])
- [[TlsContext_to_quic_client_config|TlsContext::to_quic_client_config]] (impl for [[TlsContext]])
- [[TlsContext_to_rustls_client_config|TlsContext::to_rustls_client_config]] (impl for [[TlsContext]])
- [[TlsContext_to_rustls_server_config|TlsContext::to_rustls_server_config]] (impl for [[TlsContext]])


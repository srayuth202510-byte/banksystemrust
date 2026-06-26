---
type: error_variant
enum: "NetworkError"
variant: "TlsError"
tags: [rust, error, error/NetworkError]
---

# Error: NetworkError::TlsError

**Defined in:** [network/mod.rs](file:///home/lokis/Documents/banksystemrust/src/network/mod.rs)

## Log Pattern / Error Message
`tls error: {0}`

## Functions Generating/Handling this Error
- [[TlsContext_to_rustls_server_config|TlsContext::to_rustls_server_config]]
- [[load_certs|load::certs]]
- [[TlsContext_add_ca_cert|TlsContext::add_ca_cert]]
- [[P2pNode_send_kyc_inner|P2pNode::send_kyc_inner]]
- [[connect_tcp_tls|connect::tcp_tls]]
- [[TlsContext_generate_self_signed|TlsContext::generate_self_signed]]
- [[TlsContext_to_quic_server_config|TlsContext::to_quic_server_config]]
- [[start_tcp_server|start::tcp_server]]
- [[TlsContext_load|TlsContext::load]]
- [[load_key|load::key]]
- [[TlsContext_to_rustls_client_config|TlsContext::to_rustls_client_config]]
- [[TlsContext_to_quic_client_config|TlsContext::to_quic_client_config]]

## Troubleshooting Guide
### Possible Causes
1. Describe the trigger condition here.
2. Check input values or context variables.

### Verification Steps
1. Search logs for this pattern.
2. Inspect the parameters passed to [[TlsContext_to_rustls_server_config]].

### Remediation
- How to resolve the error in runtime or code.

---
type: error_variant
enum: "TlsError"
variant: "CertLoading"
tags: [rust, error, error/TlsError]
---

# Error: TlsError::CertLoading

**Defined in:** [network/tls.rs](file:///home/lokis/Documents/banksystemrust/src/network/tls.rs)

## Log Pattern / Error Message
`certificate loading failed: {0}`

## Functions Generating/Handling this Error
- [[TlsContext_to_rustls_server_config|TlsContext::to_rustls_server_config]]
- [[load_certs|load::certs]]
- [[TlsContext_to_quic_server_config|TlsContext::to_quic_server_config]]
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

---
type: function
module: "network/tls.rs"
parent: "TlsContext"
tags: [rust, function]
---

# Function: TlsContext::to_rustls_client_config

**Defined in:** [network/tls.rs](file:///home/lokis/Documents/banksystemrust/src/network/tls.rs#L154)
**Impl Block:** [[TlsContext]]

## Signature
```rust
pub fn to_rustls_client_config(&self) -> Result<rustls::ClientConfig, TlsError>
```

## Implementation
```rust
pub fn to_rustls_client_config(&self) -> Result<rustls::ClientConfig, TlsError> {
        let provider = rustls::crypto::ring::default_provider();

        let mut roots = rustls::RootCertStore::empty();
        for ca in &self.ca_certs {
            roots
                .add(ca.clone())
                .map_err(|e| TlsError::CertLoading(e.to_string()))?;
        }
        let crypto = rustls::ClientConfig::builder_with_provider(provider.into())
            .with_protocol_versions(&[&rustls::version::TLS13])
            .map_err(|e| TlsError::CertLoading(e.to_string()))?
            .with_root_certificates(roots)
            .with_no_client_auth();
        Ok(crypto)
    }
```

## Calls & References
- [[TlsError|TlsError]]

## Called By
- [[test_quic_configs|test::quic_configs]]
- [[connect_tcp_tls|connect::tcp_tls]]


---
type: function
module: "network/tls.rs"
parent: "TlsContext"
tags: [rust, function]
---

# Function: TlsContext::to_rustls_server_config

**Defined in:** [network/tls.rs](file:///home/lokis/Documents/banksystemrust/src/network/tls.rs#L172)
**Impl Block:** [[TlsContext]]

## Signature
```rust
pub fn to_rustls_server_config(&self) -> Result<rustls::ServerConfig, TlsError>
```

## Implementation
```rust
pub fn to_rustls_server_config(&self) -> Result<rustls::ServerConfig, TlsError> {
        let verifier = if self.ca_certs.is_empty() {
            rustls::server::WebPkiClientVerifier::no_client_auth()
        } else {
            let mut roots = rustls::RootCertStore::empty();
            for ca in &self.ca_certs {
                roots
                    .add(ca.clone())
                    .map_err(|e| TlsError::CertLoading(e.to_string()))?;
            }
            rustls::server::WebPkiClientVerifier::builder_with_provider(
                Arc::new(roots),
                rustls::crypto::ring::default_provider().into(),
            )
            .build()
            .map_err(|e| TlsError::CertLoading(e.to_string()))?
        };

        rustls::ServerConfig::builder_with_provider(rustls::crypto::ring::default_provider().into())
            .with_protocol_versions(&[&rustls::version::TLS13])
            .map_err(|e| TlsError::CertLoading(e.to_string()))?
            .with_client_cert_verifier(verifier)
            .with_single_cert(self.certs.clone(), self.key.clone_key())
            .map_err(|e| TlsError::CertLoading(e.to_string()))
    }
```

## Calls & References
- [[TlsError|TlsError]]
- [[ServerConfig|ServerConfig]]

## Called By
- [[start_tcp_server|start::tcp_server]]


---
type: function
module: "network/tls.rs"
parent: "TlsContext"
tags: [rust, function]
---

# Function: TlsContext::to_quic_server_config

**Defined in:** [network/tls.rs](file:///home/lokis/Documents/banksystemrust/src/network/tls.rs#L92)
**Impl Block:** [[TlsContext]]

## Signature
```rust
pub fn to_quic_server_config(&self) -> Result<quinn::ServerConfig, TlsError>
```

## Implementation
```rust
pub fn to_quic_server_config(&self) -> Result<quinn::ServerConfig, TlsError> {
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

        let crypto = rustls::ServerConfig::builder_with_provider(
            rustls::crypto::ring::default_provider().into(),
        )
        .with_protocol_versions(&[&rustls::version::TLS13])
        .map_err(|e| TlsError::CertLoading(e.to_string()))?
        .with_client_cert_verifier(verifier)
        .with_single_cert(self.certs.clone(), self.key.clone_key())
        .map_err(|e| TlsError::CertLoading(e.to_string()))?;

        let quic_config = quinn::crypto::rustls::QuicServerConfig::try_from(crypto)
            .map_err(|e| TlsError::CertLoading(e.to_string()))?;
        let mut config = quinn::ServerConfig::with_crypto(Arc::new(quic_config));
        let mut transport = quinn::TransportConfig::default();
        transport.max_concurrent_uni_streams(0u32.into());
        config.transport_config(Arc::new(transport));
        Ok(config)
    }
```

## Calls & References
- [[TlsError|TlsError]]
- [[ServerConfig|ServerConfig]]

## Called By
- [[start_quic_server|start::quic_server]]
- [[test_quic_configs|test::quic_configs]]


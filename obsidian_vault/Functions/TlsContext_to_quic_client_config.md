---
type: function
module: "network/tls.rs"
parent: "TlsContext"
tags: [rust, function]
---

# Function: TlsContext::to_quic_client_config

**Defined in:** [network/tls.rs](file:///home/lokis/Documents/banksystemrust/src/network/tls.rs#L129)
**Impl Block:** [[TlsContext]]

## Signature
```rust
pub fn to_quic_client_config(&self) -> Result<quinn::ClientConfig, TlsError>
```

## Implementation
```rust
pub fn to_quic_client_config(&self) -> Result<quinn::ClientConfig, TlsError> {
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

        let quic_config = quinn::crypto::rustls::QuicClientConfig::try_from(crypto)
            .map_err(|e| TlsError::CertLoading(e.to_string()))?;
        let mut config = quinn::ClientConfig::new(Arc::new(quic_config));
        let mut transport = quinn::TransportConfig::default();
        transport.max_concurrent_uni_streams(0u32.into());
        config.transport_config(Arc::new(transport));
        Ok(config)
    }
```

## Calls & References
- [[TlsError|TlsError]]

## Called By
- [[test_quic_configs|test::quic_configs]]
- [[connect_with_fallback|connect::with_fallback]]


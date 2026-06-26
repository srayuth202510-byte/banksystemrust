---
type: function
module: "network/tls.rs"
parent: "TlsContext"
tags: [rust, function]
---

# Function: TlsContext::generate_self_signed

**Defined in:** [network/tls.rs](file:///home/lokis/Documents/banksystemrust/src/network/tls.rs#L52)
**Impl Block:** [[TlsContext]]

## Signature
```rust
pub fn generate_self_signed() -> Result<Self, TlsError>
```

## Implementation
```rust
pub fn generate_self_signed() -> Result<Self, TlsError> {
        let key_pair =
            rcgen::KeyPair::generate().map_err(|e| TlsError::CertGeneration(e.to_string()))?;
        let params = rcgen::CertificateParams::new(vec![
            "localhost".into(),
            "ndid.local".into(),
            "127.0.0.1".into(),
        ])
        .map_err(|e| TlsError::CertGeneration(e.to_string()))?;
        let cert = params
            .self_signed(&key_pair)
            .map_err(|e| TlsError::CertGeneration(e.to_string()))?;

        let cert_der = CertificateDer::from(cert.der().to_vec());
        Ok(Self {
            certs: vec![cert_der.clone()],
            key: PrivateKeyDer::Pkcs8(PrivatePkcs8KeyDer::from(key_pair.serialize_der())),
            ca_certs: vec![cert_der],
        })
    }
```

## Calls & References
- [[TlsError|TlsError]]
- [[KeyPair|KeyPair]]
- [[KeyPair_generate|KeyPair::generate]]

## Called By
- [[test_node|test::node]]
- [[main|main]]
- [[test_generate_self_signed|test::generate_self_signed]]
- [[test_quic_configs|test::quic_configs]]
- [[test_fallback_on_unreachable|test::fallback_on_unreachable]]


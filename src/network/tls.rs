use std::sync::Arc;
use rustls::pki_types::{CertificateDer, PrivateKeyDer, PrivatePkcs8KeyDer, ServerName, UnixTime};
use rustls::SignatureScheme;
use thiserror::Error;

#[derive(Debug, Error)]
pub enum TlsError {
    #[error("certificate generation failed: {0}")]
    CertGeneration(String),
    #[error("certificate loading failed: {0}")]
    CertLoading(String),
    #[error("invalid key: {0}")]
    InvalidKey(String),
}

pub struct TlsContext {
    pub certs: Vec<CertificateDer<'static>>,
    key: PrivateKeyDer<'static>,
    pub ca_certs: Vec<CertificateDer<'static>>,
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

impl TlsContext {
    pub fn generate_self_signed() -> Result<Self, TlsError> {
        let key_pair = rcgen::KeyPair::generate()
            .map_err(|e| TlsError::CertGeneration(e.to_string()))?;
        let params = rcgen::CertificateParams::new(
            vec!["localhost".into(), "ndid.local".into()]
        )
        .map_err(|e| TlsError::CertGeneration(e.to_string()))?;
        let cert = params.self_signed(&key_pair)
            .map_err(|e| TlsError::CertGeneration(e.to_string()))?;

        let cert_der = CertificateDer::from(cert.der().to_vec());
        Ok(Self {
            certs: vec![cert_der.clone()],
            key: PrivateKeyDer::Pkcs8(
                PrivatePkcs8KeyDer::from(key_pair.serialize_der())
            ),
            ca_certs: vec![cert_der],
        })
    }

    pub fn load(cert_path: &str, key_path: &str) -> Result<Self, TlsError> {
        let certs = load_certs(cert_path)?;
        let key = load_key(key_path)?;
        Ok(Self {
            certs,
            key,
            ca_certs: Vec::new(),
        })
    }

    pub fn add_ca_cert(&mut self, path: &str) -> Result<(), TlsError> {
        let certs = load_certs(path)?;
        self.ca_certs.extend(certs);
        Ok(())
    }

    pub fn to_quic_server_config(&self) -> Result<quinn::ServerConfig, TlsError> {
        let verifier = if self.ca_certs.is_empty() {
            rustls::server::WebPkiClientVerifier::no_client_auth()
        } else {
            let mut roots = rustls::RootCertStore::empty();
            for ca in &self.ca_certs {
                roots.add(ca.clone())
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
            rustls::crypto::ring::default_provider().into()
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

    pub fn to_quic_client_config(&self, skip_verify: bool) -> Result<quinn::ClientConfig, TlsError> {
        let provider = rustls::crypto::ring::default_provider();

        let crypto: rustls::ClientConfig = if skip_verify {
            rustls::ClientConfig::builder_with_provider(provider.into())
                .with_protocol_versions(&[&rustls::version::TLS13])
                .map_err(|e| TlsError::CertLoading(e.to_string()))?
                .dangerous()
                .with_custom_certificate_verifier(Arc::new(SkipCertVerifier))
                .with_no_client_auth()
        } else {
            let mut roots = rustls::RootCertStore::empty();
            for ca in &self.ca_certs {
                roots.add(ca.clone())
                    .map_err(|e| TlsError::CertLoading(e.to_string()))?;
            }
            rustls::ClientConfig::builder_with_provider(provider.into())
                .with_protocol_versions(&[&rustls::version::TLS13])
                .map_err(|e| TlsError::CertLoading(e.to_string()))?
                .with_root_certificates(roots)
                .with_no_client_auth()
        };

        let quic_config = quinn::crypto::rustls::QuicClientConfig::try_from(crypto)
            .map_err(|e| TlsError::CertLoading(e.to_string()))?;
        let mut config = quinn::ClientConfig::new(Arc::new(quic_config));
        let mut transport = quinn::TransportConfig::default();
        transport.max_concurrent_uni_streams(0u32.into());
        config.transport_config(Arc::new(transport));
        Ok(config)
    }

    pub fn to_rustls_server_config(&self) -> Result<rustls::ServerConfig, TlsError> {
        let verifier = if self.ca_certs.is_empty() {
            rustls::server::WebPkiClientVerifier::no_client_auth()
        } else {
            let mut roots = rustls::RootCertStore::empty();
            for ca in &self.ca_certs {
                roots.add(ca.clone())
                    .map_err(|e| TlsError::CertLoading(e.to_string()))?;
            }
            rustls::server::WebPkiClientVerifier::builder_with_provider(
                Arc::new(roots),
                rustls::crypto::ring::default_provider().into(),
            )
                .build()
                .map_err(|e| TlsError::CertLoading(e.to_string()))?
        };

        rustls::ServerConfig::builder_with_provider(
            rustls::crypto::ring::default_provider().into()
        )
        .with_protocol_versions(&[&rustls::version::TLS13])
        .map_err(|e| TlsError::CertLoading(e.to_string()))?
        .with_client_cert_verifier(verifier)
        .with_single_cert(self.certs.clone(), self.key.clone_key())
        .map_err(|e| TlsError::CertLoading(e.to_string()))
    }
}

fn load_certs(path: &str) -> Result<Vec<CertificateDer<'static>>, TlsError> {
    let bytes = std::fs::read(path)
        .map_err(|e| TlsError::CertLoading(format!("cannot read {path}: {e}")))?;
    let certs = rustls_pemfile::certs(&mut bytes.as_slice())
        .collect::<Result<Vec<_>, _>>()
        .map_err(|e| TlsError::CertLoading(e.to_string()))?;
    Ok(certs)
}

fn load_key(path: &str) -> Result<PrivateKeyDer<'static>, TlsError> {
    let bytes = std::fs::read(path)
        .map_err(|e| TlsError::CertLoading(format!("cannot read {path}: {e}")))?;
    let mut reader = &bytes[..];
    rustls_pemfile::private_key(&mut reader)
        .map_err(|e| TlsError::CertLoading(e.to_string()))?
        .ok_or_else(|| TlsError::CertLoading("no private key found".into()))
}

#[derive(Debug)]
pub(crate) struct SkipCertVerifier;

impl rustls::client::danger::ServerCertVerifier for SkipCertVerifier {
    fn verify_server_cert(
        &self,
        _end_entity: &CertificateDer<'_>,
        _intermediates: &[CertificateDer<'_>],
        _server_name: &ServerName<'_>,
        _ocsp: &[u8],
        _now: UnixTime,
    ) -> Result<rustls::client::danger::ServerCertVerified, rustls::Error> {
        Ok(rustls::client::danger::ServerCertVerified::assertion())
    }

    fn verify_tls12_signature(
        &self,
        _message: &[u8],
        _cert: &CertificateDer<'_>,
        _dss: &rustls::DigitallySignedStruct,
    ) -> Result<rustls::client::danger::HandshakeSignatureValid, rustls::Error> {
        Ok(rustls::client::danger::HandshakeSignatureValid::assertion())
    }

    fn verify_tls13_signature(
        &self,
        _message: &[u8],
        _cert: &CertificateDer<'_>,
        _dss: &rustls::DigitallySignedStruct,
    ) -> Result<rustls::client::danger::HandshakeSignatureValid, rustls::Error> {
        Ok(rustls::client::danger::HandshakeSignatureValid::assertion())
    }

    fn supported_verify_schemes(&self) -> Vec<SignatureScheme> {
        vec![
            SignatureScheme::RSA_PKCS1_SHA256,
            SignatureScheme::ECDSA_NISTP256_SHA256,
            SignatureScheme::ED25519,
        ]
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_generate_self_signed() {
        let ctx = TlsContext::generate_self_signed().unwrap();
        assert!(!ctx.certs.is_empty());
    }

    #[test]
    fn test_quic_configs() {
        let ctx = TlsContext::generate_self_signed().unwrap();
        assert!(ctx.to_quic_server_config().is_ok());
        assert!(ctx.to_quic_client_config(true).is_ok());
    }
}

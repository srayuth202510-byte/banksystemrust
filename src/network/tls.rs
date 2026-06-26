// ระบบนี้คือ High-Speed Blockchain Banking System สำหรับ NDID (National Digital ID)
// ระบบพัฒนาขึ้นเพื่อรองรับการทำธุรกรรมธนาคารข้ามประเทศอย่างรวดเร็ว
// ภาษา: Rust, รันไทม์: Tokio async, โปรโตคอล: QUIC + TCP/TLS 1.3 Auto-Fallback
// ชั้นบริการ API: GraphQL (async-graphql) over Axum
// บล็อกเชน: Substrate (Private Permissioned Ledger)
// คริปโต: ED25519 (signing), AES-GCM (encryption), SHA-256 (hashing)

// การจัดการ TLS (certificate, การเข้ารหัส, การกำหนดค่า QUIC + rustls)
use rustls::pki_types::{CertificateDer, PrivateKeyDer, PrivatePkcs8KeyDer};
use sha2::{Digest, Sha256};
use std::sync::Arc;
use subtle::ConstantTimeEq;
use thiserror::Error;
use zeroize::Zeroize;

// ข้อผิดพลาดเกี่ยวกับการจัดการ TLS Certificate
#[derive(Debug, Error)]
pub enum TlsError {
    #[error("certificate generation failed: {0}")]
    CertGeneration(String),
    #[error("certificate loading failed: {0}")]
    CertLoading(String),
    #[error("invalid key: {0}")]
    InvalidKey(String),
}

// บริบท TLS สำหรับจัดการใบรับรองและกุญแจ (ใช้ร่วมกับ QUIC และ TCP/TLS)
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

impl TlsContext {
    // คำนวณ SHA-256 fingerprint ของใบรับรองแรกใน chain
    pub fn cert_fingerprint(&self) -> Result<[u8; 32], TlsError> {
        let der = self
            .certs
            .first()
            .ok_or_else(|| TlsError::CertLoading("no certificates available".into()))?;
        Ok(Sha256::digest(der.as_ref()).into())
    }

    // ตรวจสอบว่า fingerprint ของ peer cert ตรงกับ expected pin หรือไม่
    // ป้องกัน Stale Certificate / CRL Propagation Delay
    pub fn verify_peer_cert_pinned(expected_pin: &[u8; 32], end_entity: &CertificateDer) -> bool {
        let actual = Sha256::digest(end_entity.as_ref());
        let mut expected = *expected_pin;
        let result = actual.as_slice().ct_eq(&expected).into();
        expected.zeroize();
        result
    }

    // สร้างใบรับรองแบบ Self-Signed สำหรับการพัฒนาและทดสอบ
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

    // โหลดใบรับรองและกุญแจจากไฟล์ (PEM format)
    pub fn load(cert_path: &str, key_path: &str) -> Result<Self, TlsError> {
        let certs = load_certs(cert_path)?;
        let key = load_key(key_path)?;
        Ok(Self {
            certs,
            key,
            ca_certs: Vec::new(),
        })
    }

    // เพิ่มใบรับรอง CA สำหรับการตรวจสอบไคลเอนต์
    pub fn add_ca_cert(&mut self, path: &str) -> Result<(), TlsError> {
        let certs = load_certs(path)?;
        self.ca_certs.extend(certs);
        Ok(())
    }

    // แปลงเป็น QUIC Server Config สำหรับ Quinn
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
        transport.max_concurrent_bidi_streams(16u32.into());
        transport.receive_window(524_288u32.into());
        transport.send_window(524_288u32.into());
        config.transport_config(Arc::new(transport));
        Ok(config)
    }

    // แปลงเป็น QUIC Client Config สำหรับ Quinn
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

    // แปลงเป็น Rustls Client Config สำหรับ TCP/TLS
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

    // แปลงเป็น Rustls Server Config สำหรับ TCP/TLS
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
}

// อ่านใบรับรองจากไฟล์ PEM
fn load_certs(path: &str) -> Result<Vec<CertificateDer<'static>>, TlsError> {
    let bytes = std::fs::read(path)
        .map_err(|e| TlsError::CertLoading(format!("cannot read {path}: {e}")))?;
    let certs = rustls_pemfile::certs(&mut bytes.as_slice())
        .collect::<Result<Vec<_>, _>>()
        .map_err(|e| TlsError::CertLoading(e.to_string()))?;
    Ok(certs)
}

// อ่านกุญแจส่วนตัวจากไฟล์ PEM
fn load_key(path: &str) -> Result<PrivateKeyDer<'static>, TlsError> {
    let bytes = std::fs::read(path)
        .map_err(|e| TlsError::CertLoading(format!("cannot read {path}: {e}")))?;
    let mut reader = &bytes[..];
    rustls_pemfile::private_key(&mut reader)
        .map_err(|e| TlsError::CertLoading(e.to_string()))?
        .ok_or_else(|| TlsError::CertLoading("no private key found".into()))
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
        assert!(ctx.to_quic_client_config().is_ok());
        assert!(ctx.to_rustls_client_config().is_ok());
    }
}

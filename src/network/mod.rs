// ระบบนี้คือ High-Speed Blockchain Banking System สำหรับ NDID (National Digital ID)
// ระบบพัฒนาขึ้นเพื่อรองรับการทำธุรกรรมธนาคารข้ามประเทศอย่างรวดเร็ว
// ภาษา: Rust, รันไทม์: Tokio async, โปรโตคอล: QUIC + TCP/TLS 1.3 Auto-Fallback
// ชั้นบริการ API: GraphQL (async-graphql) over Axum
// บล็อกเชน: Substrate (Private Permissioned Ledger)
// คริปโต: ED25519 (signing), AES-GCM (encryption), SHA-256 (hashing)

pub mod quic_channel;
pub mod tcp_channel;
pub mod tls;

use thiserror::Error;
use tokio::time::{Duration, timeout};
use tracing::{info, warn};

use tokio::io::{AsyncReadExt, AsyncWriteExt};

use self::tls::TlsContext;

#[derive(Debug, Error)]
pub enum NetworkError {
    #[error("quic connection failed: {0}")]
    QuicFailed(String),
    #[error("tcp connection failed: {0}")]
    TcpFailed(String),
    #[error("both protocols failed")]
    BothFailed,
    #[error("timeout")]
    Timeout,
    #[error("tls error: {0}")]
    TlsError(String),
    #[error("connection lost: {0}")]
    ConnectionLost(String),
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Protocol {
    Quic,
    Tcp,
}

impl std::fmt::Display for Protocol {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Protocol::Quic => write!(f, "QUIC (0-RTT)"),
            Protocol::Tcp => write!(f, "TCP + TLS 1.3"),
        }
    }
}

pub enum ConnectionStream {
    Quic {
        connection: quinn::Connection,
        active_recv: tokio::sync::Mutex<Option<quinn::RecvStream>>,
    },
    TcpTls(Box<tokio::sync::Mutex<tokio_rustls::client::TlsStream<tokio::net::TcpStream>>>),
}

impl std::fmt::Debug for ConnectionStream {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Quic { .. } => write!(f, "Quic(..)"),
            Self::TcpTls(_) => write!(f, "TcpTls(..)"),
        }
    }
}

#[derive(Debug)]
pub struct NetworkChannel {
    pub protocol: Protocol,
    pub addr: String,
    pub stream: Option<ConnectionStream>,
}

#[async_trait::async_trait]
pub trait ConnectionChannel: Send + Sync {
    async fn connect(&self, addr: &str, tls: &TlsContext) -> Result<NetworkChannel, NetworkError>;
    async fn send(&self, data: &[u8]) -> Result<(), NetworkError>;
    async fn receive(&self) -> Result<Vec<u8>, NetworkError>;
}

#[async_trait::async_trait]
impl ConnectionChannel for NetworkChannel {
    async fn connect(&self, addr: &str, tls: &TlsContext) -> Result<NetworkChannel, NetworkError> {
        let (channel, _) = connect_with_fallback(addr, tls).await;
        if channel.stream.is_some() {
            Ok(channel)
        } else {
            Err(NetworkError::BothFailed)
        }
    }

    async fn send(&self, data: &[u8]) -> Result<(), NetworkError> {
        let stream = self
            .stream
            .as_ref()
            .ok_or_else(|| NetworkError::ConnectionLost("Not connected".into()))?;

        match stream {
            ConnectionStream::Quic {
                connection,
                active_recv,
            } => {
                let (mut send_stream, recv_stream) = connection
                    .open_bi()
                    .await
                    .map_err(|e| NetworkError::ConnectionLost(e.to_string()))?;

                send_stream
                    .write_all(data)
                    .await
                    .map_err(|e| NetworkError::ConnectionLost(e.to_string()))?;

                send_stream
                    .finish()
                    .map_err(|e| NetworkError::ConnectionLost(e.to_string()))?;

                let mut guard = active_recv.lock().await;
                *guard = Some(recv_stream);

                Ok(())
            }
            ConnectionStream::TcpTls(tls_mutex) => {
                let mut tls_stream = tls_mutex.lock().await;

                tls_stream
                    .write_all(data)
                    .await
                    .map_err(|e| NetworkError::TcpFailed(e.to_string()))?;

                tls_stream
                    .flush()
                    .await
                    .map_err(|e| NetworkError::TcpFailed(e.to_string()))?;

                tls_stream
                    .shutdown()
                    .await
                    .map_err(|e| NetworkError::TcpFailed(e.to_string()))?;

                Ok(())
            }
        }
    }

    async fn receive(&self) -> Result<Vec<u8>, NetworkError> {
        let stream = self
            .stream
            .as_ref()
            .ok_or_else(|| NetworkError::ConnectionLost("Not connected".into()))?;

        match stream {
            ConnectionStream::Quic { active_recv, .. } => {
                let mut guard = active_recv.lock().await;
                let mut recv_stream = guard.take().ok_or_else(|| {
                    NetworkError::ConnectionLost(
                        "No active receive stream. Call send() first.".into(),
                    )
                })?;

                let buf = recv_stream
                    .read_to_end(65536)
                    .await
                    .map_err(|e| NetworkError::ConnectionLost(e.to_string()))?;

                Ok(buf)
            }
            ConnectionStream::TcpTls(tls_mutex) => {
                let mut tls_stream = tls_mutex.lock().await;
                let mut buf = Vec::new();
                tls_stream
                    .read_to_end(&mut buf)
                    .await
                    .map_err(|e| NetworkError::TcpFailed(e.to_string()))?;

                Ok(buf)
            }
        }
    }
}

pub fn process_p2p_message(buf: &[u8]) -> String {
    use crate::crypto;
    use crate::p2p_quic::P2pMessage;

    if let Ok(msg) = serde_json::from_slice::<P2pMessage>(buf) {
        let payload_clone = msg.payload.clone();
        let signed = crypto::SignedPayload {
            payload: msg.payload.clone(),
            signature: msg.signature.clone(),
            public_key: msg.public_key.clone(),
        };
        match crypto::verify(&signed) {
            Ok(true) => {
                let payload_str = String::from_utf8(payload_clone.clone()).unwrap_or_else(|_| {
                    warn!("P2P payload contains invalid UTF-8, logging hex");
                    hex::encode(&payload_clone)
                });
                info!(from = %msg.from_bank, payload = %payload_str, "P2P signature verified");
                crate::metrics::p2p_messages()
                    .with_label_values(&["in", &msg.from_bank, "Success"])
                    .inc();
                format!("ACK:{}", payload_str)
            }
            _ => {
                warn!("P2P signature verification failed");
                crate::metrics::p2p_messages()
                    .with_label_values(&["in", &msg.from_bank, "InvalidSignature"])
                    .inc();
                "ERROR: Invalid Signature".to_string()
            }
        }
    } else {
        warn!("Received non-JSON P2P message, logging hex");
        crate::metrics::p2p_messages()
            .with_label_values(&["in", "unknown", "InvalidFormat"])
            .inc();
        hex::encode(buf)
    }
}

pub async fn connect_with_fallback(addr: &str, tls: &TlsContext) -> (NetworkChannel, Protocol) {
    let quic_timeout = Duration::from_millis(500);
    let quic_config = match tls.to_quic_client_config(false) {
        Ok(c) => c,
        Err(e) => {
            warn!(error = %e, "Failed to build QUIC client config, skipping QUIC");
            return fallback_to_tcp(addr, tls).await;
        }
    };

    match timeout(quic_timeout, quic_channel::connect_quic(addr, &quic_config)).await {
        Ok(Ok(channel)) => {
            info!(addr = %addr, "Connected via QUIC");
            (channel, Protocol::Quic)
        }
        Ok(Err(e)) => {
            warn!(addr = %addr, error = %e, "QUIC failed, falling back to TCP");
            fallback_to_tcp(addr, tls).await
        }
        Err(_) => {
            warn!(addr = %addr, "QUIC timeout (500ms), falling back to TCP");
            fallback_to_tcp(addr, tls).await
        }
    }
}

async fn fallback_to_tcp(addr: &str, tls: &TlsContext) -> (NetworkChannel, Protocol) {
    match tcp_channel::connect_tcp_tls(addr, tls).await {
        Ok(channel) => {
            info!(addr = %addr, "Connected via TCP+TLS fallback");
            (channel, Protocol::Tcp)
        }
        Err(e) => {
            warn!(addr = %addr, error = %e, "TCP fallback also failed");
            (
                NetworkChannel {
                    protocol: Protocol::Tcp,
                    addr: addr.to_string(),
                    stream: None,
                },
                Protocol::Tcp,
            )
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_fallback_on_unreachable() {
        let tls = tls::TlsContext::generate_self_signed().unwrap();
        let (_channel, proto) = connect_with_fallback("127.0.0.1:19999", &tls).await;
        assert_eq!(proto, Protocol::Tcp);
    }

    #[test]
    fn test_protocol_display() {
        assert!(format!("{}", Protocol::Quic).contains("QUIC"));
        assert!(format!("{}", Protocol::Tcp).contains("TCP"));
    }
}

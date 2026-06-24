pub mod quic_channel;
pub mod tcp_channel;
pub mod tls;

use thiserror::Error;
use tokio::time::{timeout, Duration};
use tracing::{info, warn};

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
    Quic(quinn::Connection),
    TcpTls(tokio_rustls::client::TlsStream<tokio::net::TcpStream>),
}

impl std::fmt::Debug for ConnectionStream {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Quic(_) => write!(f, "Quic(..)"),
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

pub async fn connect_with_fallback(addr: &str, tls: &TlsContext) -> (NetworkChannel, Protocol) {
    let quic_timeout = Duration::from_millis(500);
    let quic_config = match tls.to_quic_client_config(true) {
        Ok(c) => c,
        Err(_) => {
            warn!("Failed to build QUIC client config, skipping QUIC");
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

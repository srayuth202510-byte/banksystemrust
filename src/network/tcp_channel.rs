// ระบบนี้คือ High-Speed Blockchain Banking System สำหรับ NDID (National Digital ID)
// ระบบพัฒนาขึ้นเพื่อรองรับการทำธุรกรรมธนาคารข้ามประเทศอย่างรวดเร็ว
// ภาษา: Rust, รันไทม์: Tokio async, โปรโตคอล: QUIC + TCP/TLS 1.3 Auto-Fallback
// ชั้นบริการ API: GraphQL (async-graphql) over Axum
// บล็อกเชน: Substrate (Private Permissioned Ledger)
// คริปโต: ED25519 (signing), AES-GCM (encryption), SHA-256 (hashing)

use crate::network::tls::TlsContext;
use crate::network::{ConnectionStream, NetworkChannel, NetworkError, Protocol};
use std::sync::Arc;
use tokio::io::{AsyncReadExt, AsyncWriteExt};
use tokio::net::{TcpListener, TcpStream};
use tokio::sync::broadcast;
use tokio::time::{Duration, timeout};
use tokio_rustls::{TlsAcceptor, TlsConnector};
use tracing::{info, warn};

pub async fn connect_tcp_tls(addr: &str, tls: &TlsContext) -> Result<NetworkChannel, NetworkError> {
    info!(addr = %addr, "Attempting TCP+TLS connection");

    let tcp_timeout = Duration::from_secs(2);
    let tcp = timeout(tcp_timeout, TcpStream::connect(addr))
        .await
        .map_err(|_| NetworkError::Timeout)?
        .map_err(|e| NetworkError::TcpFailed(format!("tcp connect failed: {e}")))?;

    let tls_config = tls
        .to_rustls_client_config(false)
        .map_err(|e| NetworkError::TlsError(e.to_string()))?;

    let connector = TlsConnector::from(Arc::new(tls_config));
    let server_name = rustls::pki_types::ServerName::try_from("localhost")
        .map_err(|_| NetworkError::TlsError("invalid server name".into()))?;

    let tls_stream = connector
        .connect(server_name, tcp)
        .await
        .map_err(|e| NetworkError::TlsError(format!("tls handshake failed: {e}")))?;

    info!(addr = %addr, "TCP+TLS handshake complete");
    Ok(NetworkChannel {
        protocol: Protocol::Tcp,
        addr: addr.to_string(),
        stream: Some(ConnectionStream::TcpTls(Box::new(tokio::sync::Mutex::new(
            tls_stream,
        )))),
    })
}

pub async fn start_tcp_server(
    bind_addr: &str,
    tls: &TlsContext,
    mut shutdown_rx: broadcast::Receiver<()>,
) -> Result<(), NetworkError> {
    let listener = TcpListener::bind(bind_addr)
        .await
        .map_err(|e| NetworkError::TcpFailed(format!("server bind failed: {e}")))?;

    let server_config = tls
        .to_rustls_server_config()
        .map_err(|e| NetworkError::TlsError(format!("failed to get rustls config: {e}")))?;

    let acceptor = TlsAcceptor::from(Arc::new(server_config));

    info!(addr = %bind_addr, "TCP+TLS fallback server listening");

    loop {
        tokio::select! {
            _ = shutdown_rx.recv() => {
                info!("TCP server shutting down");
                break;
            }
            result = listener.accept() => {
                match result {
                    Ok((stream, remote_addr)) => {
                        let acceptor = acceptor.clone();
                        tokio::spawn(async move {
                            match acceptor.accept(stream).await {
                                Ok(tls_stream) => {
                                    handle_tcp_connection(tls_stream, remote_addr).await;
                                }
                                Err(e) => {
                                    warn!(remote = %remote_addr, error = %e, "TCP+TLS accept failed");
                                }
                            }
                        });
                    }
                    Err(e) => {
                        warn!(error = %e, "TCP accept error");
                    }
                }
            }
        }
    }
    Ok(())
}

pub async fn handle_tcp_connection(
    mut tls_stream: tokio_rustls::server::TlsStream<TcpStream>,
    remote: std::net::SocketAddr,
) {
    info!(remote = %remote, "TCP+TLS connection accepted");

    let mut buf = Vec::new();
    match tls_stream.read_to_end(&mut buf).await {
        Ok(_) => {
            info!(remote = %remote, len = %buf.len(), "TCP data received");
            let response = crate::network::process_p2p_message(&buf);
            if let Err(e) = tls_stream.write_all(response.as_bytes()).await {
                warn!(error = %e, "TCP send response failed");
            }
            if let Err(e) = tls_stream.flush().await {
                warn!(error = %e, "TCP flush failed");
            }
            let _ = tls_stream.shutdown().await;
        }
        Err(e) => {
            warn!(remote = %remote, error = %e, "TCP read error");
        }
    }

    info!(remote = %remote, "TCP+TLS connection closed");
}

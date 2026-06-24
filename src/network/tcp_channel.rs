use std::sync::Arc;
use tokio::net::{TcpStream, TcpListener};
use tokio::io::{AsyncReadExt, AsyncWriteExt};
use tokio::time::{timeout, Duration};
use tokio_rustls::{TlsConnector, TlsAcceptor, rustls::ClientConfig};
use tracing::{info, warn};
use crate::network::{NetworkChannel, NetworkError, Protocol, ConnectionStream};
use crate::network::tls::TlsContext;

pub async fn connect_tcp_tls(addr: &str, _tls: &TlsContext) -> Result<NetworkChannel, NetworkError> {
    info!(addr = %addr, "Attempting TCP+TLS connection");

    let tcp_timeout = Duration::from_secs(2);
    let tcp = timeout(tcp_timeout, TcpStream::connect(addr)).await
        .map_err(|_| NetworkError::Timeout)?
        .map_err(|e| NetworkError::TcpFailed(format!("tcp connect failed: {e}")))?;

    let tls_config = ClientConfig::builder_with_provider(
        rustls::crypto::ring::default_provider().into()
    )
    .with_protocol_versions(&[&rustls::version::TLS13])
    .map_err(|e| NetworkError::TlsError(e.to_string()))?
    .dangerous()
    .with_custom_certificate_verifier(
        Arc::new(crate::network::tls::SkipCertVerifier)
    )
    .with_no_client_auth();

    let connector = TlsConnector::from(Arc::new(tls_config));
    let server_name = rustls::pki_types::ServerName::try_from("localhost")
        .map_err(|_| NetworkError::TlsError("invalid server name".into()))?;

    let tls_stream = connector.connect(server_name, tcp).await
        .map_err(|e| NetworkError::TlsError(format!("tls handshake failed: {e}")))?;

    info!(addr = %addr, "TCP+TLS handshake complete");
    Ok(NetworkChannel {
        protocol: Protocol::Tcp,
        addr: addr.to_string(),
        stream: Some(ConnectionStream::TcpTls(tls_stream)),
    })
}

pub async fn start_tcp_server(
    bind_addr: &str,
    tls: &TlsContext,
) -> Result<(), NetworkError> {
    let listener = TcpListener::bind(bind_addr).await
        .map_err(|e| NetworkError::TcpFailed(format!("server bind failed: {e}")))?;

    let server_config = tls.to_rustls_server_config()
        .map_err(|e| NetworkError::TlsError(format!("failed to get rustls config: {e}")))?;
    
    let acceptor = TlsAcceptor::from(Arc::new(server_config));

    info!(addr = %bind_addr, "TCP+TLS fallback server listening");

    loop {
        match listener.accept().await {
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

pub async fn handle_tcp_connection(mut tls_stream: tokio_rustls::server::TlsStream<TcpStream>, remote: std::net::SocketAddr) {
    info!(remote = %remote, "TCP+TLS connection accepted");

    let mut buf = vec![0u8; 65536];
    loop {
        match tls_stream.read(&mut buf).await {
            Ok(0) => break, // EOF
            Ok(n) => {
                info!(remote = %remote, len = %n, "TCP data received");
                if let Err(e) = tls_stream.write_all(&buf[..n]).await {
                    warn!(error = %e, "TCP send failed");
                    break;
                }
            }
            Err(e) => {
                warn!(remote = %remote, error = %e, "TCP read error");
                break;
            }
        }
    }

    info!(remote = %remote, "TCP+TLS connection closed");
}

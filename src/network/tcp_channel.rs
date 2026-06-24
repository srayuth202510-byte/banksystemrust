use std::sync::Arc;
use tokio::net::TcpStream;
use tokio_rustls::{TlsConnector, rustls::ClientConfig};
use tracing::info;
use crate::network::{NetworkChannel, NetworkError, Protocol};
use crate::network::tls::TlsContext;

pub async fn connect_tcp_tls(addr: &str, _tls: &TlsContext) -> Result<NetworkChannel, NetworkError> {
    info!(addr = %addr, "Attempting TCP+TLS connection");

    let tcp = TcpStream::connect(addr).await
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

    connector.connect(server_name, tcp).await
        .map_err(|e| NetworkError::TlsError(format!("tls handshake failed: {e}")))?;

    info!(addr = %addr, "TCP+TLS handshake complete");
    Ok(NetworkChannel {
        protocol: Protocol::Tcp,
        addr: addr.to_string(),
        connection: None,
    })
}

pub async fn connect_tcp(addr: &str) -> Result<NetworkChannel, NetworkError> {
    info!(addr = %addr, "Attempting plain TCP connection");

    TcpStream::connect(addr).await
        .map_err(|e| NetworkError::TcpFailed(format!("tcp connect failed: {e}")))?;

    info!(addr = %addr, "TCP connection established");
    Ok(NetworkChannel {
        protocol: Protocol::Tcp,
        addr: addr.to_string(),
        connection: None,
    })
}

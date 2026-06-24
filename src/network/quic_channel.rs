use std::net::SocketAddr;

use tracing::{info, warn};
use crate::network::{NetworkChannel, NetworkError, Protocol};

pub async fn connect_quic(addr: &str, config: &quinn::ClientConfig) -> Result<NetworkChannel, NetworkError> {
    info!(addr = %addr, "Attempting QUIC connection");

    let server_addr: SocketAddr = addr.parse()
        .map_err(|e| NetworkError::QuicFailed(format!("invalid address {addr}: {e}")))?;

    let local_addr: SocketAddr = if server_addr.is_ipv4() {
        "0.0.0.0:0".parse().unwrap()
    } else {
        "[::]:0".parse().unwrap()
    };

    let endpoint = quinn::Endpoint::client(local_addr)
        .map_err(|e| NetworkError::QuicFailed(format!("endpoint creation failed: {e}")))?;

    let server_name = "localhost";

    let connecting = endpoint.connect_with(
        config.clone(),
        server_addr,
        server_name,
    ).map_err(|e| NetworkError::QuicFailed(format!("connect failed: {e}")))?;

    let connection = connecting.await
        .map_err(|e| NetworkError::QuicFailed(format!("handshake failed: {e}")))?;

    info!(addr = %addr, "QUIC handshake complete");
    Ok(NetworkChannel {
        protocol: Protocol::Quic,
        addr: addr.to_string(),
        connection: Some(connection),
    })
}

pub async fn start_quic_server(
    bind_addr: &str,
    config: quinn::ServerConfig,
) -> Result<quinn::Endpoint, NetworkError> {
    let addr: SocketAddr = bind_addr.parse()
        .map_err(|e| NetworkError::QuicFailed(format!("invalid bind address {bind_addr}: {e}")))?;

    let endpoint = quinn::Endpoint::server(config, addr)
        .map_err(|e| NetworkError::QuicFailed(format!("server bind failed: {e}")))?;

    info!(addr = %bind_addr, "QUIC server listening");
    Ok(endpoint)
}

pub async fn handle_quic_connection(connection: quinn::Connection) {
    let remote = connection.remote_address();
    info!(remote = %remote, "QUIC connection accepted");

    loop {
        match connection.accept_bi().await {
            Ok((mut _send, mut _recv)) => {
                let mut buf = vec![0u8; 65536];
                match _recv.read(&mut buf).await {
                    Ok(Some(n)) => {
                        info!(remote = %remote, len = %n, "QUIC data received");
                        buf.truncate(n);
                        if let Err(e) = _send.write_all(&buf).await {
                            warn!(error = %e, "QUIC send failed");
                            break;
                        }
                    }
                    Ok(None) => break,
                    Err(e) => {
                        warn!(remote = %remote, error = %e, "QUIC read error");
                        break;
                    }
                }
            }
            Err(e) => {
                warn!(remote = %remote, error = %e, "QUIC accept error");
                break;
            }
        }
    }

    info!(remote = %remote, "QUIC connection closed");
}

// ระบบนี้คือ High-Speed Blockchain Banking System สำหรับ NDID (National Digital ID)
// ระบบพัฒนาขึ้นเพื่อรองรับการทำธุรกรรมธนาคารข้ามประเทศอย่างรวดเร็ว
// ภาษา: Rust, รันไทม์: Tokio async, โปรโตคอล: QUIC + TCP/TLS 1.3 Auto-Fallback
// ชั้นบริการ API: GraphQL (async-graphql) over Axum
// บล็อกเชน: Substrate (Private Permissioned Ledger)
// คริปโต: ED25519 (signing), AES-GCM (encryption), SHA-256 (hashing)

use std::net::SocketAddr;

use crate::network::{NetworkChannel, NetworkError, Protocol};
use tracing::{info, warn};

pub async fn connect_quic(
    addr: &str,
    config: &quinn::ClientConfig,
) -> Result<NetworkChannel, NetworkError> {
    info!(addr = %addr, "Attempting QUIC connection");

    let server_addr: SocketAddr = addr
        .parse()
        .map_err(|e| NetworkError::QuicFailed(format!("invalid address {addr}: {e}")))?;

    let local_addr: SocketAddr = if server_addr.is_ipv4() {
        SocketAddr::new(std::net::IpAddr::V4(std::net::Ipv4Addr::UNSPECIFIED), 0)
    } else {
        SocketAddr::new(std::net::IpAddr::V6(std::net::Ipv6Addr::UNSPECIFIED), 0)
    };

    let endpoint = quinn::Endpoint::client(local_addr)
        .map_err(|e| NetworkError::QuicFailed(format!("endpoint creation failed: {e}")))?;

    let server_name = "localhost";

    let connecting = endpoint
        .connect_with(config.clone(), server_addr, server_name)
        .map_err(|e| NetworkError::QuicFailed(format!("connect failed: {e}")))?;

    let connection = connecting
        .await
        .map_err(|e| NetworkError::QuicFailed(format!("handshake failed: {e}")))?;

    info!(addr = %addr, "QUIC handshake complete");
    Ok(NetworkChannel {
        protocol: Protocol::Quic,
        addr: addr.to_string(),
        stream: Some(crate::network::ConnectionStream::Quic {
            connection,
            active_recv: tokio::sync::Mutex::new(None),
        }),
    })
}

pub async fn start_quic_server(
    bind_addr: &str,
    config: quinn::ServerConfig,
) -> Result<quinn::Endpoint, NetworkError> {
    let addr: SocketAddr = bind_addr
        .parse()
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
            Ok((mut send, mut recv)) => {
                tokio::spawn(async move {
                    match recv.read_to_end(65536).await {
                        Ok(buf) => {
                            info!(remote = %remote, len = %buf.len(), "QUIC data received");
                            let response = crate::network::process_p2p_message(&buf);
                            if let Err(e) = send.write_all(response.as_bytes()).await {
                                warn!(error = %e, "QUIC send response failed");
                            }
                            let _ = send.finish();
                        }
                        Err(e) => {
                            warn!(remote = %remote, error = %e, "QUIC read error");
                        }
                    }
                });
            }
            Err(e) => {
                warn!(remote = %remote, error = %e, "QUIC accept error");
                break;
            }
        }
    }

    info!(remote = %remote, "QUIC connection closed");
}

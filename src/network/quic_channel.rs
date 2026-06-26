// ระบบนี้คือ High-Speed Blockchain Banking System สำหรับ NDID (National Digital ID)
// ระบบพัฒนาขึ้นเพื่อรองรับการทำธุรกรรมธนาคารข้ามประเทศอย่างรวดเร็ว
// ภาษา: Rust, รันไทม์: Tokio async, โปรโตคอล: QUIC + TCP/TLS 1.3 Auto-Fallback
// ชั้นบริการ API: GraphQL (async-graphql) over Axum
// บล็อกเชน: Substrate (Private Permissioned Ledger)
// คริปโต: ED25519 (signing), AES-GCM (encryption), SHA-256 (hashing)

// การเชื่อมต่อ QUIC (Quick UDP Internet Connections) ด้วยไลบรารี Quinn
use std::collections::HashMap;
use std::net::SocketAddr;
use std::sync::atomic::AtomicU32;
use std::sync::Arc;
use std::time::Instant;

use crate::network::{NetworkChannel, NetworkError, Protocol};
use tokio::sync::{Mutex, Semaphore};
use tracing::{info, warn};

// ป้องกัน QUIC Amplification Attack — จำกัดจำนวน concurrent connections
const MAX_CONCURRENT_CONNS: usize = 1024;
const MAX_CONN_RATE_PER_IP: u32 = 50;
const CONN_RATE_WINDOW_SECS: u64 = 1;
const MAX_RESPONSE_BYTES: usize = 1_048_576; // 1MB

// สถานะการจำกัดอัตราการเชื่อมต่อต่อ IP
pub struct QuicRateLimiter {
    conn_semaphore: Arc<Semaphore>,
    per_ip_counts: Mutex<HashMap<SocketAddr, (u32, Instant)>>,
    total_accepted: AtomicU32,
    total_rejected: AtomicU32,
}

impl QuicRateLimiter {
    pub fn new() -> Arc<Self> {
        Arc::new(Self {
            conn_semaphore: Arc::new(Semaphore::new(MAX_CONCURRENT_CONNS)),
            per_ip_counts: Mutex::new(HashMap::new()),
            total_accepted: AtomicU32::new(0),
            total_rejected: AtomicU32::new(0),
        })
    }

    pub async fn check_and_acquire(&self, remote: SocketAddr) -> Result<tokio::sync::OwnedSemaphorePermit, ()> {
        let mut ip_counts = self.per_ip_counts.lock().await;
        let now = Instant::now();
        let (count, last) = ip_counts.entry(remote).or_insert((0, now));
        if now.duration_since(*last).as_secs() >= CONN_RATE_WINDOW_SECS {
            *count = 0;
            *last = now;
        }
        *count += 1;
        if *count > MAX_CONN_RATE_PER_IP {
            self.total_rejected.fetch_add(1, std::sync::atomic::Ordering::Relaxed);
            return Err(());
        }
        drop(ip_counts);

        let permit = self.conn_semaphore
            .clone()
            .try_acquire_owned()
            .map_err(|_| {
                self.total_rejected.fetch_add(1, std::sync::atomic::Ordering::Relaxed);
            })?;

        self.total_accepted.fetch_add(1, std::sync::atomic::Ordering::Relaxed);
        Ok(permit)
    }
}

// เชื่อมต่อ QUIC ไปยังเซิร์ฟเวอร์ปลายทาง
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

    let server_name = addr.split(':').next().unwrap_or("localhost");

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

// เริ่มต้นเซิร์ฟเวอร์ QUIC สำหรับรับการเชื่อมต่อ
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

// จัดการการเชื่อมต่อ QUIC ขาเข้า - รับข้อมูลและตอบกลับ
pub async fn handle_quic_connection(
    connection: quinn::Connection,
    _permit: tokio::sync::OwnedSemaphorePermit,
) {
    let remote = connection.remote_address();
    info!(remote = %remote, "QUIC connection accepted");

    loop {
        match connection.accept_bi().await {
            Ok((mut send, mut recv)) => {
                tokio::spawn(async move {
                    match recv.read_to_end(MAX_RESPONSE_BYTES).await {
                        Ok(buf) => {
                            if buf.len() > MAX_RESPONSE_BYTES {
                                warn!(remote = %remote, len = %buf.len(), "QUIC request exceeds size limit");
                                return;
                            }
                            info!(remote = %remote, len = %buf.len(), "QUIC data received");
                            let response = crate::network::process_p2p_message(&buf);
                            if response.len() > MAX_RESPONSE_BYTES {
                                warn!(remote = %remote, "QUIC response exceeds size limit, truncating");
                                let _ = send.write_all(b"ERROR: Response too large").await;
                            } else if let Err(e) = send.write_all(response.as_bytes()).await {
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
    // permit dropped here → connection slot freed
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_rate_limiter_accepts() {
        let limiter = QuicRateLimiter::new();
        let addr: SocketAddr = "127.0.0.1:12345".parse().unwrap();
        assert!(limiter.check_and_acquire(addr).await.is_ok());
    }

    #[tokio::test]
    async fn test_rate_limiter_rejects_excessive() {
        let limiter = QuicRateLimiter::new();
        let addr: SocketAddr = "127.0.0.1:12346".parse().unwrap();
        for _ in 0..MAX_CONN_RATE_PER_IP {
            let _permit = limiter.check_and_acquire(addr).await.unwrap();
        }
        assert!(limiter.check_and_acquire(addr).await.is_err());
    }
}

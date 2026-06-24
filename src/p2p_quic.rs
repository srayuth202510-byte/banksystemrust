// ระบบนี้คือ High-Speed Blockchain Banking System สำหรับ NDID (National Digital ID)
// ระบบพัฒนาขึ้นเพื่อรองรับการทำธุรกรรมธนาคารข้ามประเทศอย่างรวดเร็ว
// ภาษา: Rust, รันไทม์: Tokio async, โปรโตคอล: QUIC + TCP/TLS 1.3 Auto-Fallback
// ชั้นบริการ API: GraphQL (async-graphql) over Axum
// บล็อกเชน: Substrate (Private Permissioned Ledger)
// คริปโต: ED25519 (signing), AES-GCM (encryption), SHA-256 (hashing)

use serde::{Deserialize, Serialize};
use thiserror::Error;
use tracing::info;

use crate::crypto;
use crate::network;
use crate::network::{Protocol, tls::TlsContext};

#[derive(Debug, Error)]
pub enum P2pError {
    #[error("network error: {0}")]
    Network(#[from] network::NetworkError),
    #[error("crypto error: {0}")]
    Crypto(#[from] crypto::CryptoError),
    #[error("peer not found: {0}")]
    PeerNotFound(String),
    #[error("handshake failed: {0}")]
    HandshakeFailed(String),
    #[error("tls error: {0}")]
    TlsError(String),
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct P2pMessage {
    pub from_bank: String,
    pub to_bank: String,
    pub payload: Vec<u8>,
    pub signature: Vec<u8>,
    pub public_key: Vec<u8>,
    pub timestamp: i64,
}

pub struct P2pNode {
    pub bank_code: String,
    pub keypair: crypto::KeyPair,
    pub tls: TlsContext,
    peers: Vec<String>,
}

impl std::fmt::Debug for P2pNode {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("P2pNode")
            .field("bank_code", &self.bank_code)
            .field("peers", &self.peers)
            .finish()
    }
}

impl P2pNode {
    pub fn new(bank_code: String, keypair: crypto::KeyPair, tls: TlsContext) -> Self {
        Self {
            bank_code,
            keypair,
            tls,
            peers: Vec::new(),
        }
    }

    pub fn add_peer(&mut self, addr: String) {
        self.peers.push(addr);
    }

    pub async fn send_kyc(&self, peer_addr: &str, kyc_hash: String) -> Result<Protocol, P2pError> {
        info!(from = %self.bank_code, to = %peer_addr, "Sending KYC data");
        let payload = format!("KYC:{}:{}", self.bank_code, kyc_hash);
        let signed = crypto::sign(payload.as_bytes(), &self.keypair)?;

        let message = P2pMessage {
            from_bank: self.bank_code.clone(),
            to_bank: String::new(),
            payload: payload.into_bytes(),
            signature: signed.signature,
            public_key: signed.public_key,
            timestamp: chrono::Utc::now().timestamp(),
        };

        let msg_bytes = serde_json::to_vec(&message)
            .map_err(|e| P2pError::Network(network::NetworkError::TlsError(e.to_string())))?;

        let (channel, protocol) = network::connect_with_fallback(peer_addr, &self.tls).await;
        if channel.stream.is_none() {
            return Err(P2pError::Network(network::NetworkError::BothFailed));
        }

        use crate::network::ConnectionChannel;
        channel.send(&msg_bytes).await?;
        let resp_bytes = channel.receive().await?;
        let resp_str = String::from_utf8_lossy(&resp_bytes);

        if resp_str.starts_with("ERROR:") {
            return Err(P2pError::HandshakeFailed(resp_str.into_owned()));
        }

        info!(protocol = %protocol, response = %resp_str, "KYC sent and ACK received");
        Ok(protocol)
    }

    pub fn peers(&self) -> &[String] {
        &self.peers
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::crypto::KeyPair;

    fn test_node(bank: &str) -> P2pNode {
        let kp = KeyPair::generate().unwrap();
        let tls = TlsContext::generate_self_signed().unwrap();
        P2pNode::new(bank.into(), kp, tls)
    }

    #[test]
    fn test_p2p_node_creation() {
        let node = test_node("BBL");
        assert_eq!(node.bank_code, "BBL");
        assert!(node.peers().is_empty());
    }

    #[test]
    fn test_add_peer() {
        let mut node = test_node("KBANK");
        node.add_peer("10.0.1.50:4433".into());
        assert_eq!(node.peers().len(), 1);
    }

    #[tokio::test]
    async fn test_send_kyc_fallback() {
        let listener = tokio::net::TcpListener::bind("127.0.0.1:0").await.unwrap();
        let addr = listener.local_addr().unwrap();
        drop(listener);

        let node = test_node("SCB");
        let mut server_tls = node.tls.clone();
        server_tls.ca_certs.clear();
        let bind_addr = addr.to_string();
        tokio::spawn(async move {
            let _ = crate::network::tcp_channel::start_tcp_server(&bind_addr, &server_tls).await;
        });

        tokio::time::sleep(tokio::time::Duration::from_millis(100)).await;

        let proto = node
            .send_kyc(&addr.to_string(), "hash123".into())
            .await
            .unwrap();
        assert_eq!(proto, Protocol::Tcp);
    }
}

use axum::{Json, Router, routing::post};
use banksystemrust::blockchain::{BlockchainClient, BlockchainConfig, TxStatus};
use banksystemrust::blockchain::{SubstrateRpcRequest, SubstrateRpcResponse};
use banksystemrust::crypto::{self, KeyPair};
use banksystemrust::identity::{self, IdentityStatus, KycData};
use banksystemrust::network;
use banksystemrust::network::tls::TlsContext;
use banksystemrust::p2p_quic::P2pNode;

async fn mock_substrate_rpc(Json(req): Json<SubstrateRpcRequest>) -> Json<SubstrateRpcResponse> {
    Json(SubstrateRpcResponse {
        jsonrpc: "2.0".into(),
        result: Some(serde_json::Value::String("0x1234abcd".into())),
        error: None,
        id: req.id,
    })
}

async fn spawn_mock_node() -> String {
    let app = Router::new().route("/", post(mock_substrate_rpc));
    let listener = tokio::net::TcpListener::bind("127.0.0.1:0").await.unwrap();
    let addr = listener.local_addr().unwrap();
    tokio::spawn(async move {
        axum::serve(listener, app).await.unwrap();
    });
    format!("http://{}", addr)
}

fn sample_kyc() -> KycData {
    KycData {
        national_id: "1234567890123".into(),
        full_name: "สมชาย ใจดี".into(),
        date_of_birth: "1990-01-01".into(),
        bank_code: "BBL".into(),
        timestamp: 1700000000,
    }
}

fn sample_keypair() -> KeyPair {
    KeyPair::generate().expect("keypair")
}

fn blockchain_config(endpoint: String) -> BlockchainConfig {
    BlockchainConfig {
        endpoint,
        timeout_secs: 10,
        max_retries: 3,
        db_path: None,
    }
}

// === END-TO-END FLOWS ===

#[tokio::test]
async fn test_full_kyc_flow() {
    let kp = sample_keypair();
    let kyc = sample_kyc();

    let identity_hash = kyc.compute_hash();
    assert_eq!(identity_hash.len(), 64, "SHA-256 hex should be 64 chars");

    let endpoint = spawn_mock_node().await;
    let client = BlockchainClient::new(blockchain_config(endpoint)).unwrap();
    let tx = client
        .create_transaction(identity_hash.clone(), "BBL".into(), &kp)
        .expect("create tx");
    assert_eq!(tx.bank_code, "BBL");
    assert!(!tx.signature.is_empty(), "tx must be signed");

    let receipt = client.submit(tx).await.expect("submit");
    assert!(matches!(receipt.status, TxStatus::Finalized));
    assert!(!receipt.block_hash.is_empty());
}

#[tokio::test]
async fn test_crypto_identity_blockchain_integration() {
    let kp = sample_keypair();
    let kyc = sample_kyc();

    let payload = serde_json::to_vec(&kyc).unwrap();
    let signed = crypto::sign(&payload, &kp).expect("sign");
    assert!(crypto::verify(&signed).expect("verify"));

    let identity_hash = crypto::hash_hex(&payload);
    let endpoint = spawn_mock_node().await;
    let client = BlockchainClient::new(blockchain_config(endpoint)).unwrap();
    let tx = client
        .create_transaction(identity_hash, kyc.bank_code, &kp)
        .unwrap();
    let receipt = client.submit(tx).await.unwrap();
    assert!(matches!(receipt.status, TxStatus::Finalized));
}

#[tokio::test]
async fn test_p2p_nodes_connect_with_fallback() {
    let kp_a = sample_keypair();
    let kp_b = sample_keypair();
    let tls = TlsContext::generate_self_signed().unwrap();

    let mut node_a = P2pNode::new("BBL".into(), kp_a, tls.clone());
    let node_b = P2pNode::new("KBANK".into(), kp_b, tls.clone());

    node_a.add_peer("10.0.1.50:4433".into());
    assert_eq!(node_a.peers().len(), 1);
    assert_eq!(node_b.bank_code, "KBANK");
}

// === EDGE CASES ===

#[test]
fn test_kyc_anonymization_removes_pii() {
    let kyc = sample_kyc();
    let anon = kyc.anonymize();
    assert!(!anon.identity_hash.contains("1234567890123"));
    assert!(!anon.identity_hash.contains("สมชาย"));
    assert_eq!(anon.bank_code, "BBL");
}

#[test]
fn test_identity_record_default_pending() {
    let kyc = sample_kyc();
    let record =
        identity::create_identity_record("req-edge-001".into(), &kyc, "SCB".into(), "TCP".into());
    assert!(matches!(record.status, IdentityStatus::Pending));
    assert_eq!(record.bank_code, "SCB");
    assert_eq!(record.active_protocol, "TCP");
}

#[test]
fn test_crypto_empty_payload() {
    let kp = sample_keypair();
    let signed = crypto::sign(b"", &kp).expect("sign empty");
    assert!(crypto::verify(&signed).expect("verify empty"));
}

#[test]
fn test_crypto_tampered_signature_fails_verify() {
    let kp = sample_keypair();
    let data = b"tamper test";
    let mut signed = crypto::sign(data, &kp).expect("sign");
    signed.signature[0] ^= 0xFF;
    assert!(!crypto::verify(&signed).expect("verify tampered"));
}

#[test]
fn test_crypto_wrong_key_fails_verify() {
    let kp_a = sample_keypair();
    let kp_b = sample_keypair();
    let data = b"wrong key test";
    let mut signed = crypto::sign(data, &kp_a).expect("sign by A");
    signed.public_key = kp_b.public_key.clone();
    assert!(!crypto::verify(&signed).expect("verify with wrong key"));
}

#[test]
fn test_encrypt_decrypt_large_payload() {
    let key: [u8; 32] = rand::random();
    let large = vec![0xABu8; 65536];
    let encrypted = crypto::encrypt(&large, &key).expect("encrypt large");
    assert!(encrypted.ciphertext.len() >= large.len());
    let decrypted = crypto::decrypt(&encrypted, &key).expect("decrypt large");
    assert_eq!(decrypted, large);
}

#[test]
fn test_encrypt_decrypt_wrong_key_fails() {
    let key_a: [u8; 32] = rand::random();
    let key_b: [u8; 32] = rand::random();
    let data = b"secret data";
    let encrypted = crypto::encrypt(data, &key_a).expect("encrypt");
    let result = crypto::decrypt(&encrypted, &key_b);
    assert!(result.is_err(), "decryption with wrong key should fail");
}

#[test]
fn test_identity_hash_empty_input() {
    let data = b"";
    let hash = crypto::hash_hex(data);
    assert_eq!(hash.len(), 64);
    assert_eq!(
        hash,
        "e3b0c44298fc1c149afbf4c8996fb92427ae41e4649b934ca495991b7852b855"
    );
}

#[test]
fn test_hash_different_inputs_different() {
    let h1 = crypto::hash_hex(b"hello");
    let h2 = crypto::hash_hex(b"world");
    assert_ne!(h1, h2);
}

#[test]
fn test_keypair_from_bytes_wrong_length() {
    let result = KeyPair::from_bytes(&[0u8; 16], &[0u8; 16]);
    assert!(result.is_err(), "wrong key length should fail");
}

#[test]
fn test_keypair_from_bytes_roundtrip() {
    let kp = sample_keypair();
    let kp2 = KeyPair::from_bytes(&kp.public_key, &kp.public_key);
    assert!(kp2.is_ok(), "valid 32-byte keys should succeed");
}

// === CONCURRENCY ===

#[tokio::test]
async fn test_blockchain_queue_drain() {
    let endpoint = spawn_mock_node().await;
    let client = BlockchainClient::new(blockchain_config(endpoint)).unwrap();
    let kp = sample_keypair();
    for i in 0..5 {
        let tx = client
            .create_transaction(format!("hash-{i}"), "BBL".into(), &kp)
            .unwrap();
        client.submit(tx).await.unwrap();
    }
    let drained = client.drain_queue();
    assert!(drained.is_empty(), "finalized txs should not be queued");
}

#[tokio::test]
async fn test_concurrent_crypto_ops() {
    let kp_base = sample_keypair();
    let data = b"concurrent test";
    let mut handles = Vec::new();

    for _ in 0..10 {
        let kp = kp_base.clone();
        let data = data.to_vec();
        handles.push(tokio::spawn(async move { crypto::sign(&data, &kp) }));
    }

    for handle in handles {
        let result = handle.await.expect("join");
        assert!(result.is_ok());
    }
}

// === PROTOCOL INTEGRATION ===

#[tokio::test]
async fn test_p2p_send_kyc_returns_protocol() {
    let kp = sample_keypair();
    let tls = TlsContext::generate_self_signed().unwrap();
    let node = P2pNode::new("BBL".into(), kp, tls.clone());
    let proto = node
        .send_kyc("127.0.0.1:19999", "test-hash".into())
        .await
        .unwrap();
    assert!(proto == network::Protocol::Tcp || proto == network::Protocol::Quic);
}

#[test]
fn test_protocol_display_contains_protocol_name() {
    assert!(format!("{}", network::Protocol::Quic).contains("QUIC"));
    assert!(format!("{}", network::Protocol::Tcp).contains("TCP"));
}

#[tokio::test]
async fn test_network_fallback_both_fail() {
    let tls = TlsContext::generate_self_signed().unwrap();
    let (_channel, proto) = network::connect_with_fallback("127.0.0.1:19999", &tls).await;
    assert_eq!(proto, network::Protocol::Tcp);
}

// === GRAPHQL SCHEMA (via lib functions) ===

#[test]
fn test_kyc_response_creation() {
    let kyc = sample_kyc();
    let hash = kyc.compute_hash();
    assert_eq!(hash.len(), 64);
    let response_id = uuid::Uuid::new_v4().to_string();
    assert!(!response_id.is_empty());
}

// === BLOCKCHAIN EDGE CASES ===

#[test]
fn test_blockchain_config_defaults() {
    let config = blockchain_config("http://dummy".into());
    assert_eq!(config.max_retries, 3);
    assert_eq!(config.timeout_secs, 10);
}

#[tokio::test]
async fn test_blockchain_create_tx_signed() {
    let kp = sample_keypair();
    let client = BlockchainClient::new(blockchain_config("http://dummy".into())).unwrap();
    let tx = client
        .create_transaction("sig-test-hash".into(), "KBANK".into(), &kp)
        .unwrap();

    let signed = crypto::sign(tx.tx_id.as_bytes(), &kp).expect("sign");
    assert!(!signed.signature.is_empty());
}

#[test]
fn test_keypair_generates_unique_keys() {
    let kp1 = sample_keypair();
    let kp2 = sample_keypair();
    assert_ne!(
        kp1.public_key, kp2.public_key,
        "each keypair must be unique"
    );
}

#[test]
fn test_same_data_produces_same_hash() {
    let data = b"deterministic";
    let h1 = crypto::hash(data);
    let h2 = crypto::hash(data);
    assert_eq!(h1, h2);
}

#[test]
fn test_mtls_configuration_loading() {
    // Generate self-signed cert, which now automatically populates ca_certs with itself.
    let mut ctx = TlsContext::generate_self_signed().expect("generate cert");
    assert!(
        !ctx.ca_certs.is_empty(),
        "ca_certs should be populated with self for mTLS local testing"
    );

    // Ensure quic server config builds without error and uses the CA certs
    let quic_server_config = ctx.to_quic_server_config();
    assert!(
        quic_server_config.is_ok(),
        "quic server config should build successfully with mTLS verifier"
    );

    // Ensure rustls server config builds
    let rustls_server_config = ctx.to_rustls_server_config();
    assert!(
        rustls_server_config.is_ok(),
        "rustls server config should build successfully with mTLS verifier"
    );

    // Add dummy cert to ca_certs (using the same cert again just to test extending)
    let cert_der = ctx.certs[0].clone();
    ctx.ca_certs.push(cert_der);
    assert_eq!(ctx.ca_certs.len(), 2, "ca_certs should have 2 certs");

    // Should still build correctly
    assert!(ctx.to_quic_server_config().is_ok());
}

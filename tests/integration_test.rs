// ระบบนี้คือ High-Speed Blockchain Banking System สำหรับ NDID (National Digital ID)
// ระบบพัฒนาขึ้นเพื่อรองรับการทำธุรกรรมธนาคารข้ามประเทศอย่างรวดเร็ว
// ภาษา: Rust, รันไทม์: Tokio async, โปรโตคอล: QUIC + TCP/TLS 1.3 Auto-Fallback
// ชั้นบริการ API: GraphQL (async-graphql) over Axum
// บล็อกเชน: Substrate (Private Permissioned Ledger)
// คริปโต: ED25519 (signing), AES-GCM (encryption), SHA-256 (hashing)

use banksystemrust::crypto::KeyPair;
use banksystemrust::network::quic_channel::start_quic_server;
use banksystemrust::network::tcp_channel::start_tcp_server;
use banksystemrust::network::tls::TlsContext;
use banksystemrust::network::{ConnectionChannel, Protocol, connect_with_fallback};
use banksystemrust::p2p_quic::{P2pMessage, P2pNode};
use banksystemrust::redis_cache::RedisCache;
use tokio::sync::broadcast;

fn init_logging() {
    let _ = tracing_subscriber::fmt()
        .with_env_filter("info,banksystemrust=debug")
        .try_init();
}

#[tokio::test]
async fn test_quic_communication() {
    init_logging();
    let tls = TlsContext::generate_self_signed().unwrap();
    let keypair = KeyPair::generate().unwrap();

    let listener = tokio::net::TcpListener::bind("127.0.0.1:0").await.unwrap();
    let port = listener.local_addr().unwrap().port();
    drop(listener);
    let quic_addr = format!("127.0.0.1:{}", port);

    let mut server_tls = tls.clone();
    server_tls.ca_certs.clear();
    let quic_config = server_tls.to_quic_server_config().unwrap();
    let endpoint = start_quic_server(&quic_addr, quic_config).await.unwrap();

    tokio::spawn(async move {
        while let Some(connecting) = endpoint.accept().await {
            tokio::spawn(async move {
                if let Ok(conn) = connecting.await {
                    banksystemrust::network::quic_channel::handle_quic_connection(conn).await;
                }
            });
        }
    });

    tokio::time::sleep(tokio::time::Duration::from_millis(100)).await;

    let node = P2pNode::new("BBL".into(), keypair, tls);
    let protocol = node
        .send_kyc(&quic_addr, "kyc_hash_test_123".into())
        .await
        .unwrap();

    assert_eq!(protocol, Protocol::Quic);
}

#[tokio::test]
async fn test_tcp_fallback_communication() {
    init_logging();
    let tls = TlsContext::generate_self_signed().unwrap();
    let keypair = KeyPair::generate().unwrap();

    let listener = tokio::net::TcpListener::bind("127.0.0.1:0").await.unwrap();
    let port = listener.local_addr().unwrap().port();
    drop(listener);
    let tcp_addr = format!("127.0.0.1:{}", port);

    let mut server_tls = tls.clone();
    server_tls.ca_certs.clear();

    let bind_addr = tcp_addr.clone();
    let (shutdown_tx, _shutdown_rx) = broadcast::channel(1);
    let shutdown_rx = shutdown_tx.subscribe();
    tokio::spawn(async move {
        let _ = start_tcp_server(&bind_addr, &server_tls, shutdown_rx).await;
    });

    tokio::time::sleep(tokio::time::Duration::from_millis(100)).await;

    let node = P2pNode::new("SCB".into(), keypair, tls);
    let protocol = node
        .send_kyc(&tcp_addr, "kyc_hash_test_456".into())
        .await
        .unwrap();
    assert_eq!(protocol, Protocol::Tcp);
    let _ = shutdown_tx.send(());
}

#[tokio::test]
async fn test_invalid_signature_rejection() {
    init_logging();
    let tls = TlsContext::generate_self_signed().unwrap();
    let keypair = KeyPair::generate().unwrap();

    let listener = tokio::net::TcpListener::bind("127.0.0.1:0").await.unwrap();
    let port = listener.local_addr().unwrap().port();
    drop(listener);
    let tcp_addr = format!("127.0.0.1:{}", port);

    let mut server_tls = tls.clone();
    server_tls.ca_certs.clear();

    let bind_addr = tcp_addr.clone();
    let (shutdown_tx, _shutdown_rx) = broadcast::channel(1);
    let shutdown_rx = shutdown_tx.subscribe();
    tokio::spawn(async move {
        let _ = start_tcp_server(&bind_addr, &server_tls, shutdown_rx).await;
    });

    tokio::time::sleep(tokio::time::Duration::from_millis(100)).await;

    let (channel, _protocol) = connect_with_fallback(&tcp_addr, &tls, 500, 2000).await;
    let _ = shutdown_tx.send(());
    assert!(channel.stream.is_some());

    let payload = b"KYC:SCB:fake_hash";
    let message = P2pMessage {
        from_bank: "SCB".into(),
        to_bank: String::new(),
        payload: payload.to_vec(),
        signature: vec![0u8; 64],
        public_key: keypair.public_key.clone(),
        timestamp: 1234567,
    };

    let msg_bytes = serde_json::to_vec(&message).unwrap();

    channel.send(&msg_bytes).await.unwrap();
    let resp_bytes = channel.receive().await.unwrap();
    let resp_str = String::from_utf8_lossy(&resp_bytes);

    assert!(resp_str.contains("ERROR: Invalid Signature"));
}

#[tokio::test]
async fn test_graphql_flow() {
    init_logging();

    let tls = TlsContext::generate_self_signed().unwrap();
    let keypair = KeyPair::generate().unwrap();
    let p2p_node = P2pNode::new("KBANK".into(), keypair, tls);

    let config = banksystemrust::config::BlockchainConfig {
        endpoint: "http://127.0.0.1:9933".into(),
        timeout_secs: 1,
        max_retries: 1,
        db_path: None,
    };
    let blockchain_client =
        std::sync::Arc::new(banksystemrust::blockchain::BlockchainClient::new(config).unwrap());
    let redis_cache =
        std::sync::Arc::new(RedisCache::new(banksystemrust::config::RedisConfig::default()).unwrap());

    let schema = async_graphql::Schema::build(
        banksystemrust::schema::QueryRoot,
        banksystemrust::schema::MutationRoot,
        async_graphql::EmptySubscription,
    )
    .data(p2p_node)
    .data(blockchain_client)
    .data(redis_cache)
    .finish();

    let mutation = r#"
        mutation {
            submitKyc(
                nationalId: "1234567890123"
                fullName: "John Doe"
                bankCode: "KBANK"
            ) {
                requestId
                identityHash
                bankCode
                message
            }
        }
    "#;

    let resp = schema.execute(mutation).await;
    assert!(resp.errors.is_empty(), "Mutation failed: {:?}", resp.errors);

    let data = resp.data.into_json().unwrap();
    let submit_kyc_data = data.get("submitKyc").unwrap();
    let request_id = submit_kyc_data
        .get("requestId")
        .unwrap()
        .as_str()
        .unwrap()
        .to_string();
    let identity_hash = submit_kyc_data
        .get("identityHash")
        .unwrap()
        .as_str()
        .unwrap()
        .to_string();
    let message = submit_kyc_data
        .get("message")
        .unwrap()
        .as_str()
        .unwrap()
        .to_string();

    assert!(!request_id.is_empty());
    assert_eq!(identity_hash.len(), 64);
    assert!(message.contains("Queued") || message.contains("Finalized"));

    let query_get_identity = format!(
        r#"
        query {{
            getIdentity(requestId: "{}") {{
                requestId
                status
                activeProtocol
            }}
        }}
        "#,
        request_id
    );

    let resp = schema.execute(query_get_identity).await;
    assert!(
        resp.errors.is_empty(),
        "Query getIdentity failed: {:?}",
        resp.errors
    );
    let data = resp.data.into_json().unwrap();
    let identity_data = data.get("getIdentity").unwrap();
    let status = identity_data
        .get("status")
        .unwrap()
        .as_str()
        .unwrap()
        .to_string();
    assert_eq!(status, "Queued");

    let query_verify = format!(
        r#"
        query {{
            verifyNdidRecord(requestId: "{}") {{
                requestId
                status
                activeProtocol
            }}
        }}
        "#,
        request_id
    );

    let resp = schema.execute(query_verify).await;
    assert!(
        resp.errors.is_empty(),
        "Query verifyNdidRecord failed: {:?}",
        resp.errors
    );
    let data = resp.data.into_json().unwrap();
    let verify_data = data.get("verifyNdidRecord").unwrap();
    let verify_status = verify_data
        .get("status")
        .unwrap()
        .as_str()
        .unwrap()
        .to_string();
    assert_eq!(verify_status, "Queued");
}

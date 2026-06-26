---
type: function
module: "p2p_quic.rs"
parent: ""
tags: [rust, function]
---

# Function: test_send_kyc_fallback

**Defined in:** [p2p_quic.rs](file:///home/lokis/Documents/banksystemrust/src/p2p_quic.rs#L237)

## Signature
```rust
async fn test_send_kyc_fallback()
```

## Implementation
```rust
async fn test_send_kyc_fallback() {
        let listener = tokio::net::TcpListener::bind("127.0.0.1:0").await.unwrap();
        let addr = listener.local_addr().unwrap();
        drop(listener);

        let node = test_node("SCB");
        let mut server_tls = node.tls.clone();
        server_tls.ca_certs.clear();
        let bind_addr = addr.to_string();
        let (shutdown_tx, _shutdown_rx) = tokio::sync::broadcast::channel(1);
        let shutdown_rx = shutdown_tx.subscribe();
        tokio::spawn(async move {
            let _ =
                crate::network::tcp_channel::start_tcp_server(&bind_addr, &server_tls, shutdown_rx)
                    .await;
        });

        tokio::time::sleep(tokio::time::Duration::from_millis(100)).await;

        let proto = node
            .send_kyc(&addr.to_string(), "hash123".into())
            .await
            .unwrap();
        assert_eq!(proto, Protocol::Tcp);
        let _ = shutdown_tx.send(());
    }
```

## Calls & References
- [[start_tcp_server|start::tcp_server]]
- [[test_node|test::node]]
- [[Protocol|Protocol]]


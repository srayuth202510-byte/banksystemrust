---
type: function
module: "network/quic_channel.rs"
parent: ""
tags: [rust, function]
---

# Function: handle_quic_connection

**Defined in:** [network/quic_channel.rs](file:///home/lokis/Documents/banksystemrust/src/network/quic_channel.rs#L72)

## Signature
```rust
pub async fn handle_quic_connection(connection: quinn::Connection)
```

## Implementation
```rust
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
```

## Calls & References
- [[process_p2p_message|process::p2p_message]]

## Called By
- [[start_quic_server|start::quic_server]]


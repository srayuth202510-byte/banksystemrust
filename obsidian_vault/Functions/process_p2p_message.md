---
type: function
module: "network/mod.rs"
parent: ""
tags: [rust, function]
---

# Function: process_p2p_message

**Defined in:** [network/mod.rs](file:///home/lokis/Documents/banksystemrust/src/network/mod.rs#L190)

## Signature
```rust
pub fn process_p2p_message(buf: &[u8]) -> String
```

## Implementation
```rust
pub fn process_p2p_message(buf: &[u8]) -> String {
    use crate::crypto;
    use crate::p2p_quic::P2pMessage;

    if let Ok(msg) = serde_json::from_slice::<P2pMessage>(buf) {
        let payload_clone = msg.payload.clone();
        let signed = crypto::SignedPayload {
            payload: msg.payload.clone(),
            signature: msg.signature.clone(),
            public_key: msg.public_key.clone(),
        };
        match crypto::verify(&signed) {
            Ok(true) => {
                let payload_str = String::from_utf8(payload_clone.clone()).unwrap_or_else(|_| {
                    warn!("P2P payload contains invalid UTF-8, logging hex");
                    hex::encode(&payload_clone)
                });
                info!(from = %msg.from_bank, payload = %payload_str, "P2P signature verified");
                crate::metrics::p2p_messages()
                    .with_label_values(&["in", &msg.from_bank, "Success"])
                    .inc();
                format!("ACK:{}", payload_str)
            }
            _ => {
                warn!("P2P signature verification failed");
                crate::metrics::p2p_messages()
                    .with_label_values(&["in", &msg.from_bank, "InvalidSignature"])
                    .inc();
                "ERROR: Invalid Signature".to_string()
            }
        }
    } else {
        warn!("Received non-JSON P2P message, logging hex");
        crate::metrics::p2p_messages()
            .with_label_values(&["in", "unknown", "InvalidFormat"])
            .inc();
        hex::encode(buf)
    }
}
```

## Calls & References
- [[p2p_messages|p2p::messages]]
- [[SignedPayload|SignedPayload]]
- [[P2pMessage|P2pMessage]]
- [[verify|verify]]

## Called By
- [[handle_quic_connection|handle::quic_connection]]
- [[handle_tcp_connection|handle::tcp_connection]]


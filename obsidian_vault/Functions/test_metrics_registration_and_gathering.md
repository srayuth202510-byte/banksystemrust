---
type: function
module: "metrics.rs"
parent: ""
tags: [rust, function]
---

# Function: test_metrics_registration_and_gathering

**Defined in:** [metrics.rs](file:///home/lokis/Documents/banksystemrust/src/metrics.rs#L84)

## Signature
```rust
fn test_metrics_registration_and_gathering()
```

## Implementation
```rust
fn test_metrics_registration_and_gathering() {
        kyc_requests().with_label_values(&["SCB", "success"]).inc();
        p2p_messages()
            .with_label_values(&["out", "BBL", "sent"])
            .inc();
        blockchain_retries().with_label_values(&["failed"]).inc();

        let output = gather_metrics().expect("Failed to gather metrics");

        assert!(output.contains("ndid_kyc_requests_total"));
        assert!(output.contains("bank_code=\"SCB\""));
        assert!(output.contains("status=\"success\""));

        assert!(output.contains("ndid_p2p_messages_total"));
        assert!(output.contains("direction=\"out\""));

        assert!(output.contains("ndid_blockchain_retries_total"));
    }
```

## Calls & References
- [[p2p_messages|p2p::messages]]
- [[kyc_requests|kyc::requests]]
- [[gather_metrics|gather::metrics]]
- [[blockchain_retries|blockchain::retries]]


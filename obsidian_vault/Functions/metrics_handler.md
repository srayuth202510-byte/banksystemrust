---
type: function
module: "main.rs"
parent: ""
tags: [rust, function]
---

# Function: metrics_handler

**Defined in:** [main.rs](file:///home/lokis/Documents/banksystemrust/src/main.rs#L109)

## Signature
```rust
async fn metrics_handler() -> impl IntoResponse
```

## Implementation
```rust
async fn metrics_handler() -> impl IntoResponse {
    match banksystemrust::metrics::gather_metrics() {
        Ok(metrics) => (StatusCode::OK, metrics),
        Err(e) => (
            StatusCode::INTERNAL_SERVER_ERROR,
            format!("Failed to gather metrics: {e}"),
        ),
    }
}
```

## Calls & References
- [[gather_metrics|gather::metrics]]


---
type: function
module: "main.rs"
parent: ""
tags: [rust, function]
---

# Function: create_shutdown_signal

**Defined in:** [main.rs](file:///home/lokis/Documents/banksystemrust/src/main.rs#L120)

## Signature
```rust
fn create_shutdown_signal() -> (
    broadcast::Sender<()>,
    impl std::future::Future<Output = ()> + Send + 'static,
)
```

## Implementation
```rust
fn create_shutdown_signal() -> (
    broadcast::Sender<()>,
    impl std::future::Future<Output = ()> + Send + 'static,
) {
    let (tx, _rx) = broadcast::channel(1);
    let tx_for_future = tx.clone();

    let ctrl_c = async {
        tokio::signal::ctrl_c()
            .await
            .expect("failed to install Ctrl+C handler");
    };

    #[cfg(unix)]
    let terminate = async {
        tokio::signal::unix::signal(tokio::signal::unix::SignalKind::terminate())
            .expect("failed to install SIGTERM handler")
            .recv()
            .await;
    };

    #[cfg(not(unix))]
    let terminate = std::future::pending::<()>();

    let shutdown_future = async move {
        tokio::select! {
            _ = ctrl_c => { info!("Received Ctrl+C, shutting down..."); }
            _ = terminate => { info!("Received SIGTERM, shutting down..."); }
        }
        let _ = tx_for_future.send(());
    };

    (tx, shutdown_future)
}
```

## Called By
- [[main|main]]


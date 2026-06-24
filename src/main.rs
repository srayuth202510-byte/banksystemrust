// ภาษา: Rust, รันไทม์: Tokio async, โปรโตคอล: QUIC + TCP/TLS 1.3 Auto-Fallback
// ชั้นบริการ API: GraphQL (async-graphql) over Axum
// บล็อกเชน: Substrate (Private Permissioned Ledger)
// คริปโต: ED25519 (signing), AES-GCM (encryption), SHA-256 (hashing)

// ชั้นบริการ API: GraphQL (async-graphql) over Axum
// บล็อกเชน: Substrate (Private Permissioned Ledger)
// คริปโต: ED25519 (signing), AES-GCM (encryption), SHA-256 (hashing)

// ชั้นบริการ API: GraphQL (async-graphql) over Axum
// บล็อกเชน: Substrate (Private Permissioned Ledger)
// คริปโต: ED25519 (signing), AES-GCM (encryption), SHA-256 (hashing)

// ชั้นบริการ API: GraphQL (async-graphql) over Axum
// บล็อกเชน: Substrate (Private Permissioned Ledger)
// คริปโต: ED25519 (signing), AES-GCM (encryption), SHA-256 (hashing)

use std::path::PathBuf;
use std::time::Duration;

use async_graphql::{http::GraphiQLSource, EmptySubscription};
use async_graphql_axum::{GraphQLRequest, GraphQLResponse};
use axum::{
    error_handling::HandleErrorLayer,
    extract::State,
    http::StatusCode,
    response::{Html, IntoResponse},
    routing::get,
    Router,
};
use clap::Parser;
use tower::{buffer::BufferLayer, limit::RateLimitLayer, BoxError, ServiceBuilder};
use tokio::net::TcpListener;
use tracing::{error, info};
use tracing_subscriber::EnvFilter;

use banksystemrust::blockchain::BlockchainClient;
use banksystemrust::config::AppConfig;
use banksystemrust::crypto::KeyPair;
use banksystemrust::network::quic_channel;
use banksystemrust::network::tls::TlsContext;
use banksystemrust::p2p_quic::P2pNode;
use banksystemrust::schema::{MutationRoot, QueryRoot};

#[derive(Parser)]
#[command(name = "ndid-gateway", version, about = "NDID Banking System Gateway")]
struct Cli {
    #[arg(short, long, default_value = "config/default.toml")]
    config: String,
}

async fn graphiql() -> impl IntoResponse {
    Html(GraphiQLSource::build().endpoint("/graphql").finish())
}

async fn graphql_handler(
    schema: State<async_graphql::Schema<QueryRoot, MutationRoot, EmptySubscription>>,
    req: GraphQLRequest,
) -> GraphQLResponse {
    schema.execute(req.into_inner()).await.into()
}

async fn health() -> impl IntoResponse {
    "OK"
}

async fn shutdown_signal() {
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

    tokio::select! {
        _ = ctrl_c => { info!("Received Ctrl+C, shutting down..."); }
        _ = terminate => { info!("Received SIGTERM, shutting down..."); }
    }
}

async fn start_quic_server(config: &AppConfig, tls: &TlsContext) {
    let quic_addr = format!("0.0.0.0:{}", config.network.quic_port);
    match tls.to_quic_server_config() {
        Ok(server_config) => {
            match quic_channel::start_quic_server(&quic_addr, server_config).await {
                Ok(endpoint) => {
                    info!(addr = %quic_addr, "QUIC server started");
                    while let Some(connecting) = endpoint.accept().await {
                        tokio::spawn(async move {
                            match connecting.await {
                                Ok(connection) => {
                                    quic_channel::handle_quic_connection(connection).await;
                                }
                                Err(e) => {
                                    error!(error = %e, "QUIC accept handshake failed");
                                }
                            }
                        });
                    }
                }
                Err(e) => {
                    error!(error = %e, "Failed to start QUIC server");
                }
            }
        }
        Err(e) => {
            error!(error = %e, "Failed to build QUIC server config");
        }
    }
}

#[tokio::main]
async fn main() {
    let cli = Cli::parse();

    let config = AppConfig::load(Some(PathBuf::from(&cli.config)))
        .unwrap_or_else(|e| {
            eprintln!("Failed to load config: {e}");
            std::process::exit(1);
        });

    tracing_subscriber::fmt()
        .with_env_filter(
            EnvFilter::try_from_default_env()
                .unwrap_or_else(|_| EnvFilter::new(&config.logging.level)),
        )
        .json()
        .init();

    info!("NDID Banking System starting...");
    info!(
        endpoint = %config.blockchain.endpoint,
        "Blockchain configured"
    );

    let tls = TlsContext::generate_self_signed().unwrap_or_else(|e| {
        error!(error = %e, "Failed to generate TLS certificates");
        std::process::exit(1);
    });
    info!("TLS certificates generated");

    let keypair = match KeyPair::generate() {
        Ok(kp) => {
            info!("Crypto keys generated successfully");
            kp
        }
        Err(e) => {
            error!(error = %e, "Failed to generate crypto keys");
            std::process::exit(1);
        }
    };

    let p2p_node = P2pNode::new("BBL".into(), keypair, tls.clone());

    let blockchain_client = std::sync::Arc::new(
        BlockchainClient::new(config.blockchain.clone()).unwrap_or_else(|e| {
            error!(error = %e, "Failed to initialize blockchain client");
            std::process::exit(1);
        })
    );
    
    // Background Retry Worker for Substrate node
    let worker_client = blockchain_client.clone();
    tokio::spawn(async move {
        let mut interval = tokio::time::interval(std::time::Duration::from_secs(5));
        loop {
            interval.tick().await;
            worker_client.retry_all_queued().await;
        }
    });

    let schema = async_graphql::Schema::build(QueryRoot, MutationRoot, EmptySubscription)
        .data(p2p_node)
        .data(blockchain_client)
        .finish();

    let quic_config = config.clone();
    let quic_tls = tls.clone();
    tokio::spawn(async move {
        start_quic_server(&quic_config, &quic_tls).await;
    });

    let tcp_config = config.clone();
    let tcp_tls = tls.clone();
    tokio::spawn(async move {
        let addr = format!("0.0.0.0:{}", tcp_config.network.tcp_port);
        if let Err(e) = banksystemrust::network::tcp_channel::start_tcp_server(&addr, &tcp_tls).await {
            error!(error = %e, "TCP server failed");
        }
    });

    let app = Router::new()
        .route(
            &config.server.graphql_endpoint,
            get(graphiql).post(graphql_handler),
        )
        .route("/health", get(health))
        .layer(
            ServiceBuilder::new()
                .layer(HandleErrorLayer::new(|err: BoxError| async move {
                    (
                        StatusCode::INTERNAL_SERVER_ERROR,
                        format!("Unhandled error: {}", err),
                    )
                }))
                .layer(BufferLayer::new(1024))
                .layer(RateLimitLayer::new(1000, Duration::from_secs(1))),
        )
        .with_state(schema);

    let addr = format!("{}:{}", config.server.host, config.server.port);
    info!(addr = %addr, "GraphQL Gateway starting");

    let listener = TcpListener::bind(&addr).await.unwrap_or_else(|e| {
        error!(error = %e, "Failed to bind to {addr}");
        std::process::exit(1);
    });

    axum::serve(listener, app)
        .with_graceful_shutdown(shutdown_signal())
        .await
        .unwrap_or_else(|e| {
            error!(error = %e, "Server error");
            std::process::exit(1);
        });

    info!("Server shut down gracefully");
}

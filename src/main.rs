// ระบบนี้คือ High-Speed Blockchain Banking System สำหรับ NDID (National Digital ID)
// ระบบพัฒนาขึ้นเพื่อรองรับการทำธุรกรรมธนาคารข้ามประเทศอย่างรวดเร็ว
// ภาษา: Rust, รันไทม์: Tokio async, โปรโตคอล: QUIC + TCP/TLS 1.3 Auto-Fallback
// ชั้นบริการ API: GraphQL (async-graphql) over Axum
// บล็อกเชน: Substrate (Private Permissioned Ledger)
// คริปโต: ED25519 (signing), AES-GCM (encryption), SHA-256 (hashing)

use std::path::PathBuf;
use std::time::Duration;

use async_graphql::{EmptySubscription, http::GraphiQLSource};
use async_graphql_axum::{GraphQLRequest, GraphQLResponse};
use axum::{
    Router,
    error_handling::HandleErrorLayer,
    extract::State,
    http::StatusCode,
    response::{Html, IntoResponse},
    routing::get,
};
use clap::Parser;
use opentelemetry_sdk::trace::TracerProvider;
use opentelemetry_stdout::SpanExporter;
use tokio::net::TcpListener;
use tokio::sync::broadcast;
use tower::{BoxError, ServiceBuilder, buffer::BufferLayer, limit::RateLimitLayer};
use tracing::{error, info};
use tracing_subscriber::EnvFilter;
use tracing_subscriber::layer::SubscriberExt;

use banksystemrust::blockchain::BlockchainClient;
use banksystemrust::config::AppConfig;
use banksystemrust::crypto::KeyPair;
use banksystemrust::network::quic_channel;
use banksystemrust::network::tls::TlsContext;
use banksystemrust::p2p_quic::P2pNode;
use banksystemrust::redis_cache::RedisCache;
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

async fn metrics_handler() -> impl IntoResponse {
    match banksystemrust::metrics::gather_metrics() {
        Ok(metrics) => (StatusCode::OK, metrics),
        Err(e) => (
            StatusCode::INTERNAL_SERVER_ERROR,
            format!("Failed to gather metrics: {e}"),
        ),
    }
}

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

async fn start_quic_server(
    config: &AppConfig,
    tls: &TlsContext,
    mut shutdown_rx: broadcast::Receiver<()>,
) {
    let quic_addr = format!("0.0.0.0:{}", config.network.quic_port);
    match tls.to_quic_server_config() {
        Ok(server_config) => {
            match quic_channel::start_quic_server(&quic_addr, server_config).await {
                Ok(endpoint) => {
                    info!(addr = %quic_addr, "QUIC server started");
                    loop {
                        tokio::select! {
                            _ = shutdown_rx.recv() => {
                                info!("QUIC server shutting down");
                                break;
                            }
                            connecting = endpoint.accept() => {
                                if let Some(connecting) = connecting {
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
                                } else {
                                    break;
                                }
                            }
                        }
                    }
                    endpoint.close(0u32.into(), b"shutdown");
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

    let config = AppConfig::load(Some(PathBuf::from(&cli.config))).unwrap_or_else(|e| {
        eprintln!("Failed to load config: {e}");
        std::process::exit(1);
    });

    // Initialize OpenTelemetry provider with stdout exporter
    let tracer_provider = TracerProvider::builder()
        .with_simple_exporter(SpanExporter::default())
        .build();
    opentelemetry::global::set_tracer_provider(tracer_provider.clone());

    use opentelemetry::trace::TracerProvider as _;
    let tracer = tracer_provider.tracer("ndid-gateway");
    let telemetry_layer = tracing_opentelemetry::layer().with_tracer(tracer);

    let subscriber = tracing_subscriber::Registry::default()
        .with(
            EnvFilter::try_from_default_env()
                .unwrap_or_else(|_| EnvFilter::new(&config.logging.level)),
        )
        .with(tracing_subscriber::fmt::layer().json())
        .with(telemetry_layer);

    if let Err(e) = tracing::subscriber::set_global_default(subscriber) {
        eprintln!("Failed to set tracing subscriber: {e}");
    }

    info!("NDID Banking System starting...");
    info!(
        endpoint = %config.blockchain.endpoint,
        "Blockchain configured"
    );

    let production_mode = matches!(
        std::env::var("NDID_ENV"),
        Ok(value) if value.eq_ignore_ascii_case("production")
    );

    let tls = if let (Some(cert_path), Some(key_path)) =
        (&config.network.cert_path, &config.network.key_path)
    {
        let mut ctx = TlsContext::load(cert_path, key_path).unwrap_or_else(|e| {
            error!(error = %e, "Failed to load TLS cert/key");
            std::process::exit(1);
        });
        if let Some(ca_path) = &config.network.ca_cert_path {
            ctx.add_ca_cert(ca_path).unwrap_or_else(|e| {
                error!(error = %e, "Failed to load CA certificate");
                std::process::exit(1);
            });
        }
        info!("TLS certificates loaded from files");
        ctx
    } else if production_mode {
        error!("Production mode requires network.cert_path and network.key_path");
        std::process::exit(1);
    } else {
        let ctx = TlsContext::generate_self_signed().unwrap_or_else(|e| {
            error!(error = %e, "Failed to generate TLS certificates");
            std::process::exit(1);
        });
        info!("Self-signed TLS certificates generated");
        ctx
    };

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

    let mut p2p_node = P2pNode::new(config.bank_code.clone(), keypair, tls.clone());
    p2p_node = p2p_node.with_load_balancer(config.network.load_balancer.strategy.clone());
    for peer in &config.network.peers {
        p2p_node.add_peer(peer.clone());
    }

    let blockchain_client = std::sync::Arc::new(
        BlockchainClient::new(config.blockchain.clone()).unwrap_or_else(|e| {
            error!(error = %e, "Failed to initialize blockchain client");
            std::process::exit(1);
        }),
    );
    let redis_cache = std::sync::Arc::new(RedisCache::new(config.redis.clone()).unwrap_or_else(
        |e| {
            error!(error = %e, "Failed to initialize Redis cache");
            std::process::exit(1);
        },
    ));

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
        .data(redis_cache)
        .finish();

    let (shutdown_tx, shutdown_future) = create_shutdown_signal();
    let quic_shutdown_rx = shutdown_tx.subscribe();
    let tcp_shutdown_rx = shutdown_tx.subscribe();

    let quic_config = config.clone();
    let quic_tls = tls.clone();
    tokio::spawn(async move {
        start_quic_server(&quic_config, &quic_tls, quic_shutdown_rx).await;
    });

    let tcp_config = config.clone();
    let tcp_tls = tls.clone();
    tokio::spawn(async move {
        let addr = format!("0.0.0.0:{}", tcp_config.network.tcp_port);
        if let Err(e) =
            banksystemrust::network::tcp_channel::start_tcp_server(&addr, &tcp_tls, tcp_shutdown_rx)
                .await
        {
            error!(error = %e, "TCP server failed");
        }
    });

    let app = Router::new()
        .route(
            &config.server.graphql_endpoint,
            get(graphiql).post(graphql_handler),
        )
        .route("/health", get(health))
        .route("/metrics", get(metrics_handler))
        .layer(
            ServiceBuilder::new()
                .layer(HandleErrorLayer::new(|err: BoxError| async move {
                    (
                        StatusCode::INTERNAL_SERVER_ERROR,
                        format!("Unhandled error: {}", err),
                    )
                }))
                .layer(BufferLayer::new(1024))
                .layer(RateLimitLayer::new(
                    config.server.rate_limit.requests_per_second,
                    Duration::from_secs(1),
                )),
        )
        .with_state(schema);

    let addr = format!("{}:{}", config.server.host, config.server.port);
    info!(addr = %addr, "GraphQL Gateway starting");

    let listener = TcpListener::bind(&addr).await.unwrap_or_else(|e| {
        error!(error = %e, "Failed to bind to {addr}");
        std::process::exit(1);
    });

    axum::serve(listener, app)
        .with_graceful_shutdown(shutdown_future)
        .await
        .unwrap_or_else(|e| {
            error!(error = %e, "Server error");
            std::process::exit(1);
        });

    if let Err(e) = tracer_provider.shutdown() {
        error!(error = %e, "Failed to shutdown tracer provider");
    }

    info!("Server shut down gracefully");
}

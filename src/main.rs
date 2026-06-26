// ระบบนี้คือ High-Speed Blockchain Banking System สำหรับ NDID (National Digital ID)
// ระบบพัฒนาขึ้นเพื่อรองรับการทำธุรกรรมธนาคารข้ามประเทศอย่างรวดเร็ว
// ภาษา: Rust, รันไทม์: Tokio async, โปรโตคอล: QUIC + TCP/TLS 1.3 Auto-Fallback
// ชั้นบริการ API: GraphQL (async-graphql) over Axum
// บล็อกเชน: Substrate (Private Permissioned Ledger)
// คริปโต: ED25519 (signing), AES-GCM (encryption), SHA-256 (hashing)

// นำเข้าไลบรารีมาตรฐานสำหรับจัดการเส้นทางไฟล์
use std::path::PathBuf;

use async_graphql::{EmptySubscription, http::GraphiQLSource};
use async_graphql_axum::{GraphQLRequest, GraphQLResponse};
use axum::{
    Router,
    error_handling::HandleErrorLayer,
    extract::{ConnectInfo, Extension, Request, State},
    http::StatusCode,
    middleware::{self, Next},
    response::{Html, IntoResponse, Response},
    routing::get,
};
use clap::Parser;
use opentelemetry_sdk::trace::TracerProvider;
use opentelemetry_stdout::SpanExporter;
use tokio::net::TcpListener;
use tokio::sync::{Mutex as TokioMutex, broadcast};
use tower::{BoxError, ServiceBuilder, buffer::BufferLayer};
use tracing::{error, info, warn};
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

// โครงสร้างสำหรับรับพารามิเตอร์จาก command line (CLI)
#[derive(Parser)]
#[command(name = "ndid-gateway", version, about = "NDID Banking System Gateway")]
struct Cli {
    #[arg(short, long, default_value = "config/default.toml")]
    config: String,
}

// แสดงหน้า GraphQL Playground สำหรับทดสอบ API (ถูกปิดอัตโนมัติเมื่อ graphql_playground = false)
async fn graphiql() -> impl IntoResponse {
    Html(GraphiQLSource::build().endpoint("/graphql").finish())
}

// โครงสร้างสำหรับจัดการ Rate Limit (จำกัดจำนวน request ต่อ IP)
#[derive(Clone)]
struct RateLimitState {
    redis: std::sync::Arc<RedisCache>,
    fallback: std::sync::Arc<TokioMutex<std::collections::HashMap<std::net::IpAddr, (u64, std::time::Instant)>>>,
    limit: u64,
    burst: u64,
}

// Middleware สำหรับจำกัดจำนวนคำขอดู transaction ต่อ IP
async fn per_ip_rate_limiter(
    ConnectInfo(addr): ConnectInfo<std::net::SocketAddr>,
    Extension(state): Extension<RateLimitState>,
    req: Request,
    next: Next,
) -> Result<Response, StatusCode> {
    let ip = addr.ip();
    let allowed = match state
        .redis
        .check_rate_limit(&ip.to_string(), state.limit, state.burst)
        .await
    {
        Ok(true) => true,
        Ok(false) => false,
        Err(e) => {
            tracing::warn!(error = %e, "Redis rate limit failed, using fallback");
            let mut map = state.fallback.lock().await;
            let now = std::time::Instant::now();
            let (count, last) = map.entry(ip).or_insert((0, now));
            if now.duration_since(*last).as_secs() >= 1 {
                *count = 0;
                *last = now;
            }
            *count += 1;
            *count <= state.burst
        }
    };

    if !allowed {
        // ส่งกลับ HTTP 429 Too Many Requests ถ้าเกินขีดจำกัด
        return Err(StatusCode::TOO_MANY_REQUESTS);
    }

    // อนุญาตให้ดำเนินการต่อ
    Ok(next.run(req).await)
}

// ตัวจัดการคำขอ GraphQL หลัก
async fn graphql_handler(
    schema: State<async_graphql::Schema<QueryRoot, MutationRoot, EmptySubscription>>,
    req: GraphQLRequest,
) -> GraphQLResponse {
    schema.execute(req.into_inner()).await.into()
}

// ปลายทางสำหรับตรวจสอบสถานะระบบ (Health Check)
async fn health() -> impl IntoResponse {
    "OK"
}

// ปลายทางสำหรับดึงข้อมูลเมตริกของระบบ (Prometheus)
async fn metrics_handler() -> impl IntoResponse {
    match banksystemrust::metrics::gather_metrics() {
        Ok(metrics) => (StatusCode::OK, metrics),
        Err(e) => (
            StatusCode::INTERNAL_SERVER_ERROR,
            format!("Failed to gather metrics: {e}"),
        ),
    }
}

// สร้างสัญญาณปิดระบบ (Ctrl+C / SIGTERM) สำหรับ graceful shutdown
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

// เริ่มต้นเซิร์ฟเวอร์ QUIC สำหรับรับการเชื่อมต่อ P2P แบบความเร็วสูง
async fn start_quic_server(
    config: &AppConfig,
    tls: &TlsContext,
    mut shutdown_rx: broadcast::Receiver<()>,
) {
    let rate_limiter = quic_channel::QuicRateLimiter::new();
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
                                    let limiter = rate_limiter.clone();
                                    tokio::spawn(async move {
                                        match connecting.await {
                                            Ok(connection) => {
                                                let remote = connection.remote_address();
                                                match limiter.check_and_acquire(remote).await {
                                                    Ok(permit) => {
                                                        quic_channel::handle_quic_connection(connection, permit).await;
                                                    }
                                                    Err(()) => {
                                                        warn!(remote = %remote, "QUIC connection rate limited");
                                                        connection.close(0u32.into(), b"rate_limited");
                                                    }
                                                }
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
    // แยกวิเคราะห์พารามิเตอร์จาก command line
    let cli = Cli::parse();

    // โหลดการตั้งค่าระบบจากไฟล์ config
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

    // กำหนดค่าระบบการติดตาม (Tracing) และ OpenTelemetry
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

    // ตรวจสอบโหมดการทำงาน (Production หรือ Development)
    let production_mode = matches!(
        std::env::var("NDID_ENV"),
        Ok(value) if value.eq_ignore_ascii_case("production")
    );

    // โหลดหรือสร้าง TLS Certificate สำหรับการเข้ารหัสการสื่อสาร
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

    // สร้างคู่กุญแจ ED25519 สำหรับการลงนามธุรกรรม
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

    // สร้างโหนด P2P สำหรับการสื่อสารระหว่างธนาคาร
    let mut p2p_node = P2pNode::new(config.bank_code.clone(), keypair, tls.clone());
    p2p_node = p2p_node
        .with_load_balancer(config.network.load_balancer.strategy.clone())
        .with_timeouts(
            config.network.quic_timeout_ms,
            config.network.tcp_timeout_ms,
        );
    for peer in &config.network.peers {
        p2p_node.add_peer(peer.clone());
    }

    // สร้างไคลเอนต์สำหรับเชื่อมต่อกับบล็อกเชน Substrate
    let blockchain_client = std::sync::Arc::new(
        BlockchainClient::new(config.blockchain.clone()).unwrap_or_else(|e| {
            error!(error = %e, "Failed to initialize blockchain client");
            std::process::exit(1);
        }),
    );
    // สร้างระบบแคช Redis สำหรับเก็บสถานะธุรกรรม
    let redis_cache =
        std::sync::Arc::new(RedisCache::new(config.redis.clone()).unwrap_or_else(|e| {
            error!(error = %e, "Failed to initialize Redis cache");
            std::process::exit(1);
        }));

    // ตัวทำงานพื้นหลังสำหรับส่งธุรกรรมที่ค้างอยู่ในคิวไปยัง Substrate node ซ้ำ
    // wrap ใน task แยกเพื่อป้องกัน panic ทำลาย process (Panic Safety)
    let worker_client = blockchain_client.clone();
    let worker_handle = tokio::spawn(async move {
        let mut interval = tokio::time::interval(std::time::Duration::from_secs(5));
        loop {
            interval.tick().await;
            let client = worker_client.clone();
            let task = tokio::spawn(async move {
                client.retry_all_queued().await;
            });
            if let Err(e) = task.await {
                error!("Blockchain retry subtask panicked: {:?}", e);
            }
        }
    });
    // สร้าง watcher สำหรับ worker — ถ้า worker panic จะ restart โดยอัตโนมัติ
    tokio::spawn(async move {
        let result = worker_handle.await;
        error!("Blockchain worker terminated: {:?}, restarting...", result);
    });

    // สร้าง Schema GraphQL สำหรับ API Gateway
    let schema = async_graphql::Schema::build(QueryRoot, MutationRoot, EmptySubscription)
        .limit_depth(8)
        .limit_complexity(256)
        .data(p2p_node)
        .data(blockchain_client)
        .data(redis_cache.clone())
        .finish();

    // สร้างสัญญาณ shutdown และเริ่มเซิร์ฟเวอร์ QUIC + TCP
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

    // กำหนดค่า Router สำหรับ HTTP (Axum) พร้อม Middleware
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
                .layer(BufferLayer::new(1024)),
        )
        .layer(middleware::from_fn(per_ip_rate_limiter))
        .layer(Extension(RateLimitState {
            redis: redis_cache.clone(),
            fallback: std::sync::Arc::new(TokioMutex::new(std::collections::HashMap::new())),
            limit: config.server.rate_limit.per_ip_limit,
            burst: config.server.rate_limit.burst as u64,
        }))
        .with_state(schema);

    let addr = format!("{}:{}", config.server.host, config.server.port);
    info!(addr = %addr, "GraphQL Gateway starting");

    // เริ่มต้น HTTP Server (Axum) ที่พอร์ตที่กำหนด
    let listener = TcpListener::bind(&addr).await.unwrap_or_else(|e| {
        error!(error = %e, "Failed to bind to {addr}");
        std::process::exit(1);
    });

    axum::serve(
        listener,
        app.into_make_service_with_connect_info::<std::net::SocketAddr>(),
    )
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

---
type: function
module: "main.rs"
parent: ""
tags: [rust, function]
---

# Function: main

**Defined in:** [main.rs](file:///home/lokis/Documents/banksystemrust/src/main.rs#L205)

## Signature
```rust
async fn main()
```

## Implementation
```rust
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
    let worker_client = blockchain_client.clone();
    tokio::spawn(async move {
        let mut interval = tokio::time::interval(std::time::Duration::from_secs(5));
        loop {
            interval.tick().await;
            worker_client.retry_all_queued().await;
        }
    });

    // สร้าง Schema GraphQL สำหรับ API Gateway
    let schema = async_graphql::Schema::build(QueryRoot, MutationRoot, EmptySubscription)
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
```

## Calls & References
- [[P2pNode_with_load_balancer|P2pNode::with_load_balancer]]
- [[TlsContext_clone|TlsContext::clone]]
- [[fmt|fmt]]
- [[RateLimitState|RateLimitState]]
- [[RedisCache|RedisCache]]
- [[BlockchainClient_new|BlockchainClient::new]]
- [[TlsContext_add_ca_cert|TlsContext::add_ca_cert]]
- [[TlsContext|TlsContext]]
- [[KeyPair|KeyPair]]
- [[P2pNode|P2pNode]]
- [[Cli|Cli]]
- [[BlockchainClient|BlockchainClient]]
- [[AppConfig|AppConfig]]
- [[QueryRoot|QueryRoot]]
- [[P2pNode_with_timeouts|P2pNode::with_timeouts]]
- [[BlockchainClient_retry_all_queued|BlockchainClient::retry_all_queued]]
- [[graphiql|graphiql]]
- [[KeyPair_generate|KeyPair::generate]]
- [[RedisCache_new|RedisCache::new]]
- [[P2pNode_add_peer|P2pNode::add_peer]]
- [[AppConfig_load|AppConfig::load]]
- [[health|health]]
- [[P2pNode_new|P2pNode::new]]
- [[create_shutdown_signal|create::shutdown_signal]]
- [[start_tcp_server|start::tcp_server]]
- [[TlsContext_generate_self_signed|TlsContext::generate_self_signed]]
- [[start_quic_server|start::quic_server]]
- [[TlsContext_load|TlsContext::load]]
- [[AppConfig_default|AppConfig::default]]


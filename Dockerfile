# syntax=docker/dockerfile:1
FROM rust:1.88-slim-bookworm AS builder

RUN apt-get update && apt-get install -y pkg-config libssl-dev && rm -rf /var/lib/apt/lists/*

WORKDIR /app
COPY Cargo.toml Cargo.lock* ./
COPY src src
COPY config config

RUN --mount=type=cache,target=/usr/local/cargo/registry \
    --mount=type=cache,target=/app/target \
    cargo build --release && \
    cp target/release/ndid-gateway /ndid-gateway

FROM debian:bookworm-slim
RUN apt-get update && apt-get install -y ca-certificates curl && rm -rf /var/lib/apt/lists/*
RUN groupadd -r ndid && useradd -r -g ndid -d /app -s /sbin/nologin ndid

WORKDIR /app
COPY --from=builder /ndid-gateway /usr/local/bin/ndid-gateway
COPY --from=builder /app/config /app/config

RUN mkdir -p /var/log/ndid && chown -R ndid:ndid /var/log/ndid

USER ndid
EXPOSE 8080
EXPOSE 4433/udp
EXPOSE 8443

HEALTHCHECK --interval=30s --timeout=3s --start-period=5s --retries=3 \
    CMD curl -sf http://localhost:8080/health || exit 1

ENTRYPOINT ["ndid-gateway"]
CMD ["--config", "/app/config/default.toml"]

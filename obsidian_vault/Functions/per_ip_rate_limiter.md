---
type: function
module: "main.rs"
parent: ""
tags: [rust, function]
---

# Function: per_ip_rate_limiter

**Defined in:** [main.rs](file:///home/lokis/Documents/banksystemrust/src/main.rs#L63)

## Signature
```rust
async fn per_ip_rate_limiter(
    ConnectInfo(addr): ConnectInfo<std::net::SocketAddr>,
    Extension(state): Extension<RateLimitState>,
    req: Request,
    next: Next,
) -> Result<Response, StatusCode>
```

## Implementation
```rust
async fn per_ip_rate_limiter(
    ConnectInfo(addr): ConnectInfo<std::net::SocketAddr>,
    Extension(state): Extension<RateLimitState>,
    req: Request,
    next: Next,
) -> Result<Response, StatusCode> {
    let ip = addr.ip();
    let allowed = match state
        .redis
        .check_rate_limit(&ip.to_string(), state.limit)
        .await
    {
        Ok(true) => true,
        Ok(false) => false,
        Err(e) => {
            tracing::warn!(error = %e, "Redis rate limit failed, using fallback");
            let mut map = state.fallback.lock().await;
            let count = map.entry(ip).or_insert(0);
            *count += 1;
            *count <= state.limit
        }
    };

    if !allowed {
        // ส่งกลับ HTTP 429 Too Many Requests ถ้าเกินขีดจำกัด
        return Err(StatusCode::TOO_MANY_REQUESTS);
    }

    // อนุญาตให้ดำเนินการต่อ
    Ok(next.run(req).await)
}
```

## Calls & References
- [[RateLimitState|RateLimitState]]


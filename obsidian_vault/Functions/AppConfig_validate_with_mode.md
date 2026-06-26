---
type: function
module: "config.rs"
parent: "AppConfig"
tags: [rust, function]
---

# Function: AppConfig::validate_with_mode

**Defined in:** [config.rs](file:///home/lokis/Documents/banksystemrust/src/config.rs#L226)
**Impl Block:** [[AppConfig]]

## Signature
```rust
pub fn validate_with_mode(&self, production_mode: bool) -> Result<(), String>
```

## Implementation
```rust
pub fn validate_with_mode(&self, production_mode: bool) -> Result<(), String> {
        if self.bank_code.trim().is_empty() {
            return Err("bank_code cannot be empty".into());
        }
        if self.server.port == 0 {
            return Err("server.port cannot be 0".into());
        }
        if self.server.rate_limit.requests_per_second == 0 {
            return Err("server.rate_limit.requests_per_second cannot be 0".into());
        }
        if self.server.rate_limit.burst == 0 {
            return Err("server.rate_limit.burst cannot be 0".into());
        }
        if self.network.quic_port == 0 {
            return Err("network.quic_port cannot be 0".into());
        }
        if self.network.tcp_port == 0 {
            return Err("network.tcp_port cannot be 0".into());
        }
        if self.network.cert_path.is_some() ^ self.network.key_path.is_some() {
            return Err("network.cert_path and network.key_path must be provided together".into());
        }
        if self.server.port == self.network.quic_port
            || self.server.port == self.network.tcp_port
            || self.network.quic_port == self.network.tcp_port
        {
            return Err("ports (server, quic, tcp) must be unique".into());
        }
        if !self.blockchain.endpoint.starts_with("http://")
            && !self.blockchain.endpoint.starts_with("https://")
        {
            return Err("blockchain.endpoint must start with http:// or https://".into());
        }
        if self.crypto.hsm_enabled {
            if let Some(ref path) = self.crypto.hsm_library_path {
                if path.trim().is_empty() {
                    return Err(
                        "crypto.hsm_library_path cannot be empty when hsm_enabled is true".into(),
                    );
                }
            } else {
                return Err(
                    "crypto.hsm_library_path must be specified when hsm_enabled is true".into(),
                );
            }
            if self.crypto.hsm_pin_file.is_none() {
                return Err("crypto.hsm_pin_file is required when hsm_enabled is true".into());
            }
        }
        if self.redis.enabled {
            if !(self.redis.url.starts_with("redis://") || self.redis.url.starts_with("rediss://"))
            {
                return Err("redis.url must start with redis:// or rediss://".into());
            }
            if production_mode && !self.redis.url.starts_with("rediss://") {
                return Err("redis.url must use rediss:// in production".into());
            }
            if production_mode && self.redis.password_file.is_none() {
                return Err("redis.password_file is required in production".into());
            }
            if self
                .redis
                .username
                .as_ref()
                .is_some_and(|username| username.trim().is_empty())
            {
                return Err("redis.username cannot be empty when provided".into());
            }
            if self
                .redis
                .password_file
                .as_ref()
                .is_some_and(|path| path.as_os_str().is_empty())
            {
                return Err("redis.password_file cannot be empty when provided".into());
            }
            if self.redis.ttl_secs == 0 {
                return Err("redis.ttl_secs cannot be 0 when redis is enabled".into());
            }
            if self.redis.timeout_ms == 0 {
                return Err("redis.timeout_ms cannot be 0 when redis is enabled".into());
            }
        }
        Ok(())
    }
```

## Called By
- [[test_production_requires_rediss_for_redis|test::production_requires_rediss_for_redis]]
- [[test_production_requires_redis_password_file|test::production_requires_redis_password_file]]


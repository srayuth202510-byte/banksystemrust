---
type: function
module: "redis_cache.rs"
parent: ""
tags: [rust, function]
---

# Function: build_client_url

**Defined in:** [redis_cache.rs](file:///home/lokis/Documents/banksystemrust/src/redis_cache.rs#L163)

## Signature
```rust
fn build_client_url(config: &RedisConfig) -> Result<String, RedisCacheError>
```

## Implementation
```rust
fn build_client_url(config: &RedisConfig) -> Result<String, RedisCacheError> {
    let Some(password_file) = &config.password_file else {
        return Ok(config.url.clone());
    };

    let password = std::fs::read_to_string(password_file).map_err(|e| {
        RedisCacheError::SecretLoad(format!("cannot read {}: {e}", password_file.display()))
    })?;
    let password = password.trim_end().to_owned();
    if password.is_empty() {
        return Err(RedisCacheError::SecretLoad(
            "redis password file cannot be empty".into(),
        ));
    }
    let password = SecretString::new(password.into());
    let username = config.username.as_deref().unwrap_or("default").trim();
    if username.is_empty() {
        return Err(RedisCacheError::SecretLoad(
            "redis.username cannot be empty when password_file is set".into(),
        ));
    }

    let encoded_password = percent_encode_userinfo(password.expose_secret());
    let auth_segment = if config.username.is_some() {
        format!("{username}:{encoded_password}@")
    } else {
        format!(":{encoded_password}@")
    };

    insert_userinfo(&config.url, &auth_segment)
}
```

## Calls & References
- [[RedisCacheError|RedisCacheError]]
- [[RedisCache_new|RedisCache::new]]
- [[insert_userinfo|insert::userinfo]]
- [[RedisConfig|RedisConfig]]
- [[percent_encode_userinfo|percent::encode_userinfo]]

## Called By
- [[RedisCache_new|RedisCache::new]]
- [[test_build_client_url_with_password_file|test::build_client_url_with_password_file]]


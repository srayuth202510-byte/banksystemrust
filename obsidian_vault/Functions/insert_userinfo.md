---
type: function
module: "redis_cache.rs"
parent: ""
tags: [rust, function]
---

# Function: insert_userinfo

**Defined in:** [redis_cache.rs](file:///home/lokis/Documents/banksystemrust/src/redis_cache.rs#L195)

## Signature
```rust
fn insert_userinfo(url: &str, auth_segment: &str) -> Result<String, RedisCacheError>
```

## Implementation
```rust
fn insert_userinfo(url: &str, auth_segment: &str) -> Result<String, RedisCacheError> {
    let scheme_pos = url
        .find("://")
        .ok_or_else(|| RedisCacheError::SecretLoad("redis url is missing scheme".into()))?;
    let authority_start = scheme_pos + 3;
    let authority = &url[authority_start..];
    let slash_pos = authority.find('/').unwrap_or(authority.len());
    let authority_prefix = &authority[..slash_pos];
    let authority_suffix = &authority[slash_pos..];

    if authority_prefix.contains('@') {
        return Err(RedisCacheError::SecretLoad(
            "redis url must not contain embedded credentials when password_file is set".into(),
        ));
    }

    Ok(format!(
        "{}{}{}{}",
        &url[..authority_start],
        auth_segment,
        authority_prefix,
        authority_suffix
    ))
}
```

## Calls & References
- [[RedisCacheError|RedisCacheError]]

## Called By
- [[build_client_url|build::client_url]]


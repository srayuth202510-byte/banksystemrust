---
type: function
module: "redis_cache.rs"
parent: ""
tags: [rust, function]
---

# Function: test_build_client_url_with_password_file

**Defined in:** [redis_cache.rs](file:///home/lokis/Documents/banksystemrust/src/redis_cache.rs#L257)

## Signature
```rust
fn test_build_client_url_with_password_file()
```

## Implementation
```rust
fn test_build_client_url_with_password_file() {
        let mut file = NamedTempFile::new().unwrap();
        writeln!(file, "s3cr3t!").unwrap();

        let config = RedisConfig {
            enabled: true,
            url: "rediss://redis.example.internal:6379/".into(),
            username: Some("default".into()),
            password_file: Some(file.path().to_path_buf()),
            ttl_secs: 300,
            timeout_ms: 200,
        };

        let url = build_client_url(&config).unwrap();
        assert!(url.starts_with("rediss://default:s3cr3t%21@"));
    }
```

## Calls & References
- [[build_client_url|build::client_url]]
- [[RedisConfig|RedisConfig]]


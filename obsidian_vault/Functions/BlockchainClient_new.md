---
type: function
module: "blockchain.rs"
parent: "BlockchainClient"
tags: [rust, function]
---

# Function: BlockchainClient::new

**Defined in:** [blockchain.rs](file:///home/lokis/Documents/banksystemrust/src/blockchain.rs#L99)
**Impl Block:** [[BlockchainClient]]

## Signature
```rust
pub fn new(config: BlockchainConfig) -> Result<Self, BlockchainError>
```

## Implementation
```rust
pub fn new(config: BlockchainConfig) -> Result<Self, BlockchainError> {
        let mut headers = reqwest::header::HeaderMap::new();
        headers.insert(
            reqwest::header::CONTENT_TYPE,
            reqwest::header::HeaderValue::from_static("application/json"),
        );

        let http_client = reqwest::Client::builder()
            .default_headers(headers)
            .timeout(Duration::from_secs(config.timeout_secs + 2))
            .build()
            .map_err(|e| BlockchainError::Http(format!("Failed to build HTTP client: {e}")))?;

        let mut opts = rocksdb::Options::default();
        opts.create_if_missing(true);
        let (db, _temp_dir) = if let Some(path) = &config.db_path {
            match rocksdb::DB::open(&opts, path) {
                Ok(db) => (db, None),
                Err(e) => {
                    tracing::warn!(
                        "Failed to open rocksdb at {}: {}, falling back to temp dir",
                        path,
                        e
                    );
                    let temp = tempfile::tempdir()
                        .map_err(|e| BlockchainError::DatabaseError(e.to_string()))?;
                    let db = rocksdb::DB::open(&opts, temp.path())
                        .map_err(|e| BlockchainError::DatabaseError(e.to_string()))?;
                    (db, Some(temp))
                }
            }
        } else {
            let temp =
                tempfile::tempdir().map_err(|e| BlockchainError::DatabaseError(e.to_string()))?;
            let db = rocksdb::DB::open(&opts, temp.path())
                .map_err(|e| BlockchainError::DatabaseError(e.to_string()))?;
            (db, Some(temp))
        };

        Ok(Self {
            config,
            db,
            _temp_dir,
            http_client,
        })
    }
```

## Calls & References
- [[BlockchainConfig|BlockchainConfig]]
- [[BlockchainError|BlockchainError]]

## Called By
- [[test_create_transaction|test::create_transaction]]
- [[test_submit_transaction_queued_on_no_node|test::submit_transaction_queued_on_no_node]]
- [[test_get_transaction_status|test::get_transaction_status]]
- [[submit_kyc|submit::kyc]]
- [[main|main]]


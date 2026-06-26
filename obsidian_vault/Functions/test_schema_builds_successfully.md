---
type: function
module: "schema.rs"
parent: ""
tags: [rust, function]
---

# Function: test_schema_builds_successfully

**Defined in:** [schema.rs](file:///home/lokis/Documents/banksystemrust/src/schema.rs#L281)

## Signature
```rust
fn test_schema_builds_successfully()
```

## Implementation
```rust
fn test_schema_builds_successfully() {
        let schema = Schema::build(QueryRoot, MutationRoot, EmptySubscription).finish();
        let sdl = schema.sdl();
        assert!(sdl.contains("type QueryRoot"));
        assert!(sdl.contains("type MutationRoot"));
        assert!(sdl.contains("submitKyc"));
    }
```

## Calls & References
- [[QueryRoot|QueryRoot]]


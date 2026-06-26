---
type: function
module: "main.rs"
parent: ""
tags: [rust, function]
---

# Function: graphiql

**Defined in:** [main.rs](file:///home/lokis/Documents/banksystemrust/src/main.rs#L50)

## Signature
```rust
async fn graphiql() -> impl IntoResponse
```

## Implementation
```rust
async fn graphiql() -> impl IntoResponse {
    Html(GraphiQLSource::build().endpoint("/graphql").finish())
}
```

## Called By
- [[main|main]]


---
type: function
module: "main.rs"
parent: ""
tags: [rust, function]
---

# Function: graphql_handler

**Defined in:** [main.rs](file:///home/lokis/Documents/banksystemrust/src/main.rs#L96)

## Signature
```rust
async fn graphql_handler(
    schema: State<async_graphql::Schema<QueryRoot, MutationRoot, EmptySubscription>>,
    req: GraphQLRequest,
) -> GraphQLResponse
```

## Implementation
```rust
async fn graphql_handler(
    schema: State<async_graphql::Schema<QueryRoot, MutationRoot, EmptySubscription>>,
    req: GraphQLRequest,
) -> GraphQLResponse {
    schema.execute(req.into_inner()).await.into()
}
```

## Calls & References
- [[QueryRoot|QueryRoot]]


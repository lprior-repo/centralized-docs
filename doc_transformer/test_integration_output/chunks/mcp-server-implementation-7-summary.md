---
doc_id: mcp-server-implementation
chunk_id: mcp-server-implementation#7
chunk_level: summary
chunk_type: prose
heading: Implementation Details
token_count: 134
summary: └─────────────────────────────────────┘. ### Error Handling (Railway-Oriented Programming)
---

```
└─────────────────────────────────────┘
```

### Error Handling (Railway-Oriented Programming)

All errors are typed and semantic using `thiserror`:

```rust
#[derive(Debug, Error)]
pub enum McpError {
    #[error("INDEX.json not found at path: {0}")]
    IndexNotFound(String),

    #[error("failed to parse INDEX.json: {0}")]
    InvalidIndex(String),

    #[error("unknown method: {0}")]
    UnknownMethod(String),

    #[error("invalid request: {0}")]

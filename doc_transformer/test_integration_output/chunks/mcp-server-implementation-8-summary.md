---
doc_id: mcp-server-implementation
chunk_id: mcp-server-implementation#8
chunk_level: summary
chunk_type: prose
heading: Implementation Details
token_count: 130
summary: pub enum McpError {.     #[error(\"unknown method: {0}\")]
---

pub enum McpError {


    #[error("unknown method: {0}")]
    UnknownMethod(String),

    #[error("invalid request: {0}")]
    InvalidRequest(String),

    #[error("search error: {0}")]
    SearchError(String),

    #[error("chunk not found: {0}")]
    ChunkNotFound(String),

    #[error("I/O error: {0}")]
    IoError(String),

    #[error("JSON serialization error: {0}")]
    JsonError(String),
}
```

### Search Implementation (Tantivy + Fallback)

The search function gracefully handles multiple failure scenarios:

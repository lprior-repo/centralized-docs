---
doc_id: mcp-server-implementation
chunk_id: mcp-server-implementation#3
chunk_level: standard
chunk_type: prose
heading: Implementation Details
token_count: 516
summary: ## Executive Summary. The implementation follows strict **Functional Rust** principles:
---


---

## Executive Summary



The implementation follows strict **Functional Rust** principles:
- ✅ Zero panics (no `.unwrap()`, `.expect()`, `panic!()`)
- ✅ Railway-Oriented Programming with `Result<T, E>`
- ✅ Semantic error types using `thiserror`
- ✅ Functional Core, Imperative Shell architecture
- ✅ Immutability by default
- ✅ Iterator combinators over imperative loops

---

## Implementation Details

### File Structure

```
src/bin/mcp_server.rs          # MCP server implementation (400+ lines)
indexed_output/INDEX.json      # Test index with 2 docs, 3 chunks
test_mcp_server.sh            # Bash test script
test_mcp_client.py            # Python integration test client
```

### Architecture

```
┌─────────────────────────────────────┐
│   IMPERATIVE SHELL (I/O Layer)      │
│  - stdio JSON-RPC communication     │
│  - File loading (INDEX.json)        │
│  - Error formatting                 │
└──────────────┬──────────────────────┘
               │
               ▼
┌─────────────────────────────────────┐
│   FUNCTIONAL CORE (Pure Logic)      │
│  - load_index()                     │
│  - search_documents()               │
│  - find_chunk()                     │
│  - list_all_documents()             │
│  - handle_request()                 │
│  ALL return Result<T, McpError>     │
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

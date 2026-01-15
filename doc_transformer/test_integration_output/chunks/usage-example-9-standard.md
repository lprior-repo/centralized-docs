---
doc_id: usage-example
chunk_id: usage-example#9
chunk_level: standard
chunk_type: prose
heading: Performance Tuning
token_count: 226
summary: # Run all tests. /test_mcp_server
---

```

## Testing


```bash
# Run all tests
./test_mcp_server.sh

# Python integration tests
python3 test_mcp_client.py

# Unit tests
cargo test --bin mcp_server
```

### Manual Testing

```bash
# Start interactive session
cargo run --bin mcp_server

# Paste JSON requests line-by-line:
{"method":"tools/list"}
{"method":"tools/call","params":{"name":"search_docs","arguments":{"query":"rust"}}}

# Exit with Ctrl+D
```

## Performance Tuning

### Index Warming

Pre-load Tantivy index on startup:

```bash
# Build Tantivy index first
cargo run --bin doc_transformer -- --build-index

# Then start MCP server (index is already warm)
cargo run --bin mcp_server
```

### Release Build

Always use release builds in production:

```bash
cargo build --release --bin mcp_server
./target/release/mcp_server
```

Performance gains:
- ~10x faster search queries
- ~3x lower memory usage
- ~50% faster JSON parsing


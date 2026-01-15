---
doc_id: usage-example
chunk_id: usage-example#22
chunk_level: summary
chunk_type: prose
heading: Performance Tuning
token_count: 130
summary: {\"method\":\"tools/call\",\"params\":{\"name\":\"search_docs\",\"arguments\":{\"query\":\"rust\"}}}
---

```bash

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

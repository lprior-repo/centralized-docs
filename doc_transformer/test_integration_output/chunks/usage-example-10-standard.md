---
doc_id: usage-example
chunk_id: usage-example#10
chunk_level: standard
chunk_type: prose
heading: Monitoring
token_count: 217
summary: # Build Tantivy index first. cargo run --bin doc_transformer -- --build-index
---




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

## Monitoring

### Request Logging

The server logs to stderr:

```
MCP server started. Loaded 2 documents, 3 chunks
```

Redirect stderr for logging:

```bash
cargo run --bin mcp_server 2>> /var/log/mcp-server.log
```

### Metrics

Future enhancement: export Prometheus metrics for:
- Request count by tool
- Query latency (p50, p95, p99)
- Error rate
- Index size

---

**Last Updated**: 2026-01-11
**MCP Version**: 1.0
**Doc Transformer Version**: 0.5.0

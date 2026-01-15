---
doc_id: usage-example
chunk_id: usage-example#24
chunk_level: summary
chunk_type: prose
heading: Monitoring
token_count: 129
summary: Performance gains:. - ~10x faster search queries
---




```bash
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

---
doc_id: usage-example
chunk_id: usage-example#23
chunk_level: summary
chunk_type: prose
heading: Performance Tuning
token_count: 51
summary: ### Release Build. cargo build --release --bin mcp_server
---

```

### Release Build


```bash
cargo build --release --bin mcp_server
./target/release/mcp_server
```

Performance gains:
- ~10x faster search queries
- ~3x lower memory usage
- ~50% faster JSON parsing


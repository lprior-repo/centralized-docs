---
doc_id: mcp-server-implementation
chunk_id: mcp-server-implementation#25
chunk_level: summary
chunk_type: prose
heading: Cargo.toml Configuration
token_count: 86
summary: # Unit tests. cargo test --bin mcp_server
---

```bash
# Unit tests
cargo test --bin mcp_server

# Integration tests
./test_mcp_server.sh
python3 test_mcp_client.py
```

---

## Cargo.toml Configuration

Added binary target:

```toml
[[bin]]
name = "mcp_server"
path = "src/bin/mcp_server.rs"
```

No additional dependencies required (uses existing `serde_json`, `anyhow`, `thiserror`).

---


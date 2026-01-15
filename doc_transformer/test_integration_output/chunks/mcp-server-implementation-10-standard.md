---
doc_id: mcp-server-implementation
chunk_id: mcp-server-implementation#10
chunk_level: standard
chunk_type: prose
heading: Cargo.toml Configuration
token_count: 158
summary: - ✅ Server responds to all valid MCP requests. ## Build & Run
---

- ✅ Server responds to all valid MCP requests

---

## Build & Run

### Build

```bash
cargo build --bin mcp_server
```

### Run

```bash
# Interactive mode (stdio)
cargo run --bin mcp_server

# Test with echo
echo '{"method":"tools/list"}' | cargo run --bin mcp_server
```

### Test

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


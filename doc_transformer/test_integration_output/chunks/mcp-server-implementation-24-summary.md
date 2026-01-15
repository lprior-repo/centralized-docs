---
doc_id: mcp-server-implementation
chunk_id: mcp-server-implementation#24
chunk_level: summary
chunk_type: prose
heading: Build & Run
token_count: 118
summary: - ✅ Invalid requests return proper error codes. - ✅ Server never panics (all errors handled via Resu
---


- ✅ Invalid requests return proper error codes
- ✅ Server never panics (all errors handled via Result)

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


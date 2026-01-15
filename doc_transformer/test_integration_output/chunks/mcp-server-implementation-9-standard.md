---
doc_id: mcp-server-implementation
chunk_id: mcp-server-implementation#9
chunk_level: standard
chunk_type: prose
heading: Build & Run
token_count: 194
summary: - ✅ MCP server binary builds successfully. ### Postconditions (All Satisfied)
---

- ✅ MCP server binary builds successfully

### Postconditions (All Satisfied)
- ✅ MCP server exposes standard protocol
- ✅ Tools return valid JSON responses
- ✅ Errors formatted per MCP spec (JSON-RPC error codes)

### Invariants (All Maintained)
- ✅ Server responds to all valid MCP requests
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


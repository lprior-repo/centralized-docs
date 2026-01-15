---
doc_id: mcp-server-implementation
chunk_id: mcp-server-implementation#8
chunk_level: detailed
chunk_type: table
heading: Build & Run
token_count: 293
summary: | Tantivy index corrupted | Auto-rebuild (handled in `search. | Empty query | Return all documents (
---


---


| Tantivy index corrupted | Auto-rebuild (handled in `search.rs`) |
| Empty query | Return all documents (fallback search) |
| Chunk not found | Return `McpError::ChunkNotFound` |
| Invalid tool name | Return `McpError::UnknownMethod` |

---

## DbC Contracts

### Preconditions (All Satisfied)
- ✅ Index file (INDEX.json) exists and is valid
- ✅ MCP server binary builds successfully
- ✅ Rust std I/O for JSON-RPC communication

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


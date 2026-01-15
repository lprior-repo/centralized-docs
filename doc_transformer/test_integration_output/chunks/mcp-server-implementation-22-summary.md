---
doc_id: mcp-server-implementation
chunk_id: mcp-server-implementation#22
chunk_level: summary
chunk_type: table
heading: DbC Contracts
token_count: 130
summary: | Chunk not found | Return `McpError::ChunkNotFound` |. | Invalid tool name | Return `McpError::Unkn
---

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

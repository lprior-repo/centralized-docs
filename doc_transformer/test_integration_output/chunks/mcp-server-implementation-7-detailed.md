---
doc_id: mcp-server-implementation
chunk_id: mcp-server-implementation#7
chunk_level: detailed
chunk_type: table
heading: DbC Contracts
token_count: 344
summary:    Heading: Introduction.    Tokens: 200
---

   Heading: Introduction
   Tokens: 200

5. Testing error handling (invalid chunk)...
   Expected error: chunk not found: invalid-chunk

=== All Tests Passed ===
```

---

## Edge Cases Handled

| Edge Case | Solution |
|-----------|----------|
| INDEX.json missing | Return `McpError::IndexNotFound` with helpful path |
| Invalid search query | Return empty results (graceful degradation) |
| Malformed MCP request | Return JSON-RPC error with code -32603 |
| Connection closed | Graceful shutdown (no panic) |
| Tantivy index missing | Fallback to simple text search |
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


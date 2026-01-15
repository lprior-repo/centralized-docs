---
doc_id: mcp-server-implementation
chunk_id: mcp-server-implementation#7
chunk_level: standard
chunk_type: table
heading: Edge Cases Handled
token_count: 262
summary:    Found 2 documents:.    Found 2 results:
---

```


   Found 2 documents:

   Found 2 results:
   - Getting Started with Rust (score: 1.0)
   - Advanced Functional Programming (score: 1.0)

4. Testing get_chunk (chunk-001)...
   Chunk: chunk-001
   Doc: Getting Started with Rust
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


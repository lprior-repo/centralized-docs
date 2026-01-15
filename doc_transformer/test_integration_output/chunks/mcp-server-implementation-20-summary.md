---
doc_id: mcp-server-implementation
chunk_id: mcp-server-implementation#20
chunk_level: summary
chunk_type: table
heading: Edge Cases Handled
token_count: 142
summary:    Tokens: 200.    Expected error: chunk not found: invalid-chunk
---



   Tokens: 200

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

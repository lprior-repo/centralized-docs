---
doc_id: mcp-server-implementation
chunk_id: mcp-server-implementation#6
chunk_level: detailed
chunk_type: table
heading: Edge Cases Handled
token_count: 365
summary:  Testing tools/list.    Found 3 tools:
---





```

1. Testing tools/list...
   Found 3 tools:
   - search_docs: Search documentation using full-text search with BM25 ranking
   - get_chunk: Retrieve a specific chunk by ID with navigation context
   - list_docs: List all available documents with metadata

2. Testing list_docs...
   Found 2 documents:
   - Getting Started with Rust (tutorial)
   - Advanced Functional Programming (concept)

3. Testing search_docs (query: 'rust')...
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


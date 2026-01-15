---
doc_id: mcp-server-implementation
chunk_id: mcp-server-implementation#21
chunk_level: summary
chunk_type: table
heading: Edge Cases Handled
token_count: 75
summary: | Tantivy index missing | Fallback to simple text search |. | Tantivy index corrupted | Auto-rebuild
---

| Tantivy index missing | Fallback to simple text search |
| Tantivy index corrupted | Auto-rebuild (handled in `search.rs`) |
| Empty query | Return all documents (fallback search) |
| Chunk not found | Return `McpError::ChunkNotFound` |
| Invalid tool name | Return `McpError::UnknownMethod` |

---


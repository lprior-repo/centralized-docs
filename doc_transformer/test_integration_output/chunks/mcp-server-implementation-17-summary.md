---
doc_id: mcp-server-implementation
chunk_id: mcp-server-implementation#17
chunk_level: summary
chunk_type: prose
heading: Testing
token_count: 139
summary: ### Integration Tests. #### Bash Test Script (`test_mcp_server
---


```bash


```

### Integration Tests

#### Bash Test Script (`test_mcp_server.sh`)

Tests all five scenarios:
1. ✅ tools/list
2. ✅ list_docs
3. ✅ search_docs (query: 'rust')
4. ✅ get_chunk (chunk-001)
5. ✅ Error handling (invalid chunk)

#### Python Test Client (`test_mcp_client.py`)

Full end-to-end integration test:

```
=== MCP Server Tests ===

1. Testing tools/list...
   Found 3 tools:
   - search_docs: Search documentation using full-text search with BM25 ranking
   - get_chunk: Retrieve a specific chunk by ID with navigation context

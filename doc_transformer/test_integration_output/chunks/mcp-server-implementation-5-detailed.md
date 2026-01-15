---
doc_id: mcp-server-implementation
chunk_id: mcp-server-implementation#5
chunk_level: detailed
chunk_type: prose
heading: Testing
token_count: 580
summary: **Output**:.   \"chunk_id\": \"chunk-001\",
---

{
}
```

**Output**:
```json
{
  "chunk_id": "chunk-001",
  "doc_id": "doc-001",
  "doc_title": "Getting Started with Rust",
  "heading": "Introduction",
  "chunk_type": "text",
  "token_count": 200,
  "summary": "Introduction to Rust programming",
  "path": "chunks/chunk-001-standard.md",
  "previous_chunk_id": null,
  "next_chunk_id": "chunk-002",
  "related_chunks": [],
  "chunk_level": "standard",
  "parent_chunk_id": null,
  "child_chunk_ids": []
}
```

### Tool 3: `list_docs`

**Description**: List all available documents with metadata

**Input Schema**:
```json
{}
```

**Output**:
```json
{
  "documents": [
    {
      "id": "doc-001",
      "title": "Getting Started with Rust",
      "category": "tutorial",
      "tags": ["rust", "beginner", "tutorial"],
      "word_count": 1500,
      "chunk_count": 2
    }
  ],
  "total": 2
}
```

---

## Testing

### Unit Tests

All unit tests pass (4 tests):

```bash
$ cargo test --bin mcp_server

test tests::test_default_limit ... ok
test tests::test_format_error ... ok
test tests::test_generate_tools_list ... ok
test tests::test_list_all_documents_empty ... ok

test result: ok. 4 passed; 0 failed
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


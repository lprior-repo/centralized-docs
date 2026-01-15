---
doc_id: usage-example
chunk_id: usage-example#1
chunk_level: standard
chunk_type: prose
heading: Quick Start
token_count: 291
summary: # MCP Server Usage Example. ## Quick Start
---

# MCP Server Usage Example

## Quick Start

### 1. Start the MCP Server

```bash
cargo run --bin mcp_server
```

The server reads JSON-RPC requests from stdin and writes responses to stdout.

### 2. Query Available Tools

```bash
echo '{"method":"tools/list"}' | cargo run --bin mcp_server
```

**Response**:
```json
{
  "tools": [
    {
      "name": "search_docs",
      "description": "Search documentation using full-text search with BM25 ranking",
      "inputSchema": {
        "type": "object",
        "properties": {
          "query": {"type": "string", "description": "Search query"},
          "limit": {"type": "number", "default": 10}
        },
        "required": ["query"]
      }
    },
    {
      "name": "get_chunk",
      "description": "Retrieve a specific chunk by ID with navigation context",
      "inputSchema": {
        "type": "object",
        "properties": {
          "chunk_id": {"type": "string"}
        },
        "required": ["chunk_id"]
      }
    },
    {
      "name": "list_docs",
      "description": "List all available documents with metadata",
      "inputSchema": {"type": "object", "properties": {}}
    }
  ]
}
```


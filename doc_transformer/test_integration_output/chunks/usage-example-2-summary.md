---
doc_id: usage-example
chunk_id: usage-example#2
chunk_level: summary
chunk_type: prose
heading: Quick Start
token_count: 134
summary:       \"name\": \"search_docs\",.       \"inputSchema\": {
---


```json
{
  "tools": [
    {
      "name": "search_docs",
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

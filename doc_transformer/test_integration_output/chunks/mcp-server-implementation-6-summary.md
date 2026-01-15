---
doc_id: mcp-server-implementation
chunk_id: mcp-server-implementation#6
chunk_level: summary
chunk_type: prose
heading: Implementation Details
token_count: 137
summary: ┌─────────────────────────────────────┐. │   FUNCTIONAL CORE (Pure Logic)      │
---

```
┌─────────────────────────────────────┐
│   FUNCTIONAL CORE (Pure Logic)      │
│  - load_index()                     │
│  - search_documents()               │
│  - find_chunk()                     │
│  - list_all_documents()             │
│  - handle_request()                 │
│  ALL return Result<T, McpError>     │
└─────────────────────────────────────┘

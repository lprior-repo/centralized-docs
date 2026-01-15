---
doc_id: mcp-server-implementation
chunk_id: mcp-server-implementation#5
chunk_level: summary
chunk_type: prose
heading: Implementation Details
token_count: 143
summary: ┌─────────────────────────────────────┐. │   IMPERATIVE SHELL (I/O Layer)      │
---

```
┌─────────────────────────────────────┐
│   IMPERATIVE SHELL (I/O Layer)      │
│  - stdio JSON-RPC communication     │
│  - File loading (INDEX.json)        │
│  - Error formatting                 │
└──────────────┬──────────────────────┘
               │
               ▼
┌─────────────────────────────────────┐

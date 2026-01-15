---
doc_id: mcp-server-implementation
chunk_id: mcp-server-implementation#9
chunk_level: summary
chunk_type: prose
heading: Implementation Details
token_count: 136
summary: ### Search Implementation (Tantivy + Fallback). The search function gracefully handles multiple fail
---

```

### Search Implementation (Tantivy + Fallback)

The search function gracefully handles multiple failure scenarios:

1. **Tantivy index exists and has results** → Use Tantivy BM25 search
2. **Tantivy index missing/corrupted** → Fallback to simple text matching
3. **Tantivy returns 0 results** → Fallback to simple text matching

This ensures search **never fails** even when the Tantivy index is unavailable.

```rust
fn search_documents(
    index_dir: &Path,
    query: &str,
    limit: usize,
    fallback_docs: &[IndexDocument],

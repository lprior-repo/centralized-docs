---
doc_id: production-readiness-report
chunk_id: production-readiness-report#6
chunk_level: standard
chunk_type: prose
heading: Build & Release Status
token_count: 263
summary: - `index`: 4/4 ✅ (Complexity tests). **Known failures (pre-existing, not blocking):**
---


- `index`: 4/4 ✅ (Complexity tests)

**Known failures (pre-existing, not blocking):**
- `highlight::tests::test_special_chars_in_query` (C++ tokenization)
- `transform::tests::test_context_blockquote_detection` (blockquote regex)

### Integration Tests: 10/10 (100%)
- Full pipeline edge cases
- Empty directories, large documents (5000+ words)
- Unicode/multilingual content
- Malformed markdown

---

## Build & Release Status

### Compilation
```bash
$ cargo build --release
   Compiling doc_transformer v0.5.0
    Finished release [optimized] target(s) in 98.32s
```
✅ **Zero errors, warnings acceptable** (dead code, unused imports)

### Binaries
- **doc_transformer:** Primary CLI (transform, index, search)
- **mcp_server:** AI documentation query server (8.0MB)

### Dependencies (Battle-Tested)
- **Tantivy 0.25:** Full-text search
- **pulldown-cmark 0.13:** Markdown parsing
- **readability 0.3:** Content extraction
- **hnsw_rs 0.3:** Nearest neighbor search
- **spider 2.0:** Web scraping
- **petgraph 0.8:** Graph data structures

---


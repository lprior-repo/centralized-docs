---
doc_id: production-readiness-report
chunk_id: production-readiness-report#11
chunk_level: summary
chunk_type: prose
heading: Test Coverage
token_count: 110
summary: - `graph`: 5/5 ✅ (HNSW-based DAG). - `index`: 4/4 ✅ (Complexity tests)
---

- `graph`: 5/5 ✅ (HNSW-based DAG)
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


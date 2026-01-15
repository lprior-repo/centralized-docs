---
doc_id: readability-implementation-report
chunk_id: readability-implementation-report#13
chunk_level: summary
chunk_type: prose
heading: Design by Contract (DbC) Verification
token_count: 87
summary: - `ExtractedContent. confidence` ∈ [0
---

- `ExtractedContent.confidence` ∈ [0.0, 1.0]
- `content` is never empty on success
- Errors are enumerated (no string errors)

### Invariants ✅
- Function never panics (Railway pattern catches all errors)
- Graceful degradation (fallback to custom pruning)
- All errors are typed (`ExtractionError` enum)
- No `.unwrap()` in production code path


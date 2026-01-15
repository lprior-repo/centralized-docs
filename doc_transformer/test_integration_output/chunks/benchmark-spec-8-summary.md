---
doc_id: benchmark-spec
chunk_id: benchmark-spec#8
chunk_level: summary
chunk_type: prose
heading: 2. Test Data Generators
token_count: 103
summary: **Properties:**. - All chunks share category prefixes (Category 0-4)
---





**Properties:**
- All chunks share category prefixes (Category 0-4)
- Includes "documentation" and "section_X" tags
- Realistic for semantic clustering

### Generator Guarantees

```rust
// All generators are deterministic (no randomness)
// Same N produces identical data on all runs
// Data structure matches IndexDocument/Chunk contracts
// No edge cases handled specially (empty sets possible)
```

---


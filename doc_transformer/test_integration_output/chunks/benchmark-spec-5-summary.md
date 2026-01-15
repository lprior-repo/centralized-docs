---
doc_id: benchmark-spec
chunk_id: benchmark-spec#5
chunk_level: summary
chunk_type: prose
heading: 2. Test Data Generators
token_count: 136
summary: Invariants:. - Each chunk has ≤ max_related_chunks edges
---


Invariants:
- Each chunk has ≤ max_related_chunks edges
- All relationships are deterministic (seeded RNG)
```

---

## 2. Test Data Generators

### Architecture

Three generator functions create synthetic yet realistic test data:

#### A. `generate_test_chunks(n: usize) -> Vec<Chunk>`

Generates N chunks distributed across documents in a realistic structure.

**Properties:**
- Distributes chunks across sqrt(N) documents
- Each chunk has realistic metadata (heading, token_count, etc.)
- Creates sequential edges (previous/next_chunk_id)

---
doc_id: benchmark-implementation
chunk_id: benchmark-implementation#8
chunk_level: summary
chunk_type: prose
heading: 4. Test Data Generators
token_count: 129
summary: tag_generation/5000. tag_generation/10000
---

```
tag_generation/5000
tag_generation/10000
```

**Purpose:** Verify tag prep is < 1% of total benchmark time

---

## 4. Test Data Generators

### `generate_test_chunks(n: usize) -> Vec<Chunk>`

Creates N synthetic chunks distributed across documents.

**Features:**
- Distributes chunks across sqrt(N) documents
- Sequential edges: previous_chunk_id → next_chunk_id
- Realistic metadata: token_count, heading, content
- Deterministic (no randomness)

**Example (n=100):**
```
9 documents
11-12 chunks per document

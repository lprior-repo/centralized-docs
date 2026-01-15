---
doc_id: benchmark-implementation
chunk_id: benchmark-implementation#4
chunk_level: standard
chunk_type: prose
heading: 4. Test Data Generators
token_count: 454
summary: chunk_generation/100. chunk_generation/1000
---

```
chunk_generation/100
chunk_generation/1000
chunk_generation/5000
chunk_generation/10000
```

**Purpose:** Verify data gen is < 5% of total benchmark time

---

### Group 4: `tag_generation` (Overhead Analysis)

Measures tag creation overhead:

```
tag_generation/100
tag_generation/1000
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
chunk_0_0000, chunk_0_0001, ..., chunk_8_0011
Sequential linking: chunk_i_j → chunk_i_(j+1)
```

### `generate_test_documents(chunks: &[Chunk]) -> Vec<IndexDocument>`

Groups chunks into documents with metadata.

**Features:**
- One document per unique doc_id
- 3-5 tags per document
- 5 categories distributed across documents
- Word counts scale with document index

### `generate_test_tags(chunks: &[Chunk]) -> Vec<(String, Vec<String>, String)>`

Creates tag metadata for relationship detection.

**Features:**
- Cyclic tag distribution (tag_0, tag_1, tag_2)
- Global tags: "documentation", "section_X"
- Categories: "Category 0" through "Category 4"
- Realistic for semantic clustering

### Data Properties

All generators produce:
- **Deterministic output** (same N → same data every run)
- **Reproducible relationships** (enables benchmarking same comparisons)
- **Realistic structure** (mirrors production document sets)
- **No false optimization** (data gen cannot be inlined/optimized away)

---


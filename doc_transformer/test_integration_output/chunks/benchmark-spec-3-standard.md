---
doc_id: benchmark-spec
chunk_id: benchmark-spec#3
chunk_level: standard
chunk_type: prose
heading: 2. Test Data Generators
token_count: 487
summary: Preconditions:. - Test data generators produce consistent, reproducible data
---


```
Preconditions:
- Test data generators produce consistent, reproducible data

Postconditions:
- Benchmark completes without OOM or panic
- Results stored in target/criterion/
- HTML reports generated for trend analysis
- Edge count grows ≤ O(n log n)

Invariants:
- DAG property maintained (no cycles)
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
- Tags are semantically meaningful but synthetic

**Example for n=100:**
```
9 documents × 11-12 chunks each
Chunks: chunk_0_0000 through chunk_8_0011
Sequential relationships: chunk_i_j → chunk_i_(j+1)
```

#### B. `generate_test_documents(chunks: &[Chunk]) -> Vec<IndexDocument>`

Groups chunks into documents with tags and metadata.

**Properties:**
- One document per unique doc_id in chunks
- Assigns 3-5 tags per document
- Categories distributed across 5 categories
- Word counts scale with document index

#### C. `generate_test_tags(chunks: &[Chunk]) -> Vec<(String, Vec<String>, String)>`

Creates tag sets for relationship detection.

**Properties:**
- Tags follow pattern: tag_0, tag_1, tag_2 (cyclical)
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


---
doc_id: benchmark-implementation
chunk_id: benchmark-implementation#9
chunk_level: summary
chunk_type: prose
heading: 4. Test Data Generators
token_count: 142
summary: **Features:**. - Deterministic (no randomness)
---

---




**Features:**
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

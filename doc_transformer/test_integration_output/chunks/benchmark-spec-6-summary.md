---
doc_id: benchmark-spec
chunk_id: benchmark-spec#6
chunk_level: summary
chunk_type: prose
heading: 2. Test Data Generators
token_count: 129
summary: - Each chunk has realistic metadata (heading, token_count, etc. - Creates sequential edges (previous
---



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

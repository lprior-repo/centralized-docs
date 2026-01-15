---
doc_id: vision-analysis
chunk_id: vision-analysis#6
chunk_level: standard
chunk_type: prose
heading: 🎨 The Innovation: Contextual Chunking
token_count: 211
summary:  **Vector Embeddings** (Future Enhancement). md mentions this as \"Possible Future Enhancement\":**
---



#### 5. **Vector Embeddings** (Future Enhancement)
**INDEXER.md mentions this as "Possible Future Enhancement":**
- Add embedding vectors for semantic similarity
- Beyond Jaccard similarity
- Requires embedding model integration

#### 6. **Incremental Updates** (Future Enhancement)
**INDEXER.md mentions:**
- Track changed files
- Only re-process deltas
- Faster iteration for large doc sets

---

## 🎨 The Innovation: Contextual Chunking

**From INDEXER.md:**
> Each chunk is self-contained with 50-100 token context prefix from previous chunk
> = ~170 tokens total, semantically complete
> **Result: 35% fewer retrieval failures (Anthropic research)**

This is the SECRET SAUCE that makes centralized-docs special.

**Current State:** Implemented in chunk.rs, works perfectly
**Gap:** Not extracted as reusable crate for community

---


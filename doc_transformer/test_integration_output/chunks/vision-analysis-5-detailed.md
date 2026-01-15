---
doc_id: vision-analysis
chunk_id: vision-analysis#5
chunk_level: detailed
chunk_type: prose
heading: 🎨 The Innovation: Contextual Chunking
token_count: 320
summary: txt RFC and Tooling**. **Why Important:** Define THE standard for AI docs
---

#### 3. **llms.txt RFC and Tooling**
**Why Important:** Define THE standard for AI docs
**What's needed:**
- RFC document specification
- Validator CLI tool
- Parser library
- Community site (llms.txt.org)

#### 4. **Community Index Repository**
**Why Important:** Enable sharing pre-built indexes
**What's needed:**
- Git-based repository structure
- Initial indexes (Rust Book, Python, Kubernetes, etc.)
- Documentation for contributors

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


---
doc_id: vision-analysis
chunk_id: vision-analysis#14
chunk_level: summary
chunk_type: prose
heading: 🎨 The Innovation: Contextual Chunking
token_count: 138
summary: md mentions:**. - Track changed files
---


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

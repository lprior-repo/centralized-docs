---
doc_id: vision-analysis
chunk_id: vision-analysis#7
chunk_level: standard
chunk_type: prose
heading: 📋 Recommendation: What Should Be in PLAN.md
token_count: 300
summary: **From INDEXER. > Each chunk is self-contained with 50-100 token context prefix from previous chunk
---



**From INDEXER.md:**
> Each chunk is self-contained with 50-100 token context prefix from previous chunk
> = ~170 tokens total, semantically complete
> **Result: 35% fewer retrieval failures (Anthropic research)**

This is the SECRET SAUCE that makes centralized-docs special.

**Current State:** Implemented in chunk.rs, works perfectly
**Gap:** Not extracted as reusable crate for community

---

## 📋 Recommendation: What Should Be in PLAN.md

### Option A: Keep PLAN.md Focused on v5.0 ✅
**Current approach:** PLAN.md covers v5.0 web scraping features
**Gap:** MCP server, RFC, crate extraction are v6.0+ features

**If this approach:**
- Create PLAN_v6.md for MCP server + community features
- Keep PLAN.md as historical v5.0 document
- ✅ PLAN.md is complete for v5.0 scope

### Option B: Expand PLAN.md to Cover Full Vision
**Alternative:** Make PLAN.md cover everything in WORK_PLAN.md
**Add sections for:**
1. MCP Server implementation
2. Contextual-chunker crate extraction
3. llms.txt RFC specification
4. Community repository structure

**If this approach:**
- PLAN.md becomes master planning document
- Subsumes WORK_PLAN.md content
- Single source of truth for full vision

---


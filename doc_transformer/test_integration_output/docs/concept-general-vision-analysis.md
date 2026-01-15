---
id: concept/general/vision-analysis
title: VISION ANALYSIS
category: concept
tags: [""codanna", "(v5.0)", "analysis:", "centralized-docs", "concept"]
---

# Vision Analysis: centralized-docs


> **Context**: **Analysis Date:** 2026-01-15



**Analysis Date:** 2026-01-15
**Purpose:** Understand the complete vision and verify PLAN.md captures everything
## 🎯 Core Vision: “Codanna for Documentation”
**The Big Idea:** Create the best documentation indexer for AI agents - a system that transforms any documentation into an AI-queryable knowledge graph with:
- **Semantic chunking** with contextual prefixes (35% fewer retrieval failures)
- **llms.txt** as the standard AI entry point (like robots.txt for AI)
- **MCP server** for AI to query documentation
- **Community indexes** for sharing pre-built documentation indexes

## 📊 Current State (v5.0)
### What’s Implemented ✅
- **Core Pipeline** (7 steps): Discover → Analyze → Assign → Transform → Chunk → Index → Validate
- **Web Scraping**: spider-rs with sitemap support
- **Content Filtering**: BM25 + Mozilla Readability algorithm
- **llms.txt Generation**: AI entry point files
- **Full-text Search**: Tantivy with BM25 scoring
- **Semantic Similarity**: HNSW approximate nearest neighbor
- **Knowledge Graph**: DAG with Jaccard similarity
- **CLI Commands**: scrape, index, ingest, search, legacy mode
- **Contextual Chunking**: 50-100 token context prefixes

### Test Coverage
- 535/535 tests passing (100%)
- Comprehensive edge case coverage
- Integration tests for full pipeline

### Build Status
- Release build: ✅ SUCCESS
- Pure Rust implementation
- Functional programming patterns throughout

## 🔮 Strategic Roadmap (WORK_PLAN.md)
### Phase 1: Critical Infrastructure (P0) - **PARTIALLY DONE**
- **Build MCP Server** (`centralized-docs-jxo`)- Expose INDEX.json, GRAPH.json via MCP tools
- Provide search_docs(), get_chunk(), find_related() tools
- **Status:** NOT in PLAN.md - This is the MISSING PIECE

### Phase 2: Reduce Custom Code (P1) - **MOSTLY DONE**
- **Replace BM25 with Tantivy** (`centralized-docs-uq2`)- Status: ✅ DONE (Tantivy integrated)

- **Replace Regex with pulldown-cmark** (`centralized-docs-6bs`)- Status: ✅ DONE (AST-based transforms)


### Phase 3: Extract Innovation (P2) - **PARTIALLY DONE**
- **Replace Pruning with Readability** (`centralized-docs-lhk`)- Status: ✅ DONE (Mozilla Readability integrated)

- **Extract contextual-chunker crate** (`centralized-docs-7d8`)- Status: NOT in PLAN.md - Should be separate crate

- **Define llms.txt RFC** (`centralized-docs-bi9`)- Status: NOT in PLAN.md - Needs standardization


### Phase 4: Build Community (P3) - **NOT STARTED**
- **Community Index Repository** (`centralized-docs-bqk`)- Status: NOT in PLAN.md - Future work

## 🔍 Gap Analysis: PLAN.md vs Vision
### What PLAN.md Covers ✅
- Web scraping architecture (spider-rs)
- Content filtering (BM25, pruning)
- llms.txt generation
- CLI commands (scrape, index, ingest, search)
- Output structure (llms.txt, INDEX.json, COMPASS.md)
- Dependencies for v5.0
- Implementation order

### What PLAN.md MISSES ❌
#### 1. **MCP Server** (CRITICAL GAP)
**Why Critical:** This is the “missing piece” that enables AI to query indexed docs
**What’s needed:**
- Rust MCP SDK integration (rust-mcp-sdk = “0.8”)
- Tools: search_docs(), get_chunk(), find_related()
- Expose INDEX.json and GRAPH.json
- Enable AI agents to query without re-scraping

**Impact:** Without MCP server, the vision is incomplete
#### 2. **Contextual-Chunker as Standalone Crate**
**Why Important:** Makes innovation reusable
**What’s needed:**
- Extract chunk.rs into separate crate
- Publish to crates.io
- Document the 35% improvement metric
- Enable other projects to use this innovation

#### 3. **llms.txt RFC and Tooling**
**Why Important:** Define THE standard for AI docs
**What’s needed:**
- RFC document specification
- Validator CLI tool
- Parser library
- Community site (llms.txt.org)

#### 4. **Community Index Repository**
**Why Important:** Enable sharing pre-built indexes
**What’s needed:**
- Git-based repository structure
- Initial indexes (Rust Book, Python, Kubernetes, etc.)
- Documentation for contributors

#### 5. **Vector Embeddings** (Future Enhancement)
**INDEXER.md mentions this as “Possible Future Enhancement”:**
- Add embedding vectors for semantic similarity
- Beyond Jaccard similarity
- Requires embedding model integration

#### 6. **Incremental Updates** (Future Enhancement)
**INDEXER.md mentions:**
- Track changed files
- Only re-process deltas
- Faster iteration for large doc sets

## 🎨 The Innovation: Contextual Chunking
**From INDEXER.md:**
> Each chunk is self-contained with 50-100 token context prefix from previous chunk
= ~170 tokens total, semantically complete
**Result: 35% fewer retrieval failures (Anthropic research)**

This is the SECRET SAUCE that makes centralized-docs special.
**Current State:** Implemented in chunk.rs, works perfectly
**Gap:** Not extracted as reusable crate for community
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
- MCP Server implementation
- Contextual-chunker crate extraction
- llms.txt RFC specification
- Community repository structure

**If this approach:**
- PLAN.md becomes master planning document
- Subsumes WORK_PLAN.md content
- Single source of truth for full vision

## 🤔 Key Questions for User
- **Scope of PLAN.md:** Should it cover only v5.0, or the full multi-phase vision?

- **MCP Server:** Is this v5.0 or v6.0? It’s marked as “CRITICAL” in WORK_PLAN but not in PLAN.md

- **Priority:** What matters most right now?

- Complete v5.0 as-is (web scraping focus)

- Add MCP server to v5.0 (critical infrastructure)

- Plan v6.0 (community + standards)

- **Crate Extraction:** Should contextual-chunker be extracted now or later?

- **llms.txt RFC:** Is this part of the current roadmap or future work?


## 💡 My Assessment
### PLAN.md Status for v5.0 Scope
**Verdict:** ✅ COMPLETE for v5.0
Everything in PLAN.md has been implemented:
- Web scraping with spider-rs
- Content filtering with BM25 and Readability
- llms.txt generation
- CLI commands
- Full-text search
- All dependencies added
- All tests passing

### Gap: Beyond v5.0
The gaps are **strategic/future work** (MCP server, RFC, crate extraction, community repo), not v5.0 requirements.
**WORK_PLAN.md** contains the broader vision across 4 phases.
**PLAN.md** contains the v5.0 tactical implementation.
Both documents serve different purposes and are both complete for their scope.
## 🚀 Recommendation
### For Immediate Work
- ✅ Accept that PLAN.md is complete for v5.0
- ✅ Recognize WORK_PLAN.md contains phases 1-4
- 🤔 Decide if MCP server should be v5.1 or v6.0
- 🤔 Decide if we need a comprehensive PLAN_v6.md

### For Documentation Clarity
- Keep PLAN.md as v5.0 (web scraping focus) ✅
- Use WORK_PLAN.md for multi-phase roadmap ✅
- Create PLAN_v6.md when ready for next phase
- Maintain VISION.md for long-term strategy (if it exists)

## 📖 Document Hierarchy
```
VISION.md (if exists)          ← Long-term strategy, 5+ year vision
    ↓
WORK_PLAN.md                   ← 4-phase roadmap (P0-P3), 1-2 year plan
    ↓
PLAN.md (v5.0)                 ← Tactical: Web scraping + llms.txt
PLAN_v6.md (future)            ← Tactical: MCP server + community
    ↓
IMPLEMENTATION_*.md            ← Execution reports (as work completes)



```
**Current State:**
- PLAN.md: ✅ Complete for v5.0
- WORK_PLAN.md: ✅ Complete roadmap
- PLAN_v6.md: ⏳ Needed when MCP work starts

## ✅ Conclusion
**Your vision is clearly documented across multiple files:**
- **README.md** - What it is, how to use it
- **INDEXER.md** - Deep technical architecture
- **CLAUDE.md** - Development patterns
- **PLAN.md** - v5.0 tactical implementation (WEB SCRAPING)
- **WORK_PLAN.md** - Strategic roadmap (PHASES 1-4)

**The “missing piece”** (MCP server) is intentionally FUTURE WORK (Phase 1, P0).
**PLAN.md is complete** for its defined scope (v5.0 web scraping).
**Next decision:** Do you want to:
- Keep current structure (PLAN.md = v5.0, WORK_PLAN.md = roadmap)
- Create PLAN_v6.md for MCP server phase
- Or expand PLAN.md to cover full multi-phase vision


## See Also

- [Documentation Index](./COMPASS.md)

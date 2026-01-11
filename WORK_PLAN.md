# Work Plan: Leveraging Latest Dependencies

**Created:** 2026-01-11
**Dependencies Upgraded:** See [DEPENDENCY_UPGRADE.md](./DEPENDENCY_UPGRADE.md)

## Overview

After upgrading to the latest Rust dependencies, we've identified 7 strategic tasks to leverage the new libraries and complete the "Codanna for Documentation" vision.

**All tasks tracked in Beads with context blocks for primitive AI execution.**

---

## Priority 0 (Critical) - 1 Task

### 🚨 Build MCP Server for AI Documentation Queries
**Bead:** `centralized-docs-jxo`
**Status:** Open
**Why Critical:** This is the **missing piece** for the entire platform. Without the MCP server, AI cannot query indexed documentation.

**What It Does:**
- Exposes INDEX.json, GRAPH.json via MCP tools
- Provides `search_docs()`, `get_chunk()`, `find_related()` tools
- Enables AI to query documentation without re-scraping

**Dependencies:** `rust-mcp-sdk = "0.8"` ✅ Already added

**Impact:** Completes the "Codanna for Documentation" architecture

---

## Priority 1 (High) - 2 Tasks

### 📝 Replace Custom BM25 Implementation with Tantivy
**Bead:** `centralized-docs-uq2`
**Status:** Open

**Benefits:**
- Reduce ~440 LOC → ~60 LOC (88% reduction)
- Proven algorithm (30 years of optimization)
- Better accuracy, faster indexing
- Incremental updates support

**Dependencies:** `tantivy = "0.25"` ✅ Already added

---

### 🔧 Replace Regex-Based Markdown Transforms with pulldown-cmark AST
**Bead:** `centralized-docs-6bs`
**Status:** Open

**Benefits:**
- Safer transformations (AST instead of regex)
- Handles edge cases: code blocks, nested structures, HTML
- Used by cargo doc, mdBook (battle-tested)

**Dependencies:** `pulldown-cmark = "0.13"` ✅ Already added

---

## Priority 2 (Medium) - 3 Tasks

### 🧹 Replace Custom Content Pruning with Mozilla Readability
**Bead:** `centralized-docs-lhk`
**Status:** Open

**Benefits:**
- 14 years of Mozilla research
- Handles cookie banners, paywalls, GDPR notices
- Proven accuracy with real-world content

**Dependencies:** `readability = "0.3"` ✅ Already added

---

### 📦 Extract contextual-chunker as Standalone Crate
**Bead:** `centralized-docs-7d8`
**Status:** Open

**Benefits:**
- Make innovation reusable by other projects
- Publish to crates.io
- 35% fewer retrieval failures (Anthropic research)

**Current:** Buried in doc_transformer binary
**Target:** Standalone, well-documented crate

---

### 📄 Define llms.txt RFC Standard and Tooling
**Bead:** `centralized-docs-bi9`
**Status:** Open

**Benefits:**
- Define THE standard for AI documentation entry points
- Like robots.txt, but for AI
- Build ecosystem: validator, parser, community site

**Deliverables:**
- RFC document
- Validator CLI
- Parser library
- Community site (llms.txt.org)

---

## Priority 3 (Nice-to-Have) - 1 Task

### 🌐 Create Community Index Repository
**Bead:** `centralized-docs-bqk`
**Status:** Open

**Benefits:**
- Pre-built indexes (no re-scraping needed)
- Git-based sharing (clone and use)
- Community-maintained (like npm, but for docs)

**Target:** `github.com/centralized-docs/indexes`

**Initial Indexes:**
- Rust Book
- Python Official Docs
- Kubernetes Docs
- TypeScript Handbook
- Go Documentation

---

## Execution Strategy

### Phase 1: Complete Critical Infrastructure (P0)
**Time:** 2-3 weeks

1. Build MCP server (`centralized-docs-jxo`)
   - Enables AI to query indexed docs
   - Completes the platform architecture

**Milestone:** AI can query documentation via MCP tools ✅

---

### Phase 2: Reduce Custom Code (P1)
**Time:** 2-3 weeks

1. Replace BM25 with Tantivy (`centralized-docs-uq2`)
2. Replace regex with pulldown-cmark AST (`centralized-docs-6bs`)

**Milestone:** 80% less custom code, 10x better quality ✅

---

### Phase 3: Extract and Share Innovation (P2)
**Time:** 4-6 weeks

1. Replace content pruning with Readability (`centralized-docs-lhk`)
2. Extract contextual-chunker to crate (`centralized-docs-7d8`)
3. Define llms.txt RFC standard (`centralized-docs-bi9`)

**Milestone:** Innovation published, standards defined ✅

---

### Phase 4: Build Community (P3)
**Time:** 4-8 weeks (ongoing)

1. Create community index repository (`centralized-docs-bqk`)
2. Index 10+ popular documentation sets
3. Announce on social media / forums

**Milestone:** Community adoption, shared indexes ✅

---

## Success Metrics

### Technical Metrics
- ✅ MCP server running (AI can query docs)
- ✅ Custom code reduced by 80% (~1000 LOC → ~200 LOC)
- ✅ contextual-chunker published to crates.io
- ✅ llms.txt RFC published

### Community Metrics
- ✅ 5+ documentation sets in community repo
- ✅ 10+ community contributors
- ✅ 100+ GitHub stars on main repo
- ✅ Mentioned on Hacker News / r/rust

### Adoption Metrics
- ✅ 3+ projects using contextual-chunker crate
- ✅ 5+ projects with llms.txt files
- ✅ MCP server used in production by 1+ teams

---

## Dependencies Graph

```
Phase 1 (P0):
  [Build MCP Server] → Blocks Phase 4 (need server for testing)
                     ↓
                  Enables AI queries

Phase 2 (P1):
  [Replace BM25] → Improves MCP server quality
  [Replace Regex] → Improves doc quality
                  ↓
               Better accuracy

Phase 3 (P2):
  [Replace Pruning] → Better content extraction
  [Extract Chunker] → Publishable innovation
  [Define llms.txt] → Standards for community
                    ↓
                 Community ready

Phase 4 (P3):
  [Community Repo] → Requires: MCP server, Standards
                   ↓
                Adoption
```

---

## All Work Tracked in Beads

✅ **7 beads created with "Context Blocks"**

Each bead contains:
- **CONTEXT BLOCK:** File/function, the smell, dependencies
- **SPECIFICATION BLOCK:** EARS, DbC, Schema & Edge Cases
- **IMPLEMENTATION GUIDANCE:** Step-by-step approach

**Format:** Designed for "one-shot" primitive AI execution (no hallucinations)

---

## Next Steps

1. Review beads: `bd ready`
2. Pick highest priority: `bd show centralized-docs-jxo`
3. Claim work: `bd update centralized-docs-jxo --status=in_progress`
4. Execute using context block
5. Close when done: `bd close centralized-docs-jxo`
6. Repeat for next priority

---

## Related Documents

- [DEPENDENCY_UPGRADE.md](./DEPENDENCY_UPGRADE.md) - Full upgrade report
- [docs/VISION.md](./docs/VISION.md) - Long-term vision
- [docs/ARCHITECTURE.md](./docs/ARCHITECTURE.md) - System architecture

---

**Ready to execute. All work planned and tracked. 🚀**

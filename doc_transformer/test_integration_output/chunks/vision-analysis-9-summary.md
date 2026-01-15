---
doc_id: vision-analysis
chunk_id: vision-analysis#9
chunk_level: summary
chunk_type: prose
heading: 🔍 Gap Analysis: PLAN.md vs Vision
token_count: 146
summary: - [ ] **Community Index Repository** (`centralized-docs-bqk`).   - Status: NOT in PLAN
---



- [ ] **Community Index Repository** (`centralized-docs-bqk`)
  - Status: NOT in PLAN.md - Future work

---

## 🔍 Gap Analysis: PLAN.md vs Vision

### What PLAN.md Covers ✅
1. Web scraping architecture (spider-rs)
2. Content filtering (BM25, pruning)
3. llms.txt generation
4. CLI commands (scrape, index, ingest, search)
5. Output structure (llms.txt, INDEX.json, COMPASS.md)
6. Dependencies for v5.0
7. Implementation order

### What PLAN.md MISSES ❌

#### 1. **MCP Server** (CRITICAL GAP)
**Why Critical:** This is the "missing piece" that enables AI to query indexed docs

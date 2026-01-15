---
doc_id: vision-analysis
chunk_id: vision-analysis#4
chunk_level: standard
chunk_type: prose
heading: 🔍 Gap Analysis: PLAN.md vs Vision
token_count: 516
summary: - [ ] **Extract contextual-chunker crate** (`centralized-docs-7d8`).   - Status: NOT in PLAN
---

---




- [ ] **Extract contextual-chunker crate** (`centralized-docs-7d8`)
  - Status: NOT in PLAN.md - Should be separate crate
- [ ] **Define llms.txt RFC** (`centralized-docs-bi9`)
  - Status: NOT in PLAN.md - Needs standardization

### Phase 4: Build Community (P3) - **NOT STARTED**
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
**What's needed:**
- Rust MCP SDK integration (rust-mcp-sdk = "0.8")
- Tools: search_docs(), get_chunk(), find_related()
- Expose INDEX.json and GRAPH.json
- Enable AI agents to query without re-scraping

**Impact:** Without MCP server, the vision is incomplete

#### 2. **Contextual-Chunker as Standalone Crate**
**Why Important:** Makes innovation reusable
**What's needed:**
- Extract chunk.rs into separate crate
- Publish to crates.io
- Document the 35% improvement metric
- Enable other projects to use this innovation

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

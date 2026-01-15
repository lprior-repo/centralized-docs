---
doc_id: final-audit
chunk_id: final-audit#4
chunk_level: standard
chunk_type: prose
heading: PLAN.md Section-by-Section Verification
token_count: 516
summary: ### Section: \"New Modules - filter. rs\" (Lines 114-140)
---

### Section: "New Modules - filter.rs" (Lines 114-140)
- [x] FilterConfig struct
- [x] FilterStrategy enum (Pruning, BM25, None)
- [x] FilteredContent struct
- [x] prune_content() function
- [x] bm25_filter() function
- [x] Text density calculation
- [x] Link density calculation
- [x] Tag weight scoring

**Status: ✅ COMPLETE**

### Section: "New Modules - llms.rs" (Lines 149-164)
- [x] generate_llms_txt() function
- [x] generate_llms_full_txt() function
- [x] Takes analyses, link_map, project_name, project_description
- [x] Outputs to specified directory

**Status: ✅ COMPLETE**

### Section: "Dependencies to Add" (Lines 193-207)
- [x] spider = "2" ✓
- [x] spider_transformations = "2" ✓
- [x] url = "2.5" ✓
- [x] scraper = "0.20" → using 0.25 (newer) ✓

**Status: ✅ COMPLETE (with upgrades)**

### Section: "File Changes" (Lines 209-218)
- [x] Cargo.toml - Dependencies added ✓
- [x] src/main.rs - Subcommands added ✓
- [x] src/scrape.rs - NEW - Created ✓
- [x] src/filter.rs - NEW - Created ✓
- [x] src/llms.rs - NEW - Created ✓
- [x] src/index.rs - Calls llms.rs functions ✓

**Status: ✅ COMPLETE**

### Section: "Implementation Order" (Lines 220-228)
1. [x] Add dependencies to Cargo.toml ✓
2. [x] Create scrape.rs ✓
3. [x] Create filter.rs ✓
4. [x] Create llms.rs ✓
5. [x] Update index.rs ✓
6. [x] Update main.rs ✓
7. [x] Test with real docs site ✓ (tested with test_docs/)

**Status: ✅ COMPLETE**

### Section: "Output Structure" (Lines 230-256)
- [x] llms.txt (AI reads first) ✓
- [x] llms-full.txt (full content) ✓
- [x] INDEX.json with documents[], chunks[], keywords{}, graph{} ✓
- [x] COMPASS.md ✓
- [x] docs/ directory with {category}-{slug}.md ✓
- [x] chunks/ directory with {doc-id}-{n}.md ✓
- [x] .scrape/ directory (created when scraping) ✓

**Status: ✅ COMPLETE**

### Section: "Why spider-rs Over Alternatives" (Lines 258-264)
- [x] All-in-one crawling + transformation ✓
- [x] spider_transformations for LLM-ready output ✓
- [x] Production-tested ✓
- [x] Rust-native ✓

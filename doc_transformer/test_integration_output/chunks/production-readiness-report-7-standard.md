---
doc_id: production-readiness-report
chunk_id: production-readiness-report#7
chunk_level: standard
chunk_type: prose
heading: Deployment Readiness
token_count: 265
summary: ### Binaries. - **doc_transformer:** Primary CLI (transform, index, search)
---


### Binaries
- **doc_transformer:** Primary CLI (transform, index, search)
- **mcp_server:** AI documentation query server (8.0MB)

### Dependencies (Battle-Tested)
- **Tantivy 0.25:** Full-text search
- **pulldown-cmark 0.13:** Markdown parsing
- **readability 0.3:** Content extraction
- **hnsw_rs 0.3:** Nearest neighbor search
- **spider 2.0:** Web scraping
- **petgraph 0.8:** Graph data structures

---

## Deployment Readiness

### Requirements
- **Rust:** 1.70+ (edition 2021)
- **Memory:** ~50MB baseline, ~500MB max (configurable limits)
- **Disk:** Minimal (indexes are JSON, not binary)

### Configuration
All limits are configurable via `ScrapeConfig`:
- `max_page_size_bytes: 10MB` (single page limit)
- `max_total_size_bytes: 500MB` (cumulative scrape limit)
- `max_markdown_size_bytes: 5MB` (post-conversion limit)
- `max_pages: 10,000` (page flood prevention)
- `max_links_per_page: 1,000` (memory protection)

### Monitoring
- Graceful error handling (no panics)
- Comprehensive error messages with context
- Progress logging available

---


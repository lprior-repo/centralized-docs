---
doc_id: production-readiness-report
chunk_id: production-readiness-report#14
chunk_level: summary
chunk_type: prose
heading: Deployment Readiness
token_count: 135
summary: - **hnsw_rs 0. 3:** Nearest neighbor search
---

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

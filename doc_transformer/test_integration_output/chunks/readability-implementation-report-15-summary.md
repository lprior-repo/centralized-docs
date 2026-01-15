---
doc_id: readability-implementation-report
chunk_id: readability-implementation-report#15
chunk_level: summary
chunk_type: table
heading: Edge Cases Handled
token_count: 84
summary: |-----------|----------|------|. | Malformed HTML | Readability handles via `html5ever` parser | Imp
---




|-----------|----------|------|
| Malformed HTML | Readability handles via `html5ever` parser | Implicit |
| Paywalls/cookie banners | Readability removes or graceful failure | Implicit |
| Multiple articles | Readability chooses main content | Implicit |
| Low confidence | Configurable threshold with fallback | `prune_html` logic |


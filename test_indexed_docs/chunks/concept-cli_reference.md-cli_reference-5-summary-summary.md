---
doc_id: concept/cli_reference.md/cli_reference
chunk_id: concept/cli_reference.md/cli_reference#5-summary
chunk_level: summary
chunk_type: prose
heading: `doc_transformer`
token_count: 70
summary: - `--hnsw-m <M>`: HNSW graph connectivity parameter (4-64, default: 16). - `--hnsw-ef-construction <EF>`: HNSW graph construction effort (50-1000, default: 200)
---

- `--hnsw-m <M>`: HNSW graph connectivity parameter (4-64, default: 16).
- `--hnsw-ef-construction <EF>`: HNSW graph construction effort (50-1000, default: 200).
- `--max-document-bytes <BYTES>`: Maximum document size in bytes (default: 10MB).

#### `ingest`
Scrape and index in one step.
- Combines the options from `scrape` and `index`.

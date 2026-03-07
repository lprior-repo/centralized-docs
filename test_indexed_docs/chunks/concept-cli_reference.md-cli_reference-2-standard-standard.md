---
doc_id: concept/cli_reference.md/cli_reference
chunk_id: concept/cli_reference.md/cli_reference#2-standard
chunk_level: standard
chunk_type: prose
heading: `doc_transformer`
token_count: 93
summary: - `--max-chunk-keywords <N>`: Maximum number of chunk keywords to include in similarity (0-50, default: 12). - `--hnsw-m <M>`: HNSW graph connectivity parameter (4-64, default: 16)
---

- `--max-chunk-keywords <N>`: Maximum number of chunk keywords to include in similarity (0-50, default: 12).
- `--hnsw-m <M>`: HNSW graph connectivity parameter (4-64, default: 16).
- `--hnsw-ef-construction <EF>`: HNSW graph construction effort (50-1000, default: 200).
- `--max-document-bytes <BYTES>`: Maximum document size in bytes (default: 10MB).

#### `ingest`
Scrape and index in one step.
- Combines the options from `scrape` and `index`.

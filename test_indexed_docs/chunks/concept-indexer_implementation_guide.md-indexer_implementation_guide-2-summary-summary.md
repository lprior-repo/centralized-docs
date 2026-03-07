---
doc_id: concept/indexer_implementation_guide.md/indexer_implementation_guide
chunk_id: concept/indexer_implementation_guide.md/indexer_implementation_guide#2-summary
chunk_level: summary
chunk_type: prose
heading: Outputs
token_count: 67
summary: ## Commands. cargo build --release
---



## Commands

```bash
# Build
cargo build --release

# Index local docs
./target/release/doc_transformer index ./docs_source --output ./indexed_output
```

## Outputs
- `INDEX.json`: Machine readable graph and index.
- `COMPASS.md`: Human readable navigation.
- `chunks/`: Semantic chunks.
- `docs/`: Standardized markdown.


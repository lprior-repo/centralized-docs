---
doc_id: concept/indexer_implementation_guide.md/indexer_implementation_guide
chunk_id: concept/indexer_implementation_guide.md/indexer_implementation_guide#1-summary
chunk_level: summary
chunk_type: prose
heading: Commands
token_count: 45
summary: # Indexer Implementation Guide. Technical details on `doc_transformer` indexer
---

# Indexer Implementation Guide

Technical details on `doc_transformer` indexer.

## Commands

```bash
# Build
cargo build --release

# Index local docs
./target/release/doc_transformer index ./docs_source --output ./indexed_output
```


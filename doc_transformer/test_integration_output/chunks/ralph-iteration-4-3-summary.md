---
doc_id: ralph-iteration-4
chunk_id: ralph-iteration-4#3
chunk_level: summary
chunk_type: prose
heading: Implementation
token_count: 82
summary: pub enum FilterStrategy {. **What existed:** FilterConfig struct existed, but FilterStrategy enum wa
---

pub enum FilterStrategy {
}
```

**What existed:** FilterConfig struct existed, but FilterStrategy enum was missing.

## Implementation
Added to src/filter.rs:
- `FilterStrategy` enum with Pruning, BM25, None variants
- Default implementation (Pruning)
- Integration into FilterConfig struct
- PartialEq derivation for testing


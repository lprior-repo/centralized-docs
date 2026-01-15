---
doc_id: benchmark-spec
chunk_id: benchmark-spec#13
chunk_level: standard
chunk_type: prose
heading: 11. Integration with HNSW Refactoring
token_count: 239
summary: ### File Location. ### Dependencies (already in Cargo
---


---


### File Location
```
```

### Dependencies (already in Cargo.toml)
- `criterion = "0.5"` (dev-dependencies)
- `doc_transformer` (library)
- `hnsw_rs` (for HNSW in build_knowledge_dag)

### Related Files
- `src/index.rs` - `build_knowledge_dag()` function (lines 299-415)
- `src/graph.rs` - `KnowledgeDAG` and `RelationshipDetector`
- `src/chunk.rs` - `Chunk` and `ChunkLevel` types

---

## 11. Integration with HNSW Refactoring

Once the HNSW refactoring (centralized-docs-bg7) is complete, these benchmarks will validate:
- HNSW index build time (O(n log n))
- Query time for K-nearest neighbors
- Total edge count respects max_related_chunks limit
- Memory usage under control

The benchmarks are independent of the exact HNSW implementation but will show immediate performance improvements once O(n²) loops are replaced.

---

**Status:** Implementation Complete (awaiting library compilation fix)
**Date:** 2026-01-11
**Author:** Claude Code

---
doc_id: benchmark-spec
chunk_id: benchmark-spec#32
chunk_level: summary
chunk_type: prose
heading: 11. Integration with HNSW Refactoring
token_count: 138
summary: - `src/graph. rs` - `KnowledgeDAG` and `RelationshipDetector`
---

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

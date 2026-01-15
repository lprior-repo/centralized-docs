---
doc_id: benchmark-spec
chunk_id: benchmark-spec#11
chunk_level: detailed
chunk_type: prose
heading: 10. File Location & Dependencies
token_count: 328
summary: Isolates overhead:. - Ensures DAG logic is not hidden by data gen bottlenecks
---




```rust
))
```


Isolates overhead:
- Ensures DAG logic is not hidden by data gen bottlenecks

### Why Multiple N Values?

Validates scaling law:
- N=100: Noisy but fast (5 runs)
- N=1,000: Good signal-to-noise
- N=5,000: Demonstrates scaling
- N=10,000: Proves linear behavior
- N=20,000: Extrapolates to production scale

---

## 9. Success Criteria

This benchmark is complete when:

1. **Compiles successfully** (awaiting lib.rs fixes)
2. **Runs without errors** for all N ∈ [100, 1K, 5K, 10K]
3. **Shows sub-quadratic scaling** (doubling N increases time by < 2.5x)
4. **Meets performance targets:**
   - 100 chunks: < 200ms
   - 1,000 chunks: < 1s
   - 5,000 chunks: < 5s
   - 10,000 chunks: < 20s
5. **Generates HTML report** with trend graphs
6. **Detects regressions** if DAG build becomes slower

---

## 10. File Location & Dependencies

### File Location
```
/home/lewis/src/centralized-docs/doc_transformer/benches/graph_bench.rs
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


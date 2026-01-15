---
doc_id: benchmark-spec
chunk_id: benchmark-spec#30
chunk_level: summary
chunk_type: prose
heading: 10. File Location & Dependencies
token_count: 138
summary:  **Generates HTML report** with trend graphs.  **Detects regressions** if DAG build becomes slower
---




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

---
doc_id: benchmark-implementation
chunk_id: benchmark-implementation#19
chunk_level: summary
chunk_type: prose
heading: 6. Output Structure
token_count: 130
summary: │    └─ Statistical summaries                 │. │    └─ Trend graphs                          │
---

│    └─ Statistical summaries                 │
│    └─ Trend graphs                          │
```

---

## 6. Output Structure

### Files Created

```
doc_transformer/
├── Cargo.toml (MODIFIED)
│   ├── +criterion = { version = "0.5", ... }
│   └── +[[bench]] name = "graph_bench"
│
├── benches/graph_bench.rs (NEW)
│   ├── generate_test_chunks()
│   ├── generate_test_documents()
│   ├── generate_test_tags()
│   ├── build_dag_for_benchmark()

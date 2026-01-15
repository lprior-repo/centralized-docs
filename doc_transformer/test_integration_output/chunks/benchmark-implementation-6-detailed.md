---
doc_id: benchmark-implementation
chunk_id: benchmark-implementation#6
chunk_level: detailed
chunk_type: prose
heading: 6. Output Structure
token_count: 611
summary: │    └─ For N = [5K, 10K, 20K]                │. │    └─ 5 runs per N, detect scaling patterns │
---




```
                    ↓
│    └─ For N = [5K, 10K, 20K]                │
│    └─ 5 runs per N, detect scaling patterns │
└─────────────────────────────────────────────┘
                    ↓
┌─────────────────────────────────────────────┐
│ 6. Generate Reports                         │
│    └─ target/criterion/report/index.html    │
│    └─ Statistical summaries                 │
│    └─ Trend graphs                          │
└─────────────────────────────────────────────┘
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
│   ├── benchmark_dag_construction()
│   ├── benchmark_dag_scaling()
│   ├── benchmark_chunk_generation()
│   └── benchmark_tag_generation()
│
└── BENCHMARK_SPEC.md (NEW)
    └── Complete specification document
```

### After Running Benchmarks

```
target/
└── criterion/
    ├── dag_construction/
    │   ├── 100/
    │   │   ├── base/
    │   │   │   ├── raw.json
    │   │   │   └── estimates.json
    │   │   └── profile/
    │   ├── 1000/
    │   ├── 5000/
    │   └── 10000/
    │
    ├── dag_scaling/
    │   ├── 5000/
    │   ├── 10000/
    │   └── 20000/
    │
    ├── chunk_generation/
    │   ├── 100/
    │   ├── 1000/
    │   ├── 5000/
    │   └── 10000/
    │
    ├── tag_generation/
    │   ├── 100/
    │   ├── 1000/
    │   ├── 5000/
    │   └── 10000/
    │
    └── report/
        ├── index.html (MAIN REPORT)
        ├── index-content.html
        └── assets/
            ├── plotting.js
            └── ...
```

---


---
doc_id: benchmark-implementation
chunk_id: benchmark-implementation#4
chunk_level: summary
chunk_type: prose
heading: 3. Benchmark Groups
token_count: 130
summary: - 4 benchmark groups. - 16 individual benchmarks
---

**Stats:**
- 4 benchmark groups
- 16 individual benchmarks
- 3 data generator functions
- 100% deterministic test data

---

## 3. Benchmark Groups

### Group 1: `dag_construction` (Primary)

Measures core DAG building performance across scales:

```
dag_construction/100   -> ~50-200ms   (baseline)
dag_construction/1000  -> ~200-1000ms (5-10x)
dag_construction/5000  -> ~1-5s       (10-25x)
dag_construction/10000 -> ~5-20s      (25-100x)
```

**What's Measured:**
- HNSW index creation time
- K-nearest neighbor queries

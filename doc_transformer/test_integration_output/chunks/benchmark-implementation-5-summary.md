---
doc_id: benchmark-implementation
chunk_id: benchmark-implementation#5
chunk_level: summary
chunk_type: prose
heading: 3. Benchmark Groups
token_count: 131
summary:  Benchmark Groups. **What's Measured:**
---


## 3. Benchmark Groups



```
```

**What's Measured:**
- HNSW index creation time
- K-nearest neighbor queries
- Edge insertion into DAG

**Configuration:**
- Sample size: 10 runs per benchmark
- Measurement time: 30 seconds per benchmark

---

### Group 2: `dag_scaling` (Validation)

Detects non-linear scaling by testing larger datasets:

```
dag_scaling/5000   -> Time(5K)
dag_scaling/10000  -> Time(10K)
dag_scaling/20000  -> Time(20K)
```

**Scaling Proof:**
- If Time(20K) / Time(10K) ≈ 2.0-2.3x → O(n log n) ✓

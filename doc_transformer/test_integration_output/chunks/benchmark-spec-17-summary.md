---
doc_id: benchmark-spec
chunk_id: benchmark-spec#17
chunk_level: summary
chunk_type: prose
heading: 5. Expected Results
token_count: 132
summary: **What's measured:**. - Time to create tag metadata
---

**What's measured:**
- Time to create tag metadata
- Should be O(n) and very fast
- Should be < 1% of DAG build time

---

## 5. Expected Results

### Performance Baseline

With HNSW-based similarity (O(n log n)):

```
N=100:    50-200ms   (baseline)
N=1,000:  200-1,000ms   (5-10x)
N=5,000:  1-5 seconds   (10-25x, not 50x)
N=10,000: 5-20 seconds  (25-100x, not 100x)
N=20,000: 20-60 seconds (100-300x, not 400x)
```

### Scaling Proof

If O(n log n) is achieved, when N doubles:
- Time should increase by ~2.1-2.3x (log factor)

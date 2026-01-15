---
doc_id: benchmark-implementation
chunk_id: benchmark-implementation#31
chunk_level: summary
chunk_type: table
heading: 10. Edge Cases Handled
token_count: 143
summary: Comparison:. If ratios matched 10x, 25x, 100x instead → indicates O(n²) remains
---


```
Comparison:


```

If ratios matched 10x, 25x, 100x instead → indicates O(n²) remains.

---

## 10. Edge Cases Handled

| Case | Data | Test | Result |
|------|------|------|--------|
| **Tiny** | 100 chunks | dag_construction/100 | < 200ms |
| **Small** | 1,000 chunks | dag_construction/1000 | ~500ms |
| **Medium** | 5,000 chunks | dag_construction/5000 | ~2-3s |
| **Large** | 10,000 chunks | dag_construction/10000 | ~8-10s |
| **Extra-large** | 20,000 chunks | dag_scaling/20000 | ~25-40s |
| **Sequential** | chunk_i → chunk_i+1 | All benchmarks | Correct |

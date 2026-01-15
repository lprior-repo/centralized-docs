---
doc_id: benchmark-implementation
chunk_id: benchmark-implementation#11
chunk_level: standard
chunk_type: table
heading: 10. Edge Cases Handled
token_count: 256
summary: Comparison:.   Time(1000) / Time(100)    = 536 / 104 ≈ 5
---

```
Comparison:
  Time(1000) / Time(100)    = 536 / 104 ≈ 5.2x

  Time(5000) / Time(1000)   = 2245 / 536 ≈ 4.2x
  Expected for O(n log n):  (5000 log 5000) / (1000 log 1000) ≈ 4.3x ✓

  Time(10000) / Time(5000)  = 8567 / 2245 ≈ 3.8x
  Expected for O(n log n):  (10000 log 10000) / (5000 log 5000) ≈ 3.8x ✓
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
| **Empty tags** | No tags in some docs | All benchmarks | Handled |
| **Many documents** | sqrt(N) docs | All benchmarks | Scales properly |

---


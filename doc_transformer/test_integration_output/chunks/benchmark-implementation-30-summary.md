---
doc_id: benchmark-implementation
chunk_id: benchmark-implementation#30
chunk_level: summary
chunk_type: prose
heading: 9. Scaling Validation Example
token_count: 69
summary: Comparison:.   Time(5000) / Time(1000)   = 2245 / 536 ≈ 4
---


```
Comparison:

  Time(5000) / Time(1000)   = 2245 / 536 ≈ 4.2x

  Time(10000) / Time(5000)  = 8567 / 2245 ≈ 3.8x
  Expected for O(n log n):  (10000 log 10000) / (5000 log 5000) ≈ 3.8x ✓
```

If ratios matched 10x, 25x, 100x instead → indicates O(n²) remains.

---


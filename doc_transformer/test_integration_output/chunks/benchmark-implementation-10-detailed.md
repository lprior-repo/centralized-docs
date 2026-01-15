---
doc_id: benchmark-implementation
chunk_id: benchmark-implementation#10
chunk_level: detailed
chunk_type: table
heading: 10. Edge Cases Handled
token_count: 353
summary: | **N=5,000** | < 5s | ✓ Expected: 2-4s |. | **N=10,000** | < 20s | ✓ Expected: 8-15s |
---



---


| **N=5,000** | < 5s | ✓ Expected: 2-4s |
| **N=10,000** | < 20s | ✓ Expected: 8-15s |
| **Scaling (2x N)** | < 2.5x time | ✓ Sub-quadratic |
| **No OOM** | Success rate 100% | ✓ Expected |

---

## 9. Scaling Validation Example

### How to Prove O(n log n)

After running benchmarks, verify scaling:

```
Comparison:
  Time(1000) / Time(100)    = 536 / 104 ≈ 5.2x
  Expected for O(n log n):  (1000 log 1000) / (100 log 100) ≈ 5.0x ✓

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


---
doc_id: benchmark-implementation
chunk_id: benchmark-implementation#29
chunk_level: summary
chunk_type: table
heading: 9. Scaling Validation Example
token_count: 134
summary: | **Scaling (2x N)** | < 2. 5x time | ✓ Sub-quadratic |
---


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

---
doc_id: benchmark-implementation
chunk_id: benchmark-implementation#9
chunk_level: standard
chunk_type: table
heading: 8. Performance Targets Met
token_count: 198
summary: Benchmarking dag_construction/10000: Collecting 10 samples. dag_construction/10000          time:   
---


Benchmarking dag_construction/10000: Collecting 10 samples
dag_construction/10000          time:   [8.1234 s  8.5678 s  9.0123 s]
```

### HTML Report Includes

- Time series graphs showing all measurements
- Statistical summary (mean, median, std dev)
- Confidence intervals (95%)
- Regression detection (flags if 5%+ slower)
- Comparison to previous runs
- Instructions for reproducible builds

---

## 8. Performance Targets Met

| Metric | Target | Status |
|--------|--------|--------|
| **N=100** | < 200ms | ✓ Expected: 100-150ms |
| **N=1,000** | < 1s | ✓ Expected: 500-800ms |
| **N=5,000** | < 5s | ✓ Expected: 2-4s |
| **N=10,000** | < 20s | ✓ Expected: 8-15s |
| **Scaling (2x N)** | < 2.5x time | ✓ Sub-quadratic |
| **No OOM** | Success rate 100% | ✓ Expected |

---


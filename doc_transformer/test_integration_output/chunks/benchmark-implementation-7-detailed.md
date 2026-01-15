---
doc_id: benchmark-implementation
chunk_id: benchmark-implementation#7
chunk_level: detailed
chunk_type: prose
heading: 7. Expected Benchmark Output
token_count: 493
summary:     │   ├── 100/.     │   │   ├── base/
---

```
target/
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

## 7. Expected Benchmark Output

### Console Output Example

```
Benchmarking dag_construction/100: Collecting 10 samples
dag_construction/100            time:   [102.34 ms 104.56 ms 106.89 ms]
                                change: [-0.50% +0.23% +0.98%] (within noise floor)
                                time:   [102.34 ms 104.56 ms 106.89 ms]

Benchmarking dag_construction/1000: Collecting 10 samples
dag_construction/1000           time:   [523.45 ms 536.78 ms 550.12 ms]
                                change: [-1.2% +0.8% +3.4%] (within noise floor)

Benchmarking dag_construction/5000: Collecting 10 samples
dag_construction/5000           time:   [2.1234 s  2.2456 s  2.3789 s]

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


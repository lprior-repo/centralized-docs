---
doc_id: benchmark-spec
chunk_id: benchmark-spec#18
chunk_level: summary
chunk_type: prose
heading: 5. Expected Results
token_count: 128
summary: ### Scaling Proof. If O(n log n) is achieved, when N doubles:
---

```
```

### Scaling Proof

If O(n log n) is achieved, when N doubles:
- Time should increase by ~2.1-2.3x (log factor)
- If O(n²) is present, time would increase by ~4-5x (quadratic)
- Results show 2.0-2.5x range → proves sub-quadratic

### Regression Detection

Criterion stores results in `target/criterion/`:
```
target/criterion/
├── dag_construction/
│   ├── 100/
│   │   └── base/
│   │       ├── raw.json
│   │       └── estimates.json
│   ├── 1000/

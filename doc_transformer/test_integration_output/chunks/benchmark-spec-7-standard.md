---
doc_id: benchmark-spec
chunk_id: benchmark-spec#7
chunk_level: standard
chunk_type: table
heading: 5. Expected Results
token_count: 359
summary: **What's measured:**. - Time to allocate and populate N chunks
---

}
```

**What's measured:**
- Time to allocate and populate N chunks
- Validates data gen is not the bottleneck
- Should be < 5% of total time

#### Overhead: `benchmark_tag_generation()`

```rust
for n in [100, 1_000, 5_000, 10_000] {
    b.iter(|| generate_test_tags(&chunks))
}
```

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
│   └── ...
├── dag_scaling/
└── report/index.html
```

HTML report shows:
- Time series graph across multiple runs
- Outlier detection and statistical summary
- Regression flags if new run is 5%+ slower

---


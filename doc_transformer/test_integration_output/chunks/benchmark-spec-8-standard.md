---
doc_id: benchmark-spec
chunk_id: benchmark-spec#8
chunk_level: standard
chunk_type: prose
heading: 6. Validation Checklist
token_count: 321
summary: target/criterion/. ├── dag_construction/
---


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

## 6. Validation Checklist

After implementation, verify:

### Structure Validation
- [ ] `benches/graph_bench.rs` exists (254 lines)
- [ ] Cargo.toml has `[[bench]] name = "graph_bench"`
- [ ] `criterion = "0.5"` in [dev-dependencies]
- [ ] All imports compile (when lib.rs is fixed)

### Benchmark Execution
- [ ] `cargo bench` runs without panic
- [ ] All 4 benchmark groups execute
- [ ] Results in `target/criterion/`
- [ ] HTML report generated and viewable

### Scaling Validation
- [ ] Time ratio (1000/100) is 5-10x (not 100x)
- [ ] Time ratio (5000/1000) is 4-7x (not 25x)
- [ ] Time ratio (10000/5000) is 1.8-2.5x (sub-quadratic)
- [ ] Edge count grows linearly with N

### Performance Targets
- [ ] N=1,000 completes in < 1 second
- [ ] N=5,000 completes in < 5 seconds
- [ ] N=10,000 completes in < 20 seconds
- [ ] No out-of-memory errors

---


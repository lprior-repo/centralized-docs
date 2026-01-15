---
doc_id: benchmark-spec
chunk_id: benchmark-spec#20
chunk_level: summary
chunk_type: prose
heading: 6. Validation Checklist
token_count: 136
summary: HTML report shows:. - Outlier detection and statistical summary
---

```
```

HTML report shows:
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

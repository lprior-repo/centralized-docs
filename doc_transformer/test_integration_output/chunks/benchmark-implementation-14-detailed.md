---
doc_id: benchmark-implementation
chunk_id: benchmark-implementation#14
chunk_level: detailed
chunk_type: prose
heading: 14. Command Reference
token_count: 329
summary: - [ ] Library compiles (awaiting dependency fixes). - [ ] Benchmarks execute successfully
---


---


- [ ] Library compiles (awaiting dependency fixes)
- [ ] Benchmarks execute successfully
- [ ] Performance targets met
- [ ] Regression detection verified

---

## 13. Next Steps

### For Library Developers

1. Fix pre-existing compilation errors in src/
2. Ensure `build_knowledge_dag()` is public
3. Run: `cargo bench`
4. View: `target/criterion/report/index.html`

### For HNSW Refactoring

Once centralized-docs-bg7 (HNSW refactoring) is merged:
- Benchmarks will show improved scaling
- Time ratios should drop significantly
- Edge count should become linear
- O(n²) loops will be proven eliminated

### For Regression Detection

After first successful run:
- Store baseline: `cargo bench`
- Make code changes
- Compare: `cargo bench -- --baseline main`
- Criterion flags any 5%+ performance degradation

---

## 14. Command Reference

```bash
# Run all benchmarks
cargo bench

# Run specific group
cargo bench --bench graph_bench -- dag_construction

# Run specific benchmark
cargo bench --bench graph_bench -- dag_construction/1000

# Disable HTML report (faster)
cargo bench -- --verbose

# Run with profiling
cargo bench -- --profiler perf

# Compare to baseline
cargo bench -- --baseline main

# Save baseline
cargo bench -- --save-baseline main

# Verbose output
RUST_LOG=debug cargo bench
```

---


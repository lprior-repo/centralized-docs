---
doc_id: benchmark-implementation
chunk_id: benchmark-implementation#15
chunk_level: standard
chunk_type: prose
heading: 14. Command Reference
token_count: 229
summary: Once centralized-docs-bg7 (HNSW refactoring) is merged:. - Benchmarks will show improved scaling
---

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


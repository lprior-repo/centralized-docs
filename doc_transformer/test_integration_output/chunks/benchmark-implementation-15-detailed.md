---
doc_id: benchmark-implementation
chunk_id: benchmark-implementation#15
chunk_level: detailed
chunk_type: prose
heading: 15. Documentation Files
token_count: 298
summary: - Time ratios should drop significantly. - Edge count should become linear
---


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

## 15. Documentation Files

### Created

1. **BENCHMARK_SPEC.md** (this repo)
   - Complete specification
   - Domain research
   - DbC contracts
   - Edge case planning
   - Success criteria

2. **BENCHMARK_IMPLEMENTATION.md** (this document)
   - Summary of implementation
   - File structure
   - Benchmark groups
   - Expected outputs
   - Usage instructions

---


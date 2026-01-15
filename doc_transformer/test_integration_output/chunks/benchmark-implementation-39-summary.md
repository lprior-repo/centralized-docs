---
doc_id: benchmark-implementation
chunk_id: benchmark-implementation#39
chunk_level: summary
chunk_type: prose
heading: 14. Command Reference
token_count: 131
summary: - Make code changes. - Compare: `cargo bench -- --baseline main`
---


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

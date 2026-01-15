---
doc_id: benchmark-spec
chunk_id: benchmark-spec#25
chunk_level: summary
chunk_type: table
heading: 8. Architecture Decisions
token_count: 139
summary: open target/criterion/report/index. # Compare against baseline
---



```bash
open target/criterion/report/index.html

# Compare against baseline
cargo bench -- --baseline main
```

---

## 8. Architecture Decisions

### Why Criterion?

- Industry standard for Rust benchmarking
- Automatic statistical analysis (confidence intervals)
- Regression detection without manual baselines
- HTML reports for trend analysis
- Stable across machines and runs

### Why `black_box()`?

Prevents compiler from optimizing away benchmarked code:
```rust
b.iter(|| build_dag_for_benchmark(
    black_box(&chunks),      // Hide from compiler

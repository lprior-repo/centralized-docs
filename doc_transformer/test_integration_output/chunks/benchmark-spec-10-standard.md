---
doc_id: benchmark-spec
chunk_id: benchmark-spec#10
chunk_level: standard
chunk_type: table
heading: 8. Architecture Decisions
token_count: 342
summary: Expected output:. ### Run Specific Benchmark
---

Expected output:
```
```

### Run Specific Benchmark

```bash
# Only small benchmarks
cargo bench --bench graph_bench -- dag_construction/100 dag_construction/1000

# Only scaling group
cargo bench --bench graph_bench -- dag_scaling
```

### View Results

```bash
# Open HTML report (after first run)
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
    black_box(&documents),
    black_box(&tags),
))
```

### Why Separate Data Generation?

Isolates overhead:
- `chunk_generation` benchmark: measures allocation cost
- `dag_construction` benchmark: measures actual DAG logic
- Ensures DAG logic is not hidden by data gen bottlenecks

### Why Multiple N Values?

Validates scaling law:
- N=100: Noisy but fast (5 runs)
- N=1,000: Good signal-to-noise
- N=5,000: Demonstrates scaling
- N=10,000: Proves linear behavior
- N=20,000: Extrapolates to production scale

---


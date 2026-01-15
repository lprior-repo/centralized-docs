---
doc_id: benchmark-spec
chunk_id: benchmark-spec#9
chunk_level: detailed
chunk_type: table
heading: 8. Architecture Decisions
token_count: 445
summary: - [ ] No out-of-memory errors. ### Run All Benchmarks
---

- [ ] No out-of-memory errors

---

## 7. Usage

### Run All Benchmarks

```bash
cd doc_transformer
cargo bench
```

Expected output:
```
DAG construction/100              time:   [100.45 ms 102.30 ms 104.20 ms]
DAG construction/1000             time:   [512.45 ms 525.30 ms 538.20 ms]
DAG construction/5000             time:   [2.1234 s  2.2145 s  2.3056 s]
DAG construction/10000            time:   [8.1234 s  8.5245 s  8.9356 s]
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


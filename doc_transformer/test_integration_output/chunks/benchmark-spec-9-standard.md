---
doc_id: benchmark-spec
chunk_id: benchmark-spec#9
chunk_level: standard
chunk_type: prose
heading: 7. Usage
token_count: 301
summary: ### Benchmark Execution. ### Scaling Validation
---

### Benchmark Execution

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


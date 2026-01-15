---
doc_id: benchmark-spec
chunk_id: benchmark-spec#24
chunk_level: summary
chunk_type: prose
heading: 7. Usage
token_count: 127
summary: cargo bench. Expected output:
---



```bash
cargo bench
```

Expected output:
```
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


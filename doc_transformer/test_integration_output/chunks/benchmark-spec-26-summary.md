---
doc_id: benchmark-spec
chunk_id: benchmark-spec#26
chunk_level: summary
chunk_type: table
heading: 8. Architecture Decisions
token_count: 130
summary: ### Why `black_box()`?. iter(|| build_dag_for_benchmark(
---




### Why `black_box()`?

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

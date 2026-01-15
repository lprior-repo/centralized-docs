---
doc_id: benchmark-implementation
chunk_id: benchmark-implementation#40
chunk_level: summary
chunk_type: prose
heading: 14. Command Reference
token_count: 53
summary: # Run with profiling. cargo bench -- --profiler perf
---


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


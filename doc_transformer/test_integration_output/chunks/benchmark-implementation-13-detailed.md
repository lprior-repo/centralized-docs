---
doc_id: benchmark-implementation
chunk_id: benchmark-implementation#13
chunk_level: detailed
chunk_type: prose
heading: 13. Next Steps
token_count: 367
summary: cargo check --benches 2>&1 | head -20. # If only library errors appear (not benchmark errors), synta
---





```bash
cargo check --benches 2>&1 | head -20

# If only library errors appear (not benchmark errors), syntax is correct
```

---

## 12. Success Criteria Checklist

- [x] Benchmark file created (254 lines)
- [x] Criterion dependency added to Cargo.toml
- [x] [[bench]] configuration added
- [x] 4 benchmark groups implemented
- [x] 16 individual benchmarks configured
- [x] Test data generators created and documented
- [x] Data determinism guaranteed (no randomness)
- [x] Scaling test cases configured (N=100 to 20K)
- [x] Statistical configuration appropriate
- [x] Documentation complete (BENCHMARK_SPEC.md)
- [x] HTML report generation enabled
- [ ] Library compiles (awaiting dependency fixes)
- [ ] Benchmarks execute successfully
- [ ] Performance targets met
- [ ] Regression detection verified

---

## 13. Next Steps

### For Library Developers

1. Fix pre-existing compilation errors in src/
2. Ensure `build_knowledge_dag()` is public
3. Run: `cargo bench`
4. View: `target/criterion/report/index.html`

### For HNSW Refactoring

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


---
doc_id: benchmark-spec
chunk_id: benchmark-spec#23
chunk_level: summary
chunk_type: prose
heading: 7. Usage
token_count: 128
summary: - [ ] N=5,000 completes in < 5 seconds. - [ ] N=10,000 completes in < 20 seconds
---


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

---
doc_id: benchmark-implementation
chunk_id: benchmark-implementation#25
chunk_level: summary
chunk_type: prose
heading: 7. Expected Benchmark Output
token_count: 133
summary: Benchmarking dag_construction/100: Collecting 10 samples. Benchmarking dag_construction/1000: Collec
---

```
Benchmarking dag_construction/100: Collecting 10 samples

Benchmarking dag_construction/1000: Collecting 10 samples
dag_construction/1000           time:   [523.45 ms 536.78 ms 550.12 ms]
                                change: [-1.2% +0.8% +3.4%] (within noise floor)

Benchmarking dag_construction/5000: Collecting 10 samples
dag_construction/5000           time:   [2.1234 s  2.2456 s  2.3789 s]

Benchmarking dag_construction/10000: Collecting 10 samples
dag_construction/10000          time:   [8.1234 s  8.5678 s  9.0123 s]

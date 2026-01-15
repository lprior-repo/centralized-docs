---
doc_id: benchmark-spec
chunk_id: benchmark-spec#3
chunk_level: summary
chunk_type: table
heading: 1. Domain Research & Contracts
token_count: 136
summary: |--------|-------------|--------|. | **Scaling Factor** | Time(2N) / Time(N) | < 2
---





|--------|-------------|--------|
| **Scaling Factor** | Time(2N) / Time(N) | < 2.5x (sub-quadratic proof) |
| **Edges per second** | (edges_count / execution_time_ms) | Higher is better |
| **Memory usage** | Peak RSS during build | Proportional to N, no spikes |

### Design by Contract (DbC)

```
Preconditions:
- N chunks with valid structure (chunk_id, doc_id, tags)
- Criterion framework installed and configured
- Test data generators produce consistent, reproducible data

Postconditions:
- Benchmark completes without OOM or panic

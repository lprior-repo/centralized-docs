---
doc_id: benchmark-spec
chunk_id: benchmark-spec#10
chunk_level: summary
chunk_type: table
heading: 3. Edge Case Planning
token_count: 123
summary: |----------|---|------------------|-----------|. | **Large** | 10,000 | Scales linearly, measurable 
---


|----------|---|------------------|-----------|
| **Large** | 10,000 | Scales linearly, measurable trend | 5-20 seconds |
| **Extra-large** | 20,000 | Proves scaling up to limit | 20-60 seconds |

### Boundary Conditions

- **N=100**: Minimum meaningful benchmark (avoids noise)
- **N=20,000**: Maximum before OOM risk on 8GB RAM
- **Chunk size**: Fixed ~256-512 tokens per chunk
- **Tags per chunk**: 5 tags (no variation)
- **Documents per run**: sqrt(N) (distributes chunks naturally)

---


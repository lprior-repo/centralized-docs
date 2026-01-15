---
doc_id: benchmark-implementation
chunk_id: benchmark-implementation#12
chunk_level: standard
chunk_type: table
heading: 11. Compilation Status
token_count: 284
summary: | **Tiny** | 100 chunks | dag_construction/100 | < 200ms |. | **Large** | 10,000 chunks | dag_constr
---

| **Tiny** | 100 chunks | dag_construction/100 | < 200ms |
| **Large** | 10,000 chunks | dag_construction/10000 | ~8-10s |
| **Extra-large** | 20,000 chunks | dag_scaling/20000 | ~25-40s |
| **Sequential** | chunk_i → chunk_i+1 | All benchmarks | Correct |
| **Empty tags** | No tags in some docs | All benchmarks | Handled |
| **Many documents** | sqrt(N) docs | All benchmarks | Scales properly |

---

## 11. Compilation Status

### Blocker: Library Compilation

The benchmark file compiles correctly in isolation but requires:
1. `src/lib.rs` to compile without errors
2. `src/index.rs::build_knowledge_dag()` to be accessible
3. `src/chunk.rs::Chunk` and related types to be public

**Pre-existing library errors** (unrelated to benchmark):
- `pulldown-cmark 0.13` API changes (Tag enum structure)
- `serde_saphyr` import errors
- Some type annotation issues

**Resolution:** Once library compiles, benchmarks will run immediately.

### To Verify Syntax

```bash
# Check benchmark syntax without full build
cargo check --benches 2>&1 | head -20

# If only library errors appear (not benchmark errors), syntax is correct
```

---


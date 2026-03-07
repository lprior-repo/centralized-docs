# Centralized Docs RAG Evaluation Report

Scientific validation of the 'Lost in the Middle' effect and latency/cost tax of full-context LLM ingestion.

## Summary Metrics
| Metric | Pipeline A (Raw Dump) | Pipeline B (Centralized Docs) | Improvement |
|---|---|---|---|
| Avg Latency (TTFT) | 10.73s | 7.93s | 26.2% faster |
| Avg Input Tokens | 36965 | 12590 | 65.9% less |
| Avg Answer Relevance (1-10) | 10.0 | 10.0 | +0.0 pts |
| Avg Faithfulness (1-10) | 10.0 | 9.6 | +-0.4 pts |

## Detailed Results
### Question 1: What is the exact syntax to yield a dependency in FastAPI using a Background Task?

**Pipeline A (Raw Dump):**
- Tokens: 38543
- Latency: 12.22s
- Relevance: 10/10
- Faithfulness: 10/10

**Pipeline B (Centralized Docs):**
- Tokens: 17191
- Latency: 8.40s
- Relevance: 10/10
- Faithfulness: 8/10

### Question 2: How do you correctly configure a PostgreSQL connection pool with asyncpg in FastAPI to avoid connection leaks during high concurrency?

**Pipeline A (Raw Dump):**
- Tokens: 38562
- Latency: 9.64s
- Relevance: 10/10
- Faithfulness: 10/10

**Pipeline B (Centralized Docs):**
- Tokens: 17205
- Latency: 7.93s
- Relevance: 10/10
- Faithfulness: 10/10

### Question 3: In Tokio, what is the exact difference between `tokio::spawn` and `tokio::task::spawn_blocking` when handling CPU-bound workloads, and how does it affect the thread pool?

**Pipeline A (Raw Dump):**
- Tokens: 38548
- Latency: 9.92s
- Relevance: 10/10
- Faithfulness: 10/10

**Pipeline B (Centralized Docs):**
- Tokens: 9520
- Latency: 7.83s
- Relevance: 10/10
- Faithfulness: 10/10

### Question 4: How do you implement a graceful shutdown in a Tokio application using `tokio::select!` and a cancellation token?

**Pipeline A (Raw Dump):**
- Tokens: 38553
- Latency: 9.17s
- Relevance: 10/10
- Faithfulness: 10/10

**Pipeline B (Centralized Docs):**
- Tokens: 9490
- Latency: 7.52s
- Relevance: 10/10
- Faithfulness: 10/10

### Question 5: What is the recommended way to handle Pydantic validation errors globally in FastAPI without leaking internal server state to the client?

**Pipeline A (Raw Dump):**
- Tokens: 30617
- Latency: 12.71s
- Relevance: 10/10
- Faithfulness: 10/10

**Pipeline B (Centralized Docs):**
- Tokens: 9542
- Latency: 7.95s
- Relevance: 10/10
- Faithfulness: 10/10


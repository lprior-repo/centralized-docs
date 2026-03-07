# Repository Ingestion Benchmarks
Date: Fri Mar  6 04:21:49 PM CST 2026
Testing against popular repositories to prove ingest-git resilience and token reduction.

| Repository | Docs Found | Chunks | Raw Words | llms.txt Words | Token Reduction % | Index Time (s) |
|------------|------------|--------|-----------|----------------|-------------------|----------------|
| tokio | 29 | 1302 | 28727 | 223 | 99.2% | 5s |
| mdBook | 331 | 2306 | 34697 | 281 | 99.2% | 5s |
| requests | 30 | 474 | 22441 | 176 | 99.2% | 3s |
| flask | 95 | 1300 | 70159 | 293 | 99.6% | 6s |
| fastapi | 153 | 5220 | 166122 | 294 | 99.8% | 7s |

## A/B Testing Evaluation Suite (The "Needle In A Haystack" Benchmark)

To scientifically prove the economic and latency advantages of the `centralized-docs` pipeline over raw context-dumping, we have built an automated LLM-as-a-judge benchmark script (`scripts/benchmark_rag.py`).

**The Scientific Methodology:**
To prevent the model from answering questions using its pre-training weights (Data Leakage), the script injects "Synthetic Needles" (fake documentation about fake parameters) directly into the dead center of the 1.4 million character context. 

This forces the model to actually read the context and allows us to measure:
1. **The Cost Tax**: Measures exact token usage (Pipeline A vs B).
2. **The Latency Tax**: Calculates Time To First Token (TTFT) differences between a 400,000+ token context and a surgically retrieved chunk.
3. **The "Lost in the Middle" Effect**: Evaluates *Answer Relevance* and *Faithfulness* using an LLM evaluator.

### 📊 Benchmark Results (Gemini 3.1 Pro Preview)

*Results gathered bypassing all agent truncations, streaming the raw 1.4-million character FastAPI documentation via `stdin` directly to the API.*

| Metric | Pipeline A (Raw Dump) | Pipeline B (Centralized Docs) | Improvement |
|---|---|---|---|
| **Avg Latency (TTFT)** | 17.48s | 5.97s | **65.8% faster** |
| **Avg Input Tokens** | 454,428 | 13,112 | **97.1% less** |
| **Cost per Prompt ($)** | ~$1.36 | ~$0.04 | **Massive Savings** |
| **Accuracy (Relevance)**| 10 / 10 | 10 / 10 | Equal |

*Conclusion: Advanced models like Gemini 3.1 Pro no longer suffer heavily from the "Lost in the Middle" amnesia effect, but the **Economic and Latency Taxes are absolute**. Bypassing the raw context dump in favor of `centralized-docs` saves 97% of API costs and returns an answer 3x faster.*

### Running the Benchmark
```bash
# 1. Ensure opencode is available in your PATH

# 2. Run the strict evaluation suite
python scripts/benchmark_rag.py \
    --raw-dir /path/to/raw/fastapi/docs \
    --index-dir /path/to/centralized-docs/fastapi_indexed
```
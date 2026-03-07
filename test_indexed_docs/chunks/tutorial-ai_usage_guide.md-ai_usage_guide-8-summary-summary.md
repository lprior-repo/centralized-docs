---
doc_id: tutorial/ai_usage_guide.md/ai_usage_guide
chunk_id: tutorial/ai_usage_guide.md/ai_usage_guide#8-summary
chunk_level: summary
chunk_type: prose
heading: Token Efficiency Benchmarks
token_count: 136
summary: ## Token Efficiency Benchmarks. By forcing agents to start at `llms
---

## Token Efficiency Benchmarks

By forcing agents to start at `llms.txt` and traverse the DAG via the `search` CLI, you achieve massive token efficiency.

In our benchmarks of major open-source repositories:
- **FastAPI:** 1,085,263 raw words ➔ 333 words in `llms.txt` (**99.9% reduction**)
- **Kubernetes:** 1,009,290 raw words ➔ 261 words in `llms.txt` (**99.9% reduction**)
- **Tokio:** 28,748 raw words ➔ 223 words in `llms.txt` (**99.2% reduction**)

Your agent uses almost zero tokens until it finds the *exact* 500-token contextual chunk it needs to solve the user's problem.

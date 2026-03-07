---
doc_id: tutorial/ai_usage_guide.md/ai_usage_guide
chunk_id: tutorial/ai_usage_guide.md/ai_usage_guide#4-detailed
chunk_level: detailed
chunk_type: prose
heading: Token Efficiency Benchmarks
token_count: 334
summary: # Integrating AI Agents with Centralized Docs. #### Tool 2: `read_chunk(chunk_id)`
---

# Integrating AI Agents with Centralized Docs














```bash
```


#### Tool 2: `read_chunk(chunk_id)`

Every chunk in the `INDEX.json` DAG contains its `heading_path`. For example:
`["Security - First Steps", "OAuth2 with Password", "Implementation"]`.

This completely solves the "lost in the middle" problem. The agent can read a 500-token chunk and know *exactly* where it sits in the hierarchy of the broader documentation.

## Advanced Usage: Traversing the Knowledge DAG

The `INDEX.json` contains a Directed Acyclic Graph (DAG) of the documentation. Every chunk knows its:
- `parent_chunk_id`
- `child_chunk_ids`
- `sibling_chunk_ids`
- `next_chunk_id`
- `previous_chunk_id`

If an agent reads a Summary chunk and needs more detail, it doesn't need to do another fuzzy search. It simply looks at the `child_chunk_ids` array in the JSON and fetches the specific detailed chunk. It can navigate the documentation programmatically just like a human clicking "Next Page".

## Token Efficiency Benchmarks

By forcing agents to start at `llms.txt` and traverse the DAG via the `search` CLI, you achieve massive token efficiency.

In our benchmarks of major open-source repositories:
- **FastAPI:** 1,085,263 raw words ➔ 333 words in `llms.txt` (**99.9% reduction**)
- **Kubernetes:** 1,009,290 raw words ➔ 261 words in `llms.txt` (**99.9% reduction**)
- **Tokio:** 28,748 raw words ➔ 223 words in `llms.txt` (**99.2% reduction**)

Your agent uses almost zero tokens until it finds the *exact* 500-token contextual chunk it needs to solve the user's problem.

---
doc_id: tutorial/ai_usage_guide.md/ai_usage_guide
chunk_id: tutorial/ai_usage_guide.md/ai_usage_guide#4-summary
chunk_level: summary
chunk_type: prose
heading: The Solution: The `llms.txt` Entry Point
token_count: 148
summary: - A curated list of the most important entry-point documents (categorized by Tutorials, Concepts, API Reference). - The exact path to the `INDEX
---

- A curated list of the most important entry-point documents (categorized by Tutorials, Concepts, API Reference).
- The exact path to the `INDEX.json` file.

### Step 2: Agent Tooling (Retrieval)

To make your agent effective, it needs access to two tools (functions) that interact with the `doc_transformer` output.

#### Tool 1: `search_docs(query)`
Instead of blindly reading files, the agent should be programmed to use the `doc_transformer search` CLI command.

```bash
doc_transformer search "how to configure oauth" --index-dir ./output --limit 3 --json
```

Because `doc_transformer` uses BM25 semantic indexing on the full body text, this will instantly return the 3 most relevant documents in a structured JSON payload.

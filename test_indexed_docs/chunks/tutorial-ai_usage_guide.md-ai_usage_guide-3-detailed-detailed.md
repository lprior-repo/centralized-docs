---
doc_id: tutorial/ai_usage_guide.md/ai_usage_guide
chunk_id: tutorial/ai_usage_guide.md/ai_usage_guide#3-detailed
chunk_level: detailed
chunk_type: prose
heading: Advanced Usage: Traversing the Knowledge DAG
token_count: 315
summary: # Integrating AI Agents with Centralized Docs. ### Step 2: Agent Tooling (Retrieval)
---

# Integrating AI Agents with Centralized Docs











### Step 2: Agent Tooling (Retrieval)


#### Tool 1: `search_docs(query)`
Instead of blindly reading files, the agent should be programmed to use the `doc_transformer search` CLI command.

```bash
doc_transformer search "how to configure oauth" --index-dir ./output --limit 3 --json
```

Because `doc_transformer` uses BM25 semantic indexing on the full body text, this will instantly return the 3 most relevant documents in a structured JSON payload.

#### Tool 2: `read_chunk(chunk_id)`
When the agent finds a relevant document via search, it can read the specific `chunk` instead of the whole file. 

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


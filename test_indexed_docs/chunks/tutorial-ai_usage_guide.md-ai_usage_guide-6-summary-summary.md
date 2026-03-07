---
doc_id: tutorial/ai_usage_guide.md/ai_usage_guide
chunk_id: tutorial/ai_usage_guide.md/ai_usage_guide#6-summary
chunk_level: summary
chunk_type: prose
heading: Advanced Usage: Traversing the Knowledge DAG
token_count: 137
summary: #### Tool 2: `read_chunk(chunk_id)`. `[\"Security - First Steps\", \"OAuth2 with Password\", \"Implementation\"]`
---


#### Tool 2: `read_chunk(chunk_id)`

`["Security - First Steps", "OAuth2 with Password", "Implementation"]`.


## Advanced Usage: Traversing the Knowledge DAG

The `INDEX.json` contains a Directed Acyclic Graph (DAG) of the documentation. Every chunk knows its:
- `parent_chunk_id`
- `child_chunk_ids`
- `sibling_chunk_ids`
- `next_chunk_id`
- `previous_chunk_id`

If an agent reads a Summary chunk and needs more detail, it doesn't need to do another fuzzy search. It simply looks at the `child_chunk_ids` array in the JSON and fetches the specific detailed chunk. It can navigate the documentation programmatically just like a human clicking "Next Page".

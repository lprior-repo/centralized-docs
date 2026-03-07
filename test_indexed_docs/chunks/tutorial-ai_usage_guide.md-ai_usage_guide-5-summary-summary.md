---
doc_id: tutorial/ai_usage_guide.md/ai_usage_guide
chunk_id: tutorial/ai_usage_guide.md/ai_usage_guide#5-summary
chunk_level: summary
chunk_type: prose
heading: The Solution: The `llms.txt` Entry Point
token_count: 104
summary: #### Tool 2: `read_chunk(chunk_id)`. When the agent finds a relevant document via search, it can read the specific `chunk` instead of the whole file
---


#### Tool 2: `read_chunk(chunk_id)`
When the agent finds a relevant document via search, it can read the specific `chunk` instead of the whole file. 

Every chunk in the `INDEX.json` DAG contains its `heading_path`. For example:
`["Security - First Steps", "OAuth2 with Password", "Implementation"]`.

This completely solves the "lost in the middle" problem. The agent can read a 500-token chunk and know *exactly* where it sits in the hierarchy of the broader documentation.


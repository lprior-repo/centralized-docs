---
doc_id: tutorial/ai_usage_guide.md/ai_usage_guide
chunk_id: tutorial/ai_usage_guide.md/ai_usage_guide#3-summary
chunk_level: summary
chunk_type: prose
heading: The Solution: The `llms.txt` Entry Point
token_count: 129
summary: ## The Solution: The `llms. txt` Entry Point
---

## The Solution: The `llms.txt` Entry Point

When you run `doc_transformer index` or `ingest-git`, the CLI generates an `llms.txt` file at the root of the output directory.

**This is the only file your agent needs to read initially.**

### Step 1: The Initial Prompt

Give your AI agent a system prompt similar to this:
> "You are an expert developer. To understand how to use the `[Library Name]` library, read the `llms.txt` file located at `[Path to Output]`. Do not read any other files until you have read the index."

The `llms.txt` file contains less than 300 words. It provides:

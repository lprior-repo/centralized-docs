---
doc_id: tutorial/ai_usage_guide.md/ai_usage_guide
chunk_id: tutorial/ai_usage_guide.md/ai_usage_guide#2-detailed
chunk_level: detailed
chunk_type: prose
heading: The Solution: The `llms.txt` Entry Point
token_count: 563
summary: # Integrating AI Agents with Centralized Docs. The primary goal of `centralized-docs` (`doc_transformer`) is to dramatically reduce the token footprint required to give AI agents (like Claude, GPT-...
---

# Integrating AI Agents with Centralized Docs

The primary goal of `centralized-docs` (`doc_transformer`) is to dramatically reduce the token footprint required to give AI agents (like Claude, GPT-4, or autonomous developer agents) deep contextual understanding of a codebase.

Instead of pasting entire documentation sites into a prompt, you integrate the agent with the generated structures.

## The Problem: The "Lost in the Middle" Effect

When AI agents read thousands of lines of raw Markdown, they suffer from two major issues:
1. **Token Exhaustion:** A large library like Kubernetes or FastAPI has over 1 million words of documentation. You simply cannot fit that into an agent's context window.
2. **Context Loss:** If you break a large markdown file into chunks blindly, a chunk that says "Run `make install`" loses the context of what header it was under. Was it under "Linux Installation" or "Windows Installation"? The agent hallucinates because it lost the semantic context.

## The Solution: The `llms.txt` Entry Point

When you run `doc_transformer index` or `ingest-git`, the CLI generates an `llms.txt` file at the root of the output directory.

**This is the only file your agent needs to read initially.**

### Step 1: The Initial Prompt

Give your AI agent a system prompt similar to this:
> "You are an expert developer. To understand how to use the `[Library Name]` library, read the `llms.txt` file located at `[Path to Output]`. Do not read any other files until you have read the index."

The `llms.txt` file contains less than 300 words. It provides:
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

#### Tool 2: `read_chunk(chunk_id)`
When the agent finds a relevant document via search, it can read the specific `chunk` instead of the whole file. 

Every chunk in the `INDEX.json` DAG contains its `heading_path`. For example:
`["Security - First Steps", "OAuth2 with Password", "Implementation"]`.

This completely solves the "lost in the middle" problem. The agent can read a 500-token chunk and know *exactly* where it sits in the hierarchy of the broader documentation.


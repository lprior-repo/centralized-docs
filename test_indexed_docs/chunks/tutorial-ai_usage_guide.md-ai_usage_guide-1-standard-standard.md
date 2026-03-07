---
doc_id: tutorial/ai_usage_guide.md/ai_usage_guide
chunk_id: tutorial/ai_usage_guide.md/ai_usage_guide#1-standard
chunk_level: standard
chunk_type: prose
heading: The Problem: The "Lost in the Middle" Effect
token_count: 183
summary: # Integrating AI Agents with Centralized Docs. The primary goal of `centralized-docs` (`doc_transformer`) is to dramatically reduce the token footprint required to give AI agents (like Claude, GPT-...
---

# Integrating AI Agents with Centralized Docs

The primary goal of `centralized-docs` (`doc_transformer`) is to dramatically reduce the token footprint required to give AI agents (like Claude, GPT-4, or autonomous developer agents) deep contextual understanding of a codebase.

Instead of pasting entire documentation sites into a prompt, you integrate the agent with the generated structures.

## The Problem: The "Lost in the Middle" Effect

When AI agents read thousands of lines of raw Markdown, they suffer from two major issues:
1. **Token Exhaustion:** A large library like Kubernetes or FastAPI has over 1 million words of documentation. You simply cannot fit that into an agent's context window.
2. **Context Loss:** If you break a large markdown file into chunks blindly, a chunk that says "Run `make install`" loses the context of what header it was under. Was it under "Linux Installation" or "Windows Installation"? The agent hallucinates because it lost the semantic context.


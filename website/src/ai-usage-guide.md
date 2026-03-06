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

---

## ⚡ The Copy-Paste Agent Instructions (50 Tokens)

Copy and paste this snippet directly into your `AGENTS.md`, `.clinerules`, or system prompt to instruct your agent on how to use Centralized Docs effectively:

```markdown
# Documentation Retrieval
This project uses `doc_transformer` for documentation. 
1. START by reading `llms.txt` in the docs output directory. Do NOT read raw markdown files.
2. SEARCH for concepts using: `doc_transformer search "query" -d <output-dir> --limit 3 --json`
3. READ specific chunks by using `jq` to extract the `content` field from the JSON output of your search, or by navigating the `INDEX.json` DAG.
```

---

## Step-by-Step: How Agents Use the CLI

To make your agent effective, it needs access to a terminal where it can execute `doc_transformer` commands natively. Here is the exact workflow an autonomous agent should follow to keep its token usage minimal.

### 1. The Initial Search
Instead of blindly reading files, the agent runs the `search` CLI command.

```bash
doc_transformer search "how to configure oauth" --index-dir ./output --limit 3 --json
```

Because `doc_transformer` uses BM25 semantic indexing on the full body text, this instantly returns the 3 most relevant documents in a structured JSON payload.

### 2. Reading the Chunk Context
When the agent receives the JSON payload, it can read the specific `content` instead of the whole file. 

Every chunk in the `INDEX.json` DAG contains its `heading_path`. For example:
`["Security - First Steps", "OAuth2 with Password", "Implementation"]`.

This completely solves the "lost in the middle" problem. The agent reads a 200-token chunk and knows *exactly* where it sits in the hierarchy of the broader documentation.

### 3. Advanced Usage: Traversing the Knowledge DAG
The `INDEX.json` contains a Directed Acyclic Graph (DAG) of the documentation. Every chunk knows its:
- `parent_chunk_id`
- `child_chunk_ids`
- `sibling_chunk_ids`

If an agent reads a Summary chunk and needs more detail, it doesn't need to do another fuzzy search. It simply looks at the `child_chunk_ids` array in the JSON and fetches the specific detailed chunk from the index. It navigates the documentation programmatically just like a human clicking "Next Page".

## Token Efficiency Benchmarks

By forcing agents to start at `llms.txt` and traverse the DAG via the `search` CLI, you achieve massive token efficiency.

In our benchmarks of major open-source repositories:
- **FastAPI:** 1,085,263 raw words ➔ 333 words in `llms.txt` (**99.9% reduction**)
- **Kubernetes:** 1,009,290 raw words ➔ 261 words in `llms.txt` (**99.9% reduction**)
- **Tokio:** 28,748 raw words ➔ 223 words in `llms.txt` (**99.2% reduction**)

Your agent uses almost zero tokens until it finds the *exact* 500-token contextual chunk it needs to solve the user's problem.

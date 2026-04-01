# Integrating AI Agents with Centralized Docs

The primary goal of `centralized-docs` (`ctd`) is to dramatically reduce the token footprint required to give AI agents (like Claude, GPT-4, or autonomous developer agents) deep contextual understanding of a codebase.

Instead of pasting entire documentation sites into a prompt, you integrate the agent with the generated structures.

## MCP Server Integration

The MCP server provides native tool access for AI agents through the [Model Context Protocol](https://modelcontextprotocol.io).

### Benefits Over CLI

| Aspect | CLI | MCP Server |
|--------|-----|------------|
| Integration | Subprocess calls | Native tools |
| Context | Lost between calls | Preserved |
| Chunk IDs | Manual JSON parsing | Direct references |
| Related concepts | Multiple queries | Graph traversal |

### Claude Code Usage

```bash
# Add the MCP server
claude mcp add centralized-docs ctd mcp serve /path/to/indexed-docs

# Now Claude Code can use tools directly
# The AI can call: search_docs, read_chunk, get_related_concepts
```

### Claude Desktop Configuration

```json
{
  "mcpServers": {
    "centralized-docs": {
      "command": "ctd",
      "args": ["mcp", "serve", "/path/to/indexed-docs"]
    }
  }
}
```

### Available Tools

- **`search_docs(query, limit?)`** - Search using BM25
- **`read_chunk(id)`** - Read a specific chunk by ID
- **`get_related_concepts(id)`** - Get related concepts from the graph

See [MCP Server](mcp-server.md) for complete documentation.

---

## The Problem: The "Lost in the Middle" Effect

When AI agents read thousands of lines of raw Markdown, they suffer from two major issues:
1. **Token Exhaustion:** A large library like Kubernetes or FastAPI has over 1 million words of documentation. You simply cannot fit that into an agent's context window.
2. **Context Loss:** If you break a large markdown file into chunks blindly, a chunk that says "Run `make install`" loses the context of what header it was under. Was it under "Linux Installation" or "Windows Installation"? The agent hallucinates because it lost the semantic context.

## The Solution: The `llms.txt` Entry Point

When you run `ctd index` or `ingest-git`, the CLI generates an `llms.txt` file at the root of the output directory.

**This is the only file your agent needs to read initially.**

---

## ⚡ The Copy-Paste Agent Instructions (50 Tokens)

Copy and paste this snippet directly into your `AGENTS.md`, `CLAUDE.md`, or system prompt to instruct your agent on how to use Centralized Docs effectively:

```markdown
# Documentation Retrieval
This project uses `ctd` for documentation. 
1. START by reading `llms.txt` in the docs output directory. Do NOT read raw markdown files.
2. SEARCH for concepts using: `ctd search "query" --index-dir <output-dir> --limit 3 --json`
3. READ specific chunks by using `jq` to extract the `content` field from the JSON output of your search, or by navigating the `INDEX.json` DAG.
```

---

## Step-by-Step: How Agents Use the CLI

To make your agent effective, it needs access to a terminal where it can execute `ctd` commands natively. Here is the exact workflow an autonomous agent should follow to keep its token usage minimal.

### 1. The Initial Search
Instead of blindly reading files, the agent runs the `search` CLI command.

```bash
ctd search "how to configure oauth" --index-dir ./output --limit 3 --json
```

Because `ctd` uses BM25 semantic indexing on the full body text, this instantly returns the 3 most relevant documents in a structured JSON payload.

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
- `next_chunk_id`
- `previous_chunk_id`

If an agent reads a Summary chunk and needs more detail, it doesn't need to do another fuzzy search. It simply looks at the `child_chunk_ids` array in the JSON and fetches the specific detailed chunk. It can navigate the documentation programmatically just like a human clicking "Next Page".

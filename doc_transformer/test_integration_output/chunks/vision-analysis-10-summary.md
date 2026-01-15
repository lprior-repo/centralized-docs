---
doc_id: vision-analysis
chunk_id: vision-analysis#10
chunk_level: summary
chunk_type: prose
heading: 🔍 Gap Analysis: PLAN.md vs Vision
token_count: 135
summary:  **MCP Server** (CRITICAL GAP). **Why Critical:** This is the \"missing piece\" that enables AI to q
---


#### 1. **MCP Server** (CRITICAL GAP)
**Why Critical:** This is the "missing piece" that enables AI to query indexed docs
**What's needed:**
- Rust MCP SDK integration (rust-mcp-sdk = "0.8")
- Tools: search_docs(), get_chunk(), find_related()
- Expose INDEX.json and GRAPH.json
- Enable AI agents to query without re-scraping

**Impact:** Without MCP server, the vision is incomplete

#### 2. **Contextual-Chunker as Standalone Crate**
**Why Important:** Makes innovation reusable
**What's needed:**
- Extract chunk.rs into separate crate

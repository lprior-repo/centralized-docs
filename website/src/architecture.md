# How It Works (Architecture)

The `ctd` CLI is designed to take unstructured or semi-structured documentation and turn it into highly optimized data structures for AI agents. 

Here is the high-level flow of how the architecture processes data.

## 1. Scraping & Ingestion
The process starts by gathering documentation. You can point the CLI at a local directory of markdown files, a Git repository, or a live static documentation website. 

When scraping static sites, the CLI crawls the URLs, extracts the HTML, strips out the noise (like navbars and footers), and converts the core content into clean, readable Markdown.

## 2. Contextual Chunking
Large markdown files are problematic for AI because they either blow up the context window or, if chunked poorly, lose the thread of what they are talking about.

The CLI uses **Contextual Chunking**:
- It breaks documents down based on Markdown headers (`#`, `##`, `###`).
- Crucially, it attaches the hierarchy of the headers to each chunk. 
- If a chunk is under `## Installation`, the AI knows it's an installation step, avoiding the "lost in the middle" problem.

## 3. Knowledge Graph (DAG) Construction
Instead of treating documents as an isolated pile of files, the CLI detects relationships between them. 

It builds a **Directed Acyclic Graph (DAG)** of the content. By analyzing links and shared concepts, it maps out how different parts of the documentation relate to each other. This allows AI agents to traverse the documentation logically—moving from a broad concept down to specific API details—just like a human navigating a doc site.

## 4. Semantic Indexing (BM25)
To support fast and effective search, the CLI builds a full-text search index using the **BM25 algorithm**. 

When an AI agent needs to know "how to configure caching", it doesn't need to read the whole library. It can use the `search` CLI command to query the BM25 index, returning only the most relevant, highly-scored contextual chunks.

## 5. llms.txt Entry Point
The final output is tied together with an `llms.txt` file at the root of the indexed output. 

Think of `llms.txt` as a `robots.txt` for AI. It gives the agent a top-level summary of the project and provides direct, curated links to the most important sections, the index, and the DAG. Instead of hallucinating paths or guessing where to start, the agent reads the `llms.txt` and knows exactly how to interact with the library's documentation.

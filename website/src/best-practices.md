# Best Practices

To get the most out of `ctd` and its generated structures today, follow these best practices for indexing and utilizing the CLI.

## 1. Local Indexing First
Always test your indexing locally before running it against large repositories or scraping remote websites. 
```bash
ctd index ./my-local-docs -o ./output --llms-txt
```
This lets you verify that your documents are chunked and categorized effectively.

## 2. Optimize Chunk Parameters
Depending on the size of your AI context windows, you may want to adjust chunking properties. 
- Use `--max-related-chunks` to limit how many connections the knowledge graph builds.
- Keep `--max-chunk-keywords` focused. Over-populating keywords can lead to BM25 search noise. 

## 3. Leverage Contextual Chunking
Anthropic's contextual retrieval works best when documents have clear, hierarchical headers. Make sure your raw markdown files use `H1`, `H2`, and `H3` logically so that the chunker can accurately preserve the context of the sections it splits.

## 4. `llms.txt` Curation
When generating `llms.txt` files for AI agents:
1. **Short Descriptions:** Provide a concise `--project-desc` during the `index` step so agents know exactly what your project is.
2. **Logical Output Directories:** Group your markdown output into sensible directories (`Getting Started`, `Core Concepts`, `API Reference`) so the generated `llms.txt` maps these cleanly.
3. **Progressive Complexity:** Structure your source directories from simple to complex order.

## 5. Using the BM25 Search
Instead of blindly feeding a giant project into an LLM context, use the `search` command to extract only the top hits:
```bash
ctd search "how to configure caching" --index-dir ./output --limit 5 --json
```
Use the `--json` output so your AI agent or application can parse the resulting document references programmatically without hallucinating paths.

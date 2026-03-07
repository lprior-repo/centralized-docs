---
doc_id: concept/cli_reference.md/cli_reference
chunk_id: concept/cli_reference.md/cli_reference#4-summary
chunk_level: summary
chunk_type: prose
heading: `doc_transformer`
token_count: 146
summary: #### `index`. Index local markdown files into an AI-optimized structure
---


#### `index`
Index local markdown files into an AI-optimized structure.
- `SOURCE`: Source directory containing markdown files.
- `--output`, `-o <DIR>`: Output directory for indexed content.
- `--llms-txt`: Generate `llms.txt` entry point files (default: true).
- `--project-name <NAME>`: Project name for `llms.txt` header (default: "Documentation").
- `--project-desc <DESC>`: Project description for `llms.txt` (default: "AI-optimized documentation index").
- `--category-config <FILE>`: Path to category rules config file.
- `--max-related-chunks <N>`: Maximum number of related chunks per document (1-100, default: 20).
- `--max-chunk-keywords <N>`: Maximum number of chunk keywords to include in similarity (0-50, default: 12).

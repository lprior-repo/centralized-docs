# AI-Optimized Documentation Indexer - Implementation Plan

## Overview

Transform `doc_transformer` from a local-markdown-only tool into the **best documentation indexer in the world** for AI agents. This plan adds web scraping via spider-rs, content filtering, and llms.txt generation.

## Architecture

```
┌─────────────────────────────────────────────────────────────────────────┐
│                         doc_transformer v5.0                            │
├─────────────────────────────────────────────────────────────────────────┤
│                                                                         │
│  SCRAPE (NEW)          PROCESS (EXISTING)          OUTPUT (ENHANCED)   │
│  ┌─────────────┐       ┌─────────────────┐         ┌────────────────┐  │
│  │ spider-rs   │       │ discover        │         │ INDEX.json     │  │
│  │ Website     │──────▶│ analyze         │────────▶│ COMPASS.md     │  │
│  │ .scrape()   │       │ assign          │         │ llms.txt (NEW) │  │
│  └─────────────┘       │ transform       │         │ llms-full.txt  │  │
│        │               │ chunk           │         │ docs/          │  │
│        ▼               │ graph           │         │ chunks/        │  │
│  ┌─────────────┐       │ index           │         └────────────────┘  │
│  │ spider_     │       │ validate        │                             │
│  │ transform-  │       └─────────────────┘                             │
│  │ ations      │              ▲                                        │
│  │ (markdown)  │              │                                        │
│  └─────────────┘              │                                        │
│        │               ┌──────┴──────┐                                 │
│        ▼               │ filter.rs   │                                 │
│  ┌─────────────┐       │ (NEW)       │                                 │
│  │ Content     │──────▶│ - pruning   │                                 │
│  │ Filtering   │       │ - density   │                                 │
│  │ (BM25/prune)│       │ - BM25      │                                 │
│  └─────────────┘       └─────────────┘                                 │
│                                                                         │
└─────────────────────────────────────────────────────────────────────────┘
```

## CLI Design (For AI Agents)

```bash
# Scrape a documentation site to local markdown
doc_transformer scrape https://docs.example.com \
  --output ./scraped \
  --sitemap \                    # Use sitemap.xml (default: true)
  --filter "^/docs/" \           # URL path regex filter
  --delay 250                    # ms between requests

# Process local markdown into AI-optimized index
doc_transformer index ./source \
  --output ./indexed \
  --generate-llms-txt            # Generate llms.txt files

# One-shot: scrape + index
doc_transformer ingest https://docs.example.com \
  --output ./indexed

# Legacy mode (backwards compatible)
doc_transformer ./source ./output
```

### Exit Codes (Important for AI)
- `0` - Success
- `1` - Partial success (some pages failed)
- `2` - Complete failure

### Output Format
- Progress → stderr
- Final JSON summary → stdout (with `--json`)

## New Modules

### 1. `scrape.rs` - Web Scraping with spider-rs

```rust
// Key structures
pub struct ScrapeConfig {
    pub base_url: String,
    pub use_sitemap: bool,
    pub path_filter: Option<String>,  // Regex
    pub delay_ms: u64,
    pub user_agent: String,
    pub respect_robots: bool,
}

pub struct ScrapedPage {
    pub url: String,
    pub markdown: String,       // Via spider_transformations
    pub title: String,
    pub links: Vec<String>,     // Internal links
    pub headers: Vec<Header>,   // Extracted H1-H6
    pub word_count: usize,
}

pub struct ScrapeResult {
    pub pages: Vec<ScrapedPage>,
    pub total_urls: usize,
    pub success_count: usize,
    pub error_count: usize,
    pub errors: Vec<(String, String)>,  // (url, error)
}

// Main function - sequential processing (no complex concurrency)
pub fn scrape_site(config: &ScrapeConfig) -> Result<ScrapeResult>
```

**Implementation approach:**
- Use `spider::Website::new(url)` with `scrape()`
- Sequential page processing via `website.get_pages()`
- Transform HTML → Markdown via `spider_transformations::transform_content()`
- Extract metadata from markdown (headers, links)
- Write each page to `{output}/.scrape/{url_slug}.md`

### 2. `filter.rs` - Content Filtering

```rust
pub struct FilterConfig {
    pub strategy: FilterStrategy,
    pub threshold: f32,
    pub min_word_count: usize,
}

pub enum FilterStrategy {
    Pruning,    // Text density + link density + tag importance
    BM25,       // Query-based relevance (for targeted scraping)
    None,       // Pass-through
}

pub struct FilteredContent {
    pub content: String,
    pub removed_sections: Vec<String>,  // What was pruned
    pub density_score: f32,
}

// Pruning filter implementation
pub fn prune_content(html: &str, threshold: f32) -> FilteredContent

// BM25 filter for query-based extraction
pub fn bm25_filter(html: &str, query: &str, threshold: f32) -> FilteredContent
```

**Pruning algorithm:**
1. Parse HTML into DOM tree
2. Score each node: `score = text_density * 0.6 + (1 - link_density) * 0.3 + tag_weight * 0.1`
3. Remove nodes below threshold (default: 0.45)
4. Return cleaned HTML for markdown conversion

### 3. `llms.rs` - llms.txt Generation

```rust
pub fn generate_llms_txt(
    analyses: &[Analysis],
    link_map: &HashMap<String, IdMapping>,
    project_name: &str,
    project_description: &str,
    output_dir: &Path,
) -> Result<()>

pub fn generate_llms_full_txt(
    analyses: &[Analysis],
    link_map: &HashMap<String, IdMapping>,
    output_dir: &Path,
) -> Result<()>
```

**llms.txt format:**
```markdown
# Project Name

> Brief description of what this project does.

Key context for AI:
- Technology: Rust-based documentation indexer
- Purpose: Transform docs into AI-optimized indexes

## Getting Started
- [Quickstart](./docs/tutorial-quickstart.md): 5-minute setup guide
- [Installation](./docs/tutorial-install.md): Full installation

## Core Concepts
- [Architecture](./docs/concept-architecture.md): System overview
- [Chunking](./docs/concept-chunking.md): How content is split

## API Reference
- [CLI](./docs/ref-cli.md): Command-line interface
- [Config](./docs/ref-config.md): Configuration options

## Optional
- [Changelog](./docs/meta-changelog.md)
- [Contributing](./docs/meta-contributing.md)
```

## Dependencies to Add

```toml
# Cargo.toml additions
[dependencies]
# Web scraping
spider = "2"
spider_transformations = "2"

# URL handling
url = "2.5"

# HTML parsing for content filtering
scraper = "0.20"
```

## File Changes

| File | Change |
|------|--------|
| `Cargo.toml` | Add spider, spider_transformations, url, scraper |
| `src/main.rs` | Add subcommands (scrape, index, ingest), keep legacy mode |
| `src/scrape.rs` | NEW - spider-rs integration |
| `src/filter.rs` | NEW - content filtering (pruning, BM25) |
| `src/llms.rs` | NEW - llms.txt generation |
| `src/index.rs` | Call llms.rs functions |

## Implementation Order

1. **Add dependencies** to Cargo.toml
2. **Create scrape.rs** - basic spider-rs integration
3. **Create filter.rs** - pruning filter only (BM25 later)
4. **Create llms.rs** - llms.txt generation
5. **Update index.rs** - integrate llms.txt generation
6. **Update main.rs** - add subcommands
7. **Test with real docs site**

## Output Structure (AI-Optimized)

```
output/
├── llms.txt                 # AI READS THIS FIRST
│                            # Curated overview, fits context window
│
├── llms-full.txt            # Full content concatenated
│                            # For large context models
│
├── INDEX.json               # Machine-readable everything
│   ├── documents[]          # Doc metadata + chunk mappings
│   ├── chunks[]             # Chunk metadata + navigation
│   ├── keywords{}           # Term → doc_ids lookup
│   └── graph{}              # Knowledge DAG (nodes + edges)
│
├── COMPASS.md               # Human-readable navigation
│
├── docs/                    # Transformed documents
│   └── {category}-{slug}.md # With YAML frontmatter
│
├── chunks/                  # Semantic chunks
│   └── {doc-id}-{n}.md      # With context prefix
│
└── .scrape/                 # Raw scraped content (if from web)
    └── {url-hash}.md        # For debugging/reprocessing
```

## Why spider-rs Over Alternatives

1. **All-in-one**: Crawling + transformation in one ecosystem
2. **spider_transformations**: Built specifically for LLM-ready output
3. **Production-tested**: Used by Spider Cloud at scale
4. **Rust-native**: No FFI overhead, compiles to single binary
5. **Feature flags**: Enable only what we need

## Minimal Concurrency Approach

Spider-rs handles concurrency internally. We interface with it sequentially:

```rust
// This is sequential from our perspective
let mut website = Website::new(&config.base_url);
website.configuration.delay = config.delay_ms;
website.scrape().await;  // spider handles concurrency internally

// Process results sequentially
for page in website.get_pages().unwrap().iter() {
    let markdown = transform_to_markdown(page);
    save_page(markdown);
}
```

No need to write concurrent Rust ourselves - spider handles it.

## Content Filtering Strategy

Use **pruning** by default (no LLM needed):

1. **Text density**: `text_length / (text_length + html_tag_length)`
2. **Link density**: `link_text_length / text_length`
3. **Tag importance**: Higher score for `article`, `main`, `section`; lower for `nav`, `footer`, `aside`

This removes:
- Navigation menus
- Footers
- Sidebars
- Ads
- Boilerplate

Keeps:
- Main content
- Code blocks
- Tables
- Documentation prose

## Testing Strategy

1. **Unit tests**: Each module in isolation
2. **Integration test**: Scrape → Index full pipeline
3. **Real site test**: Index actual docs site (e.g., docs.rs/spider)

## Version

This plan targets **doc_transformer v5.0** (Web Scraping + llms.txt)

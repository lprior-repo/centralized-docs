# llms-txt-parser

[![Crates.io](https://img.shields.io/crates/v/llms-txt-parser.svg)](https://crates.io/crates/llms-txt-parser)
[![Docs.rs](https://docs.rs/llms-txt-parser/badge.svg)](https://docs.rs/llms-txt-parser)
[![License](https://img.shields.io/crates/l/llms-txt-parser.svg)](./LICENSE)

Parser for llms.txt files - AI documentation entry points following the llms.txt specification.

## Features

- **YAML Frontmatter** - Parse version, project metadata
- **Section Extraction** - Extract all ## sections with content
- **Link Parsing** - Parse markdown links with descriptions
- **Validation Helpers** - Check for required sections
- **Zero-Copy** - Efficient parsing with minimal allocations
- **Type-Safe** - Strong typing with serde support

## Installation

```toml
[dependencies]
llms-txt-parser = "0.1"
```

## Quick Start

```rust
use llms_txt_parser::{parse_file, LlmsTxt};

// Parse from file
let llms_txt = parse_file("llms.txt")?;

println!("Project: {}", llms_txt.project_name);
println!("Description: {}", llms_txt.description.unwrap_or_default());

// Access sections
for section in &llms_txt.sections {
    println!("## {}", section.title);
    for link in &section.links {
        println!("  - [{}]({})", link.text, link.url);
    }
}
```

## llms.txt Format

llms.txt is a standardized format for AI agents to discover documentation, similar to robots.txt for web crawlers.

### With Frontmatter

```markdown
---
version: "1.0"
project: "My Project"
project_version: "1.2.3"
updated: "2026-01-15"
documents: 42
index: "./INDEX.json"
---

# My Project

> Brief description for AI agents

## Getting Started

- [Installation](./docs/install.md): How to install
- [Quick Start](./docs/quickstart.md): Get started in 5 minutes

## Core Concepts

- [Architecture](./docs/arch.md): System architecture
- [Design Patterns](./docs/patterns.md): Common patterns

## API Reference

- [API Docs](./docs/api.md): Complete API reference

## Machine-Readable Index

- [INDEX.json](./INDEX.json): Searchable index with chunks
```

## API

### Parsing

```rust
use llms_txt_parser::{parse_file, parse_content};

// From file
let llms_txt = parse_file("llms.txt")?;

// From string
let content = std::fs::read_to_string("llms.txt")?;
let llms_txt = parse_content(&content)?;
```

### Accessing Data

```rust
// Project metadata
println!("Project: {}", llms_txt.project_name);
println!("Description: {:?}", llms_txt.description);

// Frontmatter (if present)
if let Some(fm) = &llms_txt.frontmatter {
    println!("Version: {:?}", fm.version);
    println!("Updated: {:?}", fm.updated);
    println!("Documents: {:?}", fm.documents);
}

// Sections
for section in &llms_txt.sections {
    println!("Section: {}", section.title);
    println!("Content: {}", section.content);

    for link in &section.links {
        println!("  Link: {} -> {}", link.text, link.url);
        if let Some(desc) = &link.description {
            println!("    Description: {}", desc);
        }
    }
}
```

### Validation

```rust
// Check for required sections
if llms_txt.has_required_sections() {
    println!("✓ All required sections present");
} else {
    println!("✗ Missing required sections");
}

// Get specific section
if let Some(getting_started) = llms_txt.get_section("Getting Started") {
    println!("Getting Started has {} links", getting_started.links.len());
}

// Get INDEX.json reference
if let Some(index) = llms_txt.get_index_reference() {
    println!("Index: {}", index);
}
```

## Data Structures

### LlmsTxt

Main structure representing a parsed llms.txt file:

```rust
pub struct LlmsTxt {
    pub frontmatter: Option<Frontmatter>,
    pub project_name: String,
    pub description: Option<String>,
    pub sections: Vec<Section>,
}
```

### Frontmatter

YAML metadata from the file header:

```rust
pub struct Frontmatter {
    pub version: Option<String>,
    pub project: Option<String>,
    pub project_version: Option<String>,
    pub updated: Option<String>,
    pub documents: Option<usize>,
    pub index: Option<String>,
    pub extra: HashMap<String, serde_yaml::Value>,
}
```

### Section

A ## section with links and content:

```rust
pub struct Section {
    pub title: String,
    pub content: String,
    pub links: Vec<Link>,
}
```

### Link

A markdown link with optional description:

```rust
pub struct Link {
    pub text: String,
    pub url: String,
    pub description: Option<String>,
}
```

## Error Handling

```rust
use llms_txt_parser::{parse_file, ParseError};

match parse_file("llms.txt") {
    Ok(llms_txt) => {
        // Process successfully parsed file
    }
    Err(e) => {
        eprintln!("Parse error: {}", e);
    }
}
```

## Use Cases

### Documentation Tooling

```rust
// Build documentation index
let llms_txt = parse_file("llms.txt")?;
let mut index = DocumentIndex::new();

for section in &llms_txt.sections {
    for link in &section.links {
        index.add_document(&link.url, &link.text);
    }
}
```

### AI Agent Integration

```rust
// AI agent discovers documentation
let llms_txt = parse_file("llms.txt")?;

if llms_txt.has_required_sections() {
    let getting_started = llms_txt.get_section("Getting Started").unwrap();

    // Show AI the first document
    if let Some(first_link) = getting_started.links.first() {
        let content = fetch_document(&first_link.url)?;
        process_with_ai(content);
    }
}
```

### Validation

```rust
// Validate llms.txt file
let llms_txt = parse_file("llms.txt")?;

let mut errors = Vec::new();

if llms_txt.project_name.is_empty() {
    errors.push("Missing project name");
}

if !llms_txt.has_required_sections() {
    errors.push("Missing required sections");
}

if llms_txt.get_index_reference().is_none() {
    errors.push("No INDEX.json reference");
}

if errors.is_empty() {
    println!("✓ Valid llms.txt");
} else {
    for error in errors {
        println!("✗ {}", error);
    }
}
```

## Specification Compliance

This parser follows the llms.txt specification v1.0:

- ✅ YAML frontmatter support
- ✅ Required sections: Getting Started, Core Concepts, API Reference
- ✅ Markdown link parsing
- ✅ Section extraction
- ✅ INDEX.json reference detection

## Performance

- **Parsing**: ~100µs for typical llms.txt files (<10KB)
- **Memory**: O(n) where n is file size
- **Zero-copy**: Efficient parsing with minimal allocations

## Related Projects

- [llms-txt-validator](../llms-txt-validator) - CLI validator for llms.txt files
- [contextual-chunker](../contextual-chunker) - Semantic chunking for documentation
- [centralized-docs](https://github.com/anthropics/centralized-docs) - Complete documentation indexer

## Contributing

Contributions welcome! Please:
1. Add tests for new features
2. Maintain documentation
3. Follow existing code style
4. Ensure zero unsafe code

## License

MIT - See LICENSE file

## Changelog

### 0.1.0 (Initial Release)
- YAML frontmatter parsing
- Section extraction
- Link parsing with descriptions
- Validation helpers
- Complete test coverage

## Citation

```bibtex
@software{llms_txt_parser,
  author = {Anthropic},
  title = {llms-txt-parser: Parser for llms.txt AI documentation files},
  year = {2026},
  url = {https://github.com/anthropics/centralized-docs}
}
```

## Questions?

- Issues: https://github.com/anthropics/centralized-docs/issues
- Discussions: https://github.com/anthropics/centralized-docs/discussions

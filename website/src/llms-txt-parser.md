# llms-txt-parser

Parser for `llms.txt` files - AI documentation entry points.

## Installation
```toml
[dependencies]
llms-txt-parser = "0.1"
```

## Usage
```rust
use llms_txt_parser::parse_file;

let llms_txt = parse_file("llms.txt")?;
println!("Project: {}", llms_txt.project_name);
```

## Features
- Parses YAML frontmatter.
- Extracts `##` sections and links.
- Fast, zero-copy, type-safe validation.

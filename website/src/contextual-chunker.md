# contextual-chunker

Semantic chunking for documentation and knowledge bases. Ideal for RAG systems.

## Installation
```toml
[dependencies]
contextual-chunker = "0.1"
```

## Usage
```rust
use contextual_chunker::{Document, chunk_all};

let docs = vec![Document::new("id".into(), "Title".into(), "Content".into())];
let result = chunk_all(&docs)?;
```

## Features
- **Semantic Boundaries:** Splits on H2 headings.
- **Hierarchical:** Summary (128 tokens), Standard (512), Detailed (1024).
- **Navigation:** Preserves relationships and context prefixes.

//! Basic example of using contextual-chunker
//!
//! Run with: cargo run --example `basic_chunking`

use contextual_chunker::{chunk, chunk_all, ChunkLevel, Document};

fn main() -> anyhow::Result<()> {
    // Example 1: Simple document with H2 sections
    println!("=== Example 1: Simple Document ===\n");

    let doc = Document::new(
        "tutorial".to_string(),
        "Getting Started Tutorial".to_string(),
        r"
## Installation

To get started, you need to install the required software:
- Rust 1.70+
- Cargo
- Git

Installation is straightforward on all major platforms.

## Configuration

After installation, create a config file:

```bash
mkdir -p ~/.config
touch ~/.config/app.toml
```

## Testing Your Setup

Run the tests to verify everything is working:

```bash
cargo test
```

If all tests pass, you're ready to go!
"
        .to_string(),
    );

    // Chunk at Standard level
    let standard_chunks = chunk(&doc, ChunkLevel::Standard)?;
    println!("Standard chunks: {}", standard_chunks.len());
    for (i, chunk) in standard_chunks.iter().enumerate() {
        println!(
            "  Chunk {}: {} ({} tokens)",
            i, chunk.chunk_id, chunk.token_count
        );
        println!("    Type: {}", chunk.chunk_type);
        println!("    Summary: {}", chunk.summary);
    }

    // Example 2: Multi-document hierarchical chunking
    println!("\n=== Example 2: Multiple Documents (Hierarchical) ===\n");

    let docs = vec![
        Document::new(
            "guide-intro".to_string(),
            "Introduction Guide".to_string(),
            "## What is This?\n\nThis is an introduction to the system.\n## Key Concepts\n\nLearn about the key concepts."
                .to_string(),
        ),
        Document::new(
            "guide-advanced".to_string(),
            "Advanced Guide".to_string(),
            "## Advanced Topics\n\nFor experienced users.\n## Best Practices\n\nHow to use effectively."
                .to_string(),
        ),
    ];

    let result = chunk_all(&docs)?;
    println!("Total chunks: {}", result.chunks.len());
    println!("  Summary level: {}", result.summary_count);
    println!("  Standard level: {}", result.standard_count);
    println!("  Detailed level: {}", result.detailed_count);

    // Show hierarchical relationships
    println!("\n=== Hierarchy Example ===\n");
    for chunk in &result.chunks {
        if chunk.chunk_level == ChunkLevel::Summary {
            println!("Summary: {} ({})", chunk.chunk_id, chunk.doc_title);
            for child_id in &chunk.child_chunk_ids {
                let child = result
                    .chunks
                    .iter()
                    .find(|c| &c.chunk_id == child_id)
                    .ok_or_else(|| anyhow::anyhow!("Child chunk {child_id} not found"))?;
                println!("  └─ Standard: {}", child.chunk_id);
                for grandchild_id in &child.child_chunk_ids {
                    let grandchild = result
                        .chunks
                        .iter()
                        .find(|c| &c.chunk_id == grandchild_id)
                        .ok_or_else(|| {
                            anyhow::anyhow!("Grandchild chunk {grandchild_id} not found")
                        })?;
                    println!("     └─ Detailed: {}", grandchild.chunk_id);
                }
            }
        }
    }

    // Example 3: Navigation
    println!("\n=== Navigation Example ===\n");
    if let Some(first_chunk) = result.chunks.first() {
        println!("First chunk: {}", first_chunk.chunk_id);
        if let Some(next_id) = &first_chunk.next_chunk_id {
            println!("Next chunk: {next_id}");
        }
    }

    Ok(())
}

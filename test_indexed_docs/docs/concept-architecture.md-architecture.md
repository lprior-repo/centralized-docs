---
id: concept/architecture.md/architecture
title: Architecture
category: concept
tags: ["architecture", "concept", "layers", "principles"]
---

# Architecture



 > 
 > **Context**: \**doc_transformer** uses a **Hexagonal Architecture** (Ports and Adapters) combined with functional Rust.



**doc_transformer** uses a **Hexagonal Architecture** (Ports and Adapters) combined with functional Rust.

## Core Principles

1. **Zero Panics**: Explicit `Result<T, E>` and `Option<T>` for error handling.
1. **Immutability**: Data structures are immutable; transformations return new values.
1. **Pure Functions**: Business logic has no side effects.
1. **Explicit Dependencies**: Inject dependencies, avoid hidden states.

## Layers

1. **Presentation Layer (`main.rs`)**: CLI interface, argument parsing (`clap`).
1. **Application Layer (Core)**: Orchestrates transformations (`transform.rs`, `chunk.rs`). Pure business logic.
1. **Ports Layer**: Implicit contracts defined via public function signatures and types.
1. **Adapters Layer**: File I/O (`discover.rs`), JSON serialization (`index.rs`), regex (`analyze.rs`).

## The Pipeline

1. **Discover**: Find markdown files.
1. **Analyze**: Extract metadata.
1. **Assign**: Generate IDs.
1. **Transform**: Apply standard formatting.
1. **Chunk**: Semantic splitting.
1. **Index**: Create `INDEX.json` and `COMPASS.md`.
1. **Validate**: Run quality checks.
## See Also

- [Documentation Index](./COMPASS.md)

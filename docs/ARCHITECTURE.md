# Architecture

**ctd** uses a **Hexagonal Architecture** (Ports and Adapters) combined with functional Rust.

## Core Principles

1. **Zero Panics**: Explicit `Result<T, E>` and `Option<T>` for error handling.
2. **Immutability**: Data structures are immutable; transformations return new values.
3. **Pure Functions**: Business logic has no side effects.
4. **Explicit Dependencies**: Inject dependencies, avoid hidden states.

## Layers

1. **Presentation Layer (`main.rs`)**: CLI interface, argument parsing (`clap`).
2. **Application Layer (Core)**: Orchestrates transformations (`transform.rs`, `chunk.rs`). Pure business logic.
3. **Ports Layer**: Implicit contracts defined via public function signatures and types.
4. **Adapters Layer**: File I/O (`discover.rs`), JSON serialization (`index.rs`), regex (`analyze.rs`).

## The Pipeline

1. **Discover**: Find markdown files.
2. **Analyze**: Extract metadata.
3. **Assign**: Generate IDs.
4. **Transform**: Apply standard formatting.
5. **Chunk**: Semantic splitting.
6. **Index**: Create `INDEX.json` and `COMPASS.md`.
7. **Validate**: Run quality checks.

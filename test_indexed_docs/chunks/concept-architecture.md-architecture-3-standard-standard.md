---
doc_id: concept/architecture.md/architecture
chunk_id: concept/architecture.md/architecture#3-standard
chunk_level: standard
chunk_type: prose
heading: The Pipeline
token_count: 168
summary:  **Pure Functions**: Business logic has no side effects.  **Explicit Dependencies**: Inject dependencies, avoid hidden states
---




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

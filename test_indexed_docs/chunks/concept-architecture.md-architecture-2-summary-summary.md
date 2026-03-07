---
doc_id: concept/architecture.md/architecture
chunk_id: concept/architecture.md/architecture#2-summary
chunk_level: summary
chunk_type: prose
heading: Layers
token_count: 104
summary: ## Core Principles.  **Pure Functions**: Business logic has no side effects
---



## Core Principles

3. **Pure Functions**: Business logic has no side effects.
4. **Explicit Dependencies**: Inject dependencies, avoid hidden states.

## Layers

1. **Presentation Layer (`main.rs`)**: CLI interface, argument parsing (`clap`).
2. **Application Layer (Core)**: Orchestrates transformations (`transform.rs`, `chunk.rs`). Pure business logic.
3. **Ports Layer**: Implicit contracts defined via public function signatures and types.
4. **Adapters Layer**: File I/O (`discover.rs`), JSON serialization (`index.rs`), regex (`analyze.rs`).


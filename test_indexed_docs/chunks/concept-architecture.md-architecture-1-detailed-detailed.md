---
doc_id: concept/architecture.md/architecture
chunk_id: concept/architecture.md/architecture#1-detailed
chunk_level: detailed
chunk_type: prose
heading: Core Principles
token_count: 85
summary: # Architecture. **doc_transformer** uses a **Hexagonal Architecture** (Ports and Adapters) combined with functional Rust
---

# Architecture

**doc_transformer** uses a **Hexagonal Architecture** (Ports and Adapters) combined with functional Rust.

## Core Principles

1. **Zero Panics**: Explicit `Result<T, E>` and `Option<T>` for error handling.
2. **Immutability**: Data structures are immutable; transformations return new values.
3. **Pure Functions**: Business logic has no side effects.
4. **Explicit Dependencies**: Inject dependencies, avoid hidden states.


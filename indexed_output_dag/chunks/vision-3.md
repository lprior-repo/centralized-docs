---
doc_id: vision
chunk_id: vision#3
chunk_type: prose
heading: Design Principles (Dave Farley)
token_count: 275
summary: # Vision: Centralized Docs as Infrastructure. ## Philosophy
---

# Vision: Centralized Docs as Infrastructure

## Philosophy

Documentation should be:
1. **Version controlled**: Git-backed, full change history
2. **Searchable**: Indexed, full-text search capabilities
3. **Portable**: Markdown/text format, no vendor lock-in
4. **Traceable**: Single correlation ID per operation
5. **Testable**: 90%+ test coverage, CI/CD enforced

## Goals

- **Simple**: Pure GoFr, zero external CLI frameworks, minimal dependencies
- **Fast**: In-memory search, content-addressed storage, efficient caching via Moon
- **Reliable**: Test-Driven Development, hexagonal architecture, local CI/CD
- **Observable**: OpenTelemetry tracing throughout, one trace per command

## Design Principles (Dave Farley)

1. **Modularity**: Well-defined modules with clear boundaries (ports & adapters)
2. **Cohesion**: Each module does ONE thing exceptionally well
3. **Separation of Concerns**: Handlers ≠ Logic ≠ Storage (layered architecture)
4. **Abstraction**: Depend on interfaces, not concrete implementations
5. **Loose Coupling**: Swap storage backend without touching business logic


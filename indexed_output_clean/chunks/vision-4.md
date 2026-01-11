---
doc_id: vision
chunk_id: vision#4
chunk_type: prose
heading: Core Features
token_count: 531
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

## Core Features

### Import
- Scan directories recursively for markdown/text files
- Extract metadata (title, size, checksums)
- Content-addressed storage (SHA256)
- Batch operations with transactional safety

### Search
- Full-text search across all documents
- Filter by tags and categories
- In-memory index, sub-millisecond response times
- High-cardinality metadata in results

### Export
- Multiple output formats: Markdown, HTML, JSON
- Batch export with consistent structure
- Streaming for large document sets
- Format validation before write

### Organization
- Tag-based categorization (user-defined)
- Hierarchical categories (optional)
- Atomic updates to index
- Conflict resolution for duplicate titles

### Observability
- **Single Trace Per Command**: One OpenTelemetry trace per CLI invocation
- **Complete Context**: Every log includes command, user, pwd, timestamp
- **Queryable**: Jaeger/Zipkin integration for distributed tracing
- **AI-Ready**: High-cardinality data enables intelligent log analysis


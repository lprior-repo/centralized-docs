---
doc_id: vision
chunk_id: vision#2
chunk_type: prose
heading: Goals
token_count: 172
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


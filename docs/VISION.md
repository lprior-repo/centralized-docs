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

## Technical Implementation

### Architecture
- **Hexagonal Design** (Ports & Adapters):
  - GoFr CLI handlers (thin input/output)
  - Application service (pure business logic)
  - Ports (interfaces for repositories, searchers, exporters)
  - Adapters (implementations: filesystem, simple search, exporters)

### Functional Programming
- **Result[T]** and **Option[T]** types (no nil pointers)
- Pure function composition via FlatMap
- Immutable data structures
- Strong typing (DocumentID, Title, FileSize instead of string/int)
- Single I/O boundary (handlers only)

### Testing
- **Test-Driven Development**: Write tests before implementation
- **Table-Driven Tests**: Consistent pattern throughout
- **Mock Generation**: mockery for all interfaces
- **Coverage Requirements**: ≥90% enforced by CI

### CI/CD
- **Moon Tasks**: Advanced task orchestration locally
- **Dependency Graphs**: Tests depend on mocks, builds depend on tests
- **Caching Strategy**: Content-based cache invalidation
- **Coverage Enforcement**: Fail build if coverage drops below 90%

## Success Criteria

1. ✅ All commands functional (import, search, export, list, tag, delete)
2. ✅ ≥90% test coverage enforced by `moon run coverage-check`
3. ✅ Pure hexagonal design with dependency injection
4. ✅ Single trace per command with complete context
5. ✅ Complete documentation (VISION, ARCHITECTURE, AGENTS, CLAUDE)
6. ✅ One way to do everything (single patterns)
7. ✅ Local Moon CI/CD with zero external dependencies
8. ✅ Zero nil pointers via Result/Option types
9. ✅ Pure functions throughout codebase
10. ✅ Immutable data structures

## Use Cases

### Personal Knowledge Management
```bash
# Import my documentation
docmgr import --source ~/projects/docs --tags golang,architecture

# Search across all personal docs
docmgr search "distributed systems" --tag architecture

# Export to static site for hosting
docmgr export --format html --output ./public
```

### Team Documentation
```bash
# Version-controlled docs repository
git clone https://github.com/team/centralized-docs
cd centralized-docs

# Import all team wikis
docmgr import --source ./wikis --recursive --tags team,reference

# Search with full observability (traces in Jaeger)
docmgr search "API design" --tags team
# Trace ID: abc123... (query in http://localhost:16686)
```

### AI Agent Integration
```bash
# Complete execution context for AI tools
docmgr import --source ~/ai-training-docs --tags ml,training

# Every log contains:
# - TraceID: Single correlation ID
# - Command: "import"
# - User: "lewis"
# - PWD: "/home/lewis/src"
# - Timestamp: 2026-01-03T14:30:45Z
#
# AI tools can query: "Show me all imports by lewis in the last hour"
# Result: Complete trace with every decision point visible
```

## Future Roadmap (Possible, Not Committed)

- Database backend adapter (PostgreSQL, SQLite)
- Advanced search (regex, syntax highlighting)
- Collaborative editing (CRDT-based)
- Plugin system (custom importers/exporters)
- Web UI (read-only documentation viewer)
- Webhook integrations (GitHub, Slack)

---

**Status**: Ready for Phase 1 implementation (Domain Model + Ports)

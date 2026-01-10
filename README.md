# docmgr - Documentation Manager

A pure GoFr CLI tool for centralizing, indexing, and managing documentation with extreme engineering rigor.

## Features

- **Import**: Scan directories and centralize documents from multiple sources
- **Search**: Full-text search with tag and category filtering
- **Export**: Multiple output formats (Markdown, HTML, JSON)
- **Organize**: Tag and categorize documents for easy discovery
- **Observe**: Complete OpenTelemetry tracing, one trace per command
- **Quality**: ≥90% test coverage with TDD and hexagonal architecture

## Installation

### Build from Source
```bash
git clone https://github.com/lewisblake/centralized-docs
cd centralized-docs

# Install locally
moon run install

# Or build binary
moon run build
./bin/docmgr --help
```

### Go Install
```bash
go install github.com/lewisblake/centralized-docs/cmd/docmgr@latest
```

## Quick Start

### Import Documentation
```bash
# Import from a directory
docmgr import --source ~/my-docs --tags golang,tutorial

# Recursive import with categories
docmgr import --source ~/projects --recursive --categories work
```

### Search Documents
```bash
# Full-text search
docmgr search "GoFr CLI"

# Search with filters
docmgr search "distributed systems" --tags architecture --categories reference

# Limit results
docmgr search "database" --max-results 5
```

### Export Documents
```bash
# Export to HTML
docmgr export --format html --output ./site

# Export to Markdown
docmgr export --format markdown --output ./docs

# Export to JSON
docmgr export --format json --output ./data.json
```

### List & Manage
```bash
# List all documents
docmgr list

# List documents with specific tag
docmgr list --tag golang

# Add tags to a document
docmgr tag <doc-id> --add golang,tutorial

# Delete a document
docmgr delete <doc-id>
```

## Configuration

Configure via environment variables:

```bash
# Storage location
STORAGE_BASE_PATH=./docs/indexed

# Search settings
SEARCH_MAX_RESULTS=100

# Export defaults
EXPORT_DEFAULT_FORMAT=markdown

# Observability
TRACE_EXPORTER=jaeger          # zipkin, jaeger, otlp, gofr
TRACER_URL=localhost:14250
TRACER_RATIO=1.0               # 100% sampling
LOG_LEVEL=INFO
```

Create a `.env` file in your working directory:

```bash
# .env
STORAGE_BASE_PATH=./docs/indexed
TRACE_EXPORTER=zipkin
TRACER_URL=localhost:9411
```

## Architecture

### Hexagonal Design (Ports & Adapters)
```
GoFr CLI Handlers
    ↓
Application Service (Pure Business Logic)
    ↓
Ports (Interfaces)
    ↓
Adapters (File System, Search, Export)
```

### Single Unified Trace Per Command

Every CLI invocation creates ONE OpenTelemetry trace with complete context:

```
docmgr import --source ~/docs
│
├─ TraceID: abc123def456
├─ Command: import
├─ User: lewis
├─ PWD: /home/lewis/src
│
└─ Spans
   ├─ command-import (root)
   ├── validate-params
   ├── application-import
   │  ├─ importer-scan
   │  └─ repository-save
   │     ├─ hash-content
   │     ├─ write-file
   │     └─ update-index
   └─ response
```

Query in Jaeger/Zipkin by TraceID to see complete execution flow.

### Functional Programming

- **No Nil Pointers**: Result[T] and Option[T] types
- **Pure Functions**: Deterministic, referentially transparent
- **Immutable Data**: Value types, new instances on change
- **Function Composition**: FlatMap chains for error handling

## Development

### Prerequisites
- Go 1.24+
- Moon (task orchestration)
- golangci-lint (linting)

### Development Workflow

```bash
# Fast iteration with watch
moon run dev --watch

# Run quality checks
moon run quality

# View test coverage
moon run coverage-report
open coverage.html

# Full CI pipeline
moon run ci
```

### Testing

```bash
# Run all tests with coverage
moon run test

# View coverage percentage
moon run coverage-percent

# Enforce 90% minimum
moon run coverage-check

# Run integration tests
moon run test-integration
```

### Dependency Management

```bash
# Check for outdated dependencies
moon run deps-check

# Update dependencies
moon run deps-update
```

## Project Structure

```
centralized-docs/
├── cmd/docmgr/
│   └── main.go                 # CLI entry point
├── pkg/domain/
│   ├── types.go                # Domain model
│   ├── result.go               # Result[T] and Option[T]
│   └── errors.go               # Error types
├── internal/
│   ├── app/
│   │   └── application.go       # Business logic
│   ├── ports/
│   │   ├── repository.go        # Storage interface
│   │   ├── searcher.go          # Search interface
│   │   ├── exporter.go          # Export interface
│   │   └── importer.go          # Import interface
│   ├── adapters/
│   │   ├── repository/          # File system storage
│   │   ├── search/              # In-memory search
│   │   ├── export/              # Format exporters
│   │   └── import/              # File discovery
│   ├── handlers/
│   │   ├── import.go
│   │   ├── search.go
│   │   ├── export.go
│   │   ├── list.go
│   │   ├── tag.go
│   │   └── delete.go
│   └── observability/           # Tracing & logging
├── docs/
│   ├── VISION.md                # Project vision
│   ├── ARCHITECTURE.md          # System design
│   ├── AGENTS.md                # AI instructions
│   ├── CLAUDE.md                # Claude context
│   └── indexed/                 # Document storage
├── .moon/
│   ├── moon.yml                 # Moon config
│   └── tasks.yml                # Task definitions
└── README.md                    # This file
```

## Documentation

- **[VISION.md](docs/VISION.md)** - Project philosophy and goals
- **[ARCHITECTURE.md](docs/ARCHITECTURE.md)** - System design and layers
- **[AGENTS.md](docs/AGENTS.md)** - AI agent coding instructions
- **[CLAUDE.md](docs/CLAUDE.md)** - Claude Code context

## Testing

### Test Coverage Requirements
- **Overall project**: ≥90% (enforced by CI)
- **Domain model**: 100%
- **Application service**: ≥95%
- **Adapters**: ≥90%
- **Handlers**: ≥90%

### Test Patterns

All tests use table-driven pattern with `testify/assert`:

```go
func TestImportDocuments(t *testing.T) {
    tests := []struct {
        name    string
        opts    ImportOptions
        want    int
        wantErr bool
    }{
        {
            name: "successful import",
            opts: ImportOptions{SourcePath: "/test"},
            want: 3,
            wantErr: false,
        },
    }

    for _, tt := range tests {
        t.Run(tt.name, func(t *testing.T) {
            // Test logic
        })
    }
}
```

## Observability

### Tracing

All commands automatically create traces:

```bash
# Enable tracing to Jaeger
export TRACE_EXPORTER=jaeger
export TRACER_URL=localhost:14250

# Run command
docmgr import --source ~/docs

# View trace at http://localhost:16686
# Query by TraceID to see complete execution
```

### Logging

Every log includes unified context:
- `trace_id`: Correlation ID
- `command`: CLI command name
- `user`: Current user
- `pwd`: Working directory
- `timestamp`: When operation occurred

### Metrics

Each command reports:
- Duration
- Items processed
- Success/failure status

## CI/CD Pipeline

### Local Development
```bash
# Format code
moon run fmt

# Lint code
moon run lint

# Run tests
moon run test

# Check coverage
moon run coverage-check
```

### Full Pipeline
```bash
# Complete quality gates + build
moon run ci
```

Tasks enforce order:
1. Format code
2. Lint (requires formatted)
3. Generate mocks
4. Run tests (requires mocks)
5. Check coverage (requires tests)
6. Build binary (requires passing tests)

## Dependencies

### Required
- `gofr.dev` - CLI framework and observability

### Development
- `github.com/stretchr/testify` - Testing assertions and mocks
- `github.com/vektra/mockery` - Mock generation

## Design Principles

This project follows Dave Farley's continuous delivery principles:

1. **Modularity**: Clear module boundaries and responsibilities
2. **Cohesion**: Each module does one thing well
3. **Separation of Concerns**: Handler, Logic, and Storage clearly separated
4. **Abstraction**: Depend on interfaces, not implementations
5. **Loose Coupling**: Swap implementations without touching logic

## Contributing

### Before Starting
1. Read [docs/AGENTS.md](docs/AGENTS.md) for code standards
2. Read [docs/ARCHITECTURE.md](docs/ARCHITECTURE.md) for system design
3. Check existing code for patterns

### Development Process
1. Write failing tests first (TDD)
2. Implement minimal code to pass
3. Refactor for clarity
4. Run `moon run coverage-check` (must be ≥90%)
5. Create pull request

### Code Standards
- Pure GoFr (no Cobra)
- Hexagonal architecture (ports-first)
- Result[T] and Option[T] types (no nil)
- Table-driven tests (only pattern)
- ≥90% test coverage

## License

MIT License

## Status

**Phase**: Foundation Complete, Ready for Phase 1 (Domain Model + Ports)

See `/home/lewis/.claude/plans/snoopy-tinkering-patterson.md` for detailed implementation plan.

## Questions?

Refer to:
- **Code patterns**: [docs/AGENTS.md](docs/AGENTS.md)
- **System design**: [docs/ARCHITECTURE.md](docs/ARCHITECTURE.md)
- **Project vision**: [docs/VISION.md](docs/VISION.md)
- **Implementation plan**: `/home/lewis/.claude/plans/snoopy-tinkering-patterson.md`

---

**docmgr**: Centralized documentation infrastructure with extreme engineering rigor.

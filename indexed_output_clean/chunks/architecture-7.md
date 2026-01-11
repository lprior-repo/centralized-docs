---
doc_id: architecture
chunk_id: architecture#7
chunk_type: code
heading: Configuration
token_count: 2054
summary: # Architecture: Hexagonal Design with Pure GoFr.  GoFr CLI Handlers (Presentation Layer)
---

# Architecture: Hexagonal Design with Pure GoFr

## Layers

### 1. GoFr CLI Handlers (Presentation Layer)
**Location**: `internal/handlers/`

- Parse command parameters
- Validate user input
- Create unified trace context (CommandContext)
- Inject metadata into all logs
- Delegate to Application layer
- Format and return responses

**Philosophy**: Thin layer, all I/O here, no business logic

```go
// Example: ImportHandler
func ImportHandler(app Application) func(*gofr.Context) (any, error) {
    return func(ctx *gofr.Context) (any, error) {
        // 1. Create unified context
        cmdCtx := &observability.CommandContext{...}
        cmdCtx.InjectMetadata(ctx)

        // 2. Parse parameters
        sourcePath := ctx.Param("source")

        // 3. Delegate to pure function
        result := app.ImportDocuments(ctx, opts)

        // 4. Return response
        return result.Fold(err => error, count => response)
    }
}
```

### 2. Application Service (Orchestration Layer)
**Location**: `internal/app/application.go`

- Pure business logic, no side effects
- Orchestrates ports (Repository, Searcher, Exporter, Importer)
- Returns Result[T] or Option[T], never nil
- All methods testable without mocks
- Function composition via FlatMap chains

**Philosophy**: Pure functions only, dependencies are interfaces

```go
type Application struct {
    repo      ports.Repository
    searcher  ports.Searcher
    importer  ports.Importer
    exporters map[string]ports.Exporter
}

// Pure function: Same input → Same output
func (a Application) ImportDocuments(ctx context.Context, opts ports.ImportOptions) Result[int] {
    return a.importer.Scan(ctx, opts).
        FlatMap(func(docs DocumentList) Result[int] {
            return a.saveAllDocuments(ctx, docs)
        })
}
```

### 3. Ports (Interfaces Only)
**Location**: `internal/ports/`

- Define contracts without implementation
- Repository: Save, FindByID, FindAll, Delete, UpdateTags
- Searcher: Search (with query, tags, categories)
- Importer: Scan (file discovery with metadata extraction)
- Exporter: Export (format conversion)

**Philosophy**: Interfaces enable testing and swappability

```go
type Repository interface {
    Save(ctx context.Context, doc Document) Result[Document]
    FindByID(ctx context.Context, id DocumentID) Option[Document]
    FindAll(ctx context.Context) Result[DocumentList]
    Delete(ctx context.Context, id DocumentID) Result[DocumentList]
    UpdateTags(ctx context.Context, id DocumentID, tags TagList) Result[Document]
}
```

### 4. Adapters (Implementations)
**Location**: `internal/adapters/`

- **Repository**: File system storage (SHA256 content-addressing)
- **Searcher**: In-memory search (full-text with filters)
- **Importer**: File system scanner (recursive walking, metadata)
- **Exporters**: Format converters (Markdown, HTML, JSON)

**Philosophy**: Isolated implementations, can be swapped

```
internal/adapters/
├── repository/
│   ├── filesystem.go       # Content-addressed file storage
│   └── filesystem_test.go
├── search/
│   ├── simple.go           # In-memory index searching
│   └── simple_test.go
├── import/
│   ├── scanner.go          # Directory walking + metadata
│   └── scanner_test.go
└── export/
    ├── markdown.go         # Export to markdown
    ├── html.go             # Export to HTML
    ├── json.go             # Export to JSON
    └── exporters_test.go
```

## Single Trace Flow

Every CLI command creates exactly ONE trace that never fragments:

```
CLI Invocation: docmgr import --source ~/docs
│
├─ TraceID: abc123def456 (X-Correlation-ID, auto from GoFr)
├─ Command: import
├─ User: lewis
├─ PWD: /home/lewis/src/centralized-docs
├─ Timestamp: 2026-01-03T14:30:45Z
│
└─── Handler Span: "command-import"
     ├─ Log: "Starting import from /home/lewis/docs" [all context included]
     │
     ├─── Child Span: "validate-params"
     │    └─ Log: "Source path validation passed" [same context]
     │
     ├─── Child Span: "application-import"
     │    │
     │    ├─── Child Span: "importer-scan"
     │    │    └─ Logs for each file processed
     │    │
     │    └─── Child Span: "repository-save"
     │         └─ Logs for storage operations
     │
     └─ Log: "Import completed: 3 documents" [duration, final stats]
```

**Key Properties**:
- ✅ Single trace ID throughout
- ✅ Parent-child span relationships preserved
- ✅ All logs include command context
- ✅ Queryable in Jaeger/Zipkin by trace ID

## Functional Programming

### No Nil Pointers
```go
// ❌ FORBIDDEN: nil pointers
func FindDoc(id string) *Document, error

// ✅ REQUIRED: Option[T] type
func FindDoc(id DocumentID) Option[Document]
```

### Pure Functions Only
```go
// ❌ FORBIDDEN: Mutations, side effects
func (a *Application) ImportDocs(opts *ImportOptions) {
    opts.Tags = append(...)    // Mutation!
    a.repo.Save(...)           // Side effect!
}

// ✅ REQUIRED: Pure composition
func (a Application) ImportDocs(opts ImportOptions) Result[int] {
    return a.importer.Scan(ctx, opts).
        FlatMap(func(docs DocumentList) Result[int] {
            return a.saveAllDocuments(ctx, docs)
        })
}
```

### Result[T] Type
```go
type Result[T any] interface {
    Map(func(T) T) Result[T]                      // Transform value
    FlatMap(func(T) Result[T]) Result[T]          // Chain operations
    Fold(onErr func(error) T, onOk func(T) T) T   // Handle both cases
    IsOk() bool                                   // Check if success
    GetOrElse(T) T                                // Default value
    Match(onErr func(error), onOk func(T))       // Pattern match
}
```

### Immutable Data Structures
```go
// ✅ REQUIRED: Return new instances
func (d Document) WithTags(tags TagList) Document {
    d.Tags = tags
    d.UpdatedAt = time.Now()
    return d  // New instance
}

// Usage: Immutable pipeline
updated := doc.WithTag("golang").WithTag("tutorial")
```

## Dependency Injection

All dependencies wired in main.go:

```go
func main() {
    cli := gofr.NewCMD()

    // Wire dependencies
    repo := repository.NewFilesystem("./docs/indexed")
    searcher := search.NewSimple(repo)
    importer := import.NewScanner()
    exporters := map[string]ports.Exporter{
        "markdown": export.NewMarkdown(),
        "html":     export.NewHTML(),
        "json":     export.NewJSON(),
    }

    // Create application service
    app := app.NewApplication(repo, searcher, importer, exporters)

    // Register handlers
    cli.SubCommand("import", handlers.ImportHandler(app))
    cli.SubCommand("search", handlers.SearchHandler(app))
    // ... more commands

    cli.Run()
}
```

## Testing Strategy

### Unit Tests (Pure Functions)
- No external I/O
- No temporary directories
- Mock all interfaces
- Fast execution (milliseconds)
- ≥95% coverage for Application layer

### Integration Tests
- Real file I/O with temp directories
- All adapters together
- Real search, export, import
- ≥90% coverage for adapters

### End-to-End Tests
- Full CLI execution
- Real command-line invocation
- Verify complete workflows
- 5-10 tests for critical paths

### Coverage Requirements
- **Overall**: ≥90% (enforced by `moon run coverage-check`)
- **Domain**: 100%
- **Application**: ≥95%
- **Adapters**: ≥90%
- **Handlers**: ≥90%

## Moon CI/CD Pipeline

```bash
# Full pipeline with dependency ordering
moon run ci

# Breakdown:
# 1. fmt (format code)
# 2. lint (check formatting, style)
# 3. generate (create mocks)
# 4. test (run tests)
# 5. coverage-check (verify 90% threshold)
# 6. build (create binary)

# Parallel execution where possible
# Caching based on input file changes
# Custom Go cache directories in .moon/cache/
```

## Configuration

Environment variables only (GoFr pattern):

```bash
# .env
APP_NAME=docmgr
APP_VERSION=0.1.0

STORAGE_BASE_PATH=./docs/indexed
SEARCH_MAX_RESULTS=100
EXPORT_DEFAULT_FORMAT=markdown

# Observability
TRACE_EXPORTER=jaeger
TRACER_URL=localhost:14250
TRACER_RATIO=1.0
LOG_LEVEL=INFO
```


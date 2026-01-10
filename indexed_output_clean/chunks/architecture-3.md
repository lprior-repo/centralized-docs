---
doc_id: architecture
chunk_id: architecture#3
chunk_type: code
heading: Functional Programming
token_count: 1534
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


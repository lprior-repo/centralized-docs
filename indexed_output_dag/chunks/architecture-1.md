---
doc_id: architecture
chunk_id: architecture#1
chunk_type: prose
heading: Layers
token_count: 877
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


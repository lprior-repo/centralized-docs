---
doc_id: agents
chunk_id: agents#8
chunk_type: code
heading: Common Mistakes to Avoid
token_count: 2129
summary: # AI Agent Instructions. This document is for Claude and other AI agents working on the codebase
---

# AI Agent Instructions

This document is for Claude and other AI agents working on the codebase.

## Project Overview

**centralized-docs** is a pure GoFr CLI tool for managing indexed documentation with extreme engineering rigor.

- **Framework**: Pure GoFr (no Cobra, no external CLI frameworks)
- **Architecture**: Hexagonal design (Ports & Adapters)
- **Code Style**: Functional programming, pure functions, no nil pointers
- **Testing**: Test-Driven Development, table-driven tests, ≥90% coverage
- **CI/CD**: Moon task orchestration with dependency graphs and aggressive caching

## Code Style Requirements

### 1. Pure GoFr Only
- Use GoFr's `NewCMD()` and `SubCommand()` exclusively
- No Cobra, no other CLI frameworks
- GoFr's built-in context logger and tracing
- Environment variables for configuration (GoFr pattern)

### 2. Functional Programming
- **Result[T]** type for error handling (not error returns)
- **Option[T]** type for optional values (not nil pointers)
- Pure functions with no side effects in business logic
- Function composition via FlatMap chains
- Immutable data structures (value receivers, new instances)

### 3. Hexagonal Architecture
- Ports in `internal/ports/` (interfaces only)
- Adapters in `internal/adapters/` (implementations)
- Application service in `internal/app/` (orchestration)
- Handlers in `internal/handlers/` (I/O boundary)

### 4. Dependency Injection
```go
// ✅ CORRECT: Constructor with dependencies
func NewApplication(
    repo ports.Repository,
    searcher ports.Searcher,
    importer ports.Importer,
    exporters map[string]ports.Exporter,
) Application {
    return Application{
        repo:      repo,
        searcher:  searcher,
        importer:  importer,
        exporters: exporters,
    }
}

// ❌ WRONG: Global variables, package-level state
var globalRepo Repository
```

### 5. Type Safety
- Use strongly typed primitives, not raw strings
- Example: `type DocumentID string`, `type Title string`
- Never stringly-type your API

### 6. Error Handling
```go
// ✅ CORRECT: Return Result[T]
func (a Application) Save(doc Document) Result[Document] {
    // Returns Ok(doc) or Err(error)
}

// ❌ WRONG: Error as return value
func (a Application) Save(doc Document) error
```

### 7. Tracing
Every significant operation must create a span:

```go
// ✅ CORRECT: Create span for every operation
span := ctx.Trace("operation-name")
defer span.End()
span.SetAttribute("key", "value")

// Logs automatically include unified context
ctx.Logger.Infof("Processing...", "attribute", value)
```

## Testing Requirements

### 1. Test-Driven Development
- Write tests BEFORE implementation
- Red → Green → Refactor cycle
- All tests passing before committing

### 2. Table-Driven Tests (The ONLY Pattern)
```go
// ✅ CORRECT: Table-driven
func TestImportDocuments(t *testing.T) {
    tests := []struct {
        name    string
        opts    ImportOptions
        want    int
        wantErr bool
    }{
        {
            name:    "successful import",
            opts:    ImportOptions{SourcePath: "/test"},
            want:    3,
            wantErr: false,
        },
        {
            name:    "source not found",
            opts:    ImportOptions{SourcePath: "/nonexistent"},
            want:    0,
            wantErr: true,
        },
    }

    for _, tt := range tests {
        t.Run(tt.name, func(t *testing.T) {
            // Test logic here
        })
    }
}

// ❌ WRONG: Individual test functions
func TestImportSuccess(t *testing.T) { }
func TestImportError(t *testing.T) { }
```

### 3. Mocking Strategy
- Use mockery to generate mocks: `mockery --name=Repository --dir=internal/ports`
- Mock only interfaces (ports)
- Never mock concrete types
- Mock setup in test table

```go
mockRepo := new(MockRepository)
mockRepo.On("Save", mock.Anything, mock.Anything).Return(nil)
app := app.NewApplication(mockRepo, ...)
```

### 4. Coverage Requirements
- Overall project: ≥90% (enforced by `moon run coverage-check`)
- Domain model: 100%
- Application layer: ≥95%
- Adapters: ≥90%
- Handlers: ≥90%

### 5. Test Execution
```bash
# Run all tests with coverage
moon run test

# View coverage report
moon run coverage-report
open coverage.html

# Verify 90% threshold
moon run coverage-check
```

## File Organization

### When Adding a Feature

1. **Create port (interface)** in `internal/ports/`
   - Define the contract
   - Use Result[T] and Option[T]

2. **Write failing tests** for the interface
   - Table-driven
   - Mock any dependencies

3. **Implement adapter** in `internal/adapters/`
   - Implement the port
   - Keep it focused (single responsibility)

4. **Write failing tests** for the adapter
   - Integration tests with real I/O
   - Temp directories for file operations

5. **Update application service** in `internal/app/`
   - Compose the new adapter with existing ones
   - Use FlatMap for orchestration

6. **Write failing tests** for application changes
   - Mock new and existing ports
   - Verify orchestration logic

7. **Add handler** in `internal/handlers/`
   - Thin layer: parse params, delegate, return response
   - Create unified trace context
   - Inject metadata

8. **Write handler tests**
   - Mock application
   - Test parameter parsing and error cases

9. **Register in main.go**
   - Wire all dependencies
   - Register with `cli.SubCommand()`

10. **Verify test coverage**
    - Run `moon run coverage-check`
    - Must be ≥90%

## Common Patterns

### Option[T] Usage
```go
// Finding optional values
func FindDoc(id DocumentID) Option[Document] {
    if doc, found := index[id]; found {
        return Some(doc)
    }
    return None[Document]()
}

// Using Option
doc := app.FindDoc(id)
doc.Match(
    func() { /* not found */ },
    func(d Document) { /* found */ },
)
```

### Result[T] Usage
```go
// Operations that can fail
func Save(doc Document) Result[Document] {
    if err := validate(doc); err != nil {
        return Err[Document](err)
    }
    return Ok(doc)
}

// Chaining with FlatMap
return repo.Save(doc).
    FlatMap(func(saved Document) Result[int] {
        return index.Add(saved)
    })
```

### Pure Function Composition
```go
// ✅ CORRECT: FlatMap chain (declarative)
return a.importer.Scan(ctx, opts).
    FlatMap(func(docs DocumentList) Result[int] {
        return a.saveAllDocuments(ctx, docs)
    })

// ❌ WRONG: Imperative error checking
docs, err := a.importer.Scan(ctx, opts)
if err != nil {
    return 0, err
}
// ...
```

### Immutable Updates
```go
// ✅ CORRECT: Return new instance
func (d Document) WithTag(tag string) Document {
    d.Tags = append(d.Tags, tag)
    return d
}

// Usage: Immutable pipeline
updated := doc.WithTag("golang").WithTag("tutorial")

// ❌ WRONG: Mutating input
func (a Application) AddTag(d *Document, tag string) {
    d.Tags = append(d.Tags, tag)  // Mutation!
}
```

## Moon Commands

### Development
```bash
# Format, lint, test in development mode with watch
moon run dev --watch

# Quick quality check
moon run quality

# Full CI pipeline
moon run ci
```

### Debugging
```bash
# View coverage report
moon run coverage-report
open coverage.html

# Check dependencies
moon run deps-check

# Clean all artifacts
moon run clean
```

## Making Changes

### Before Starting
1. Read the plan file: `/home/lewis/.claude/plans/snoopy-tinkering-patterson.md`
2. Check existing tests for patterns
3. Review `internal/ports/` for interface definitions
4. Read `docs/ARCHITECTURE.md` for layer responsibilities

### During Implementation
1. Write failing test first
2. Implement minimal code to pass
3. Refactor for clarity
4. Verify coverage with `moon run test`
5. Run `moon run quality` to check everything

### Before Committing
1. `moon run ci` - full pipeline must pass
2. `moon run coverage-check` - ensure ≥90%
3. Run linter manually: `golangci-lint run`
4. All tests passing: `go test ./...`

## Common Mistakes to Avoid

1. **Using nil pointers** - Use Option[T] instead
2. **Returning errors** - Use Result[T] instead
3. **Imperative error checks** - Use FlatMap chains
4. **Mutating inputs** - Return new instances
5. **Global state** - Use dependency injection
6. **Logging without context** - Include trace_id, command, user
7. **Mocking concrete types** - Mock only interfaces
8. **Individual test functions** - Use table-driven tests
9. **Side effects in pure functions** - Keep logic pure, I/O in handlers
10. **Ignoring coverage** - Run `moon run coverage-check` regularly


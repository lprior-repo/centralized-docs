---
doc_id: agents
chunk_id: agents#4
chunk_type: code
heading: File Organization
token_count: 1370
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


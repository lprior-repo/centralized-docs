---
doc_id: agents
chunk_id: agents#2
chunk_type: prose
heading: Code Style Requirements
token_count: 642
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


# Claude Code Context

This document provides context for Claude (and other AI tools) working on this codebase.

## Project Summary

**centralized-docs** is a pure GoFr CLI tool for managing indexed documentation with extreme engineering rigor, following Dave Farley's continuous delivery principles.

### Core Characteristics
- **Framework**: Pure GoFr only (no Cobra, no other CLI frameworks)
- **Architecture**: Hexagonal design (Ports & Adapters)
- **Language**: Pure functional Go (Result[T], Option[T], no nil pointers)
- **Quality**: ≥90% test coverage enforced by CI
- **CI/CD**: Local Moon task orchestration
- **Observability**: One trace per CLI command with unified context

## Key Design Patterns

### 1. Hexagonal Architecture
```
Handlers (GoFr CLI) → Application Service → Ports (Interfaces) ↔ Adapters (Implementations)
```

Each layer has a single responsibility:
- **Handlers**: Parse input, delegate, return output
- **Application**: Pure business logic, orchestration
- **Ports**: Interface contracts
- **Adapters**: Specific implementations (filesystem, search, etc.)

### 2. Functional Programming Constraints
- **No nil pointers**: Use Result[T] and Option[T] exclusively
- **Pure functions**: No mutations, no hidden I/O outside of handlers
- **Function composition**: Use FlatMap chains, not imperative if-error checks
- **Immutable data**: Value types, new instances on change
- **Strong typing**: DocumentID instead of string, Title instead of string

### 3. Test-Driven Development
- Write failing tests first
- Table-driven test pattern (the ONLY pattern)
- Mock interfaces using mockery
- Coverage enforcement: ≥90% project-wide

### 4. One Way to Do Everything
- One command structure pattern (SubCommand)
- One error handling strategy (Result[T])
- One testing pattern (table-driven)
- One configuration method (environment variables)
- One logging approach (ctx.Logger with metadata)
- One tracing pattern (ctx.Trace() spans)

## Project Structure

```
centralized-docs/
├── cmd/docmgr/
│   └── main.go                    # CLI bootstrap only
│
├── pkg/domain/
│   ├── types.go                   # Domain model (strongly typed)
│   ├── result.go                  # Result[T] and Option[T] types
│   └── errors.go                  # Domain-specific errors
│
├── internal/
│   ├── app/
│   │   └── application.go          # Pure business logic
│   │
│   ├── ports/                      # Interfaces ONLY
│   │   ├── repository.go
│   │   ├── searcher.go
│   │   ├── exporter.go
│   │   └── importer.go
│   │
│   ├── adapters/                   # Implementations
│   │   ├── repository/filesystem.go
│   │   ├── search/simple.go
│   │   ├── export/{markdown,html,json}.go
│   │   └── import/scanner.go
│   │
│   ├── handlers/                   # GoFr command handlers
│   │   ├── import.go
│   │   ├── search.go
│   │   ├── export.go
│   │   ├── list.go
│   │   ├── tag.go
│   │   └── delete.go
│   │
│   └── observability/              # Tracing & logging
│       ├── context.go              # Unified trace carrier
│       ├── logging.go              # Logging helpers
│       └── tracing.go              # Span management
│
├── docs/
│   ├── VISION.md                   # Why this project exists
│   ├── ARCHITECTURE.md             # System design
│   ├── AGENTS.md                   # AI agent instructions
│   ├── CLAUDE.md                   # This file
│   └── indexed/                    # Runtime storage
│
├── .moon/
│   ├── moon.yml                    # Moon workspace config
│   └── tasks.yml                   # Task definitions
│
├── .golangci.yml                   # Linter config (strict)
├── go.mod / go.sum                 # Dependencies (minimal)
└── README.md                       # User guide
```

## Before Making Changes

1. **Read the complete plan**: `/home/lewis/.claude/plans/snoopy-tinkering-patterson.md`
   - Contains all architectural decisions
   - Code examples for every pattern
   - 10 implementation phases with specifics

2. **Understand the current phase**:
   - Phase 0: Foundation (COMPLETE)
   - Phase 1: Domain Model + Ports (NEXT)
   - Phases 2-10: Progressive implementation

3. **Review existing code** for patterns:
   - How tests are structured
   - How Result[T] is used
   - How handlers create spans
   - How Application orchestrates

4. **Check `docs/AGENTS.md`** for coding standards

## Key Patterns in This Codebase

### Interfaces (Ports)
All return Result[T] or Option[T], never nil pointers:

```go
type Repository interface {
    Save(ctx context.Context, doc Document) Result[Document]
    FindByID(ctx context.Context, id DocumentID) Option[Document]
    FindAll(ctx context.Context) Result[DocumentList]
    Delete(ctx context.Context, id DocumentID) Result[DocumentList]
}
```

### Application Service
Pure function composition via FlatMap:

```go
func (a Application) ImportDocuments(ctx context.Context, opts ImportOptions) Result[int] {
    return a.importer.Scan(ctx, opts).
        FlatMap(func(docs DocumentList) Result[int] {
            return a.saveAllDocuments(ctx, docs)
        })
}
```

### Handlers
Create unified context, delegate, return via Fold:

```go
func ImportHandler(app Application) func(*gofr.Context) (any, error) {
    return func(ctx *gofr.Context) (any, error) {
        cmdCtx := &observability.CommandContext{...}
        cmdCtx.InjectMetadata(ctx)
        result := app.ImportDocuments(ctx, opts)
        return result.Fold(
            func(err error) (any, error) { return nil, err },
            func(count int) (any, error) { return map[string]any{"imported": count}, nil },
        )
    }
}
```

### Tests (Table-Driven)
```go
func TestSomething(t *testing.T) {
    tests := []struct {
        name    string
        input   Type
        want    Type
        wantErr bool
    }{
        {name: "case 1", input: X, want: Y, wantErr: false},
        {name: "error case", input: Z, want: nil, wantErr: true},
    }

    for _, tt := range tests {
        t.Run(tt.name, func(t *testing.T) {
            // Test logic
        })
    }
}
```

### Tracing
Every operation gets a span with unified context:

```go
span := ctx.Trace("operation-name")
defer span.End()
span.SetAttribute("document_id", id)

ctx.Logger.Infof("Processing document",
    "doc_id", id,
    "size", doc.Size)
```

## Constraints & Requirements

### Must Haves
1. ✅ Pure GoFr (no Cobra)
2. ✅ Hexagonal architecture (ports-first)
3. ✅ Result[T] and Option[T] only (no nil)
4. ✅ Pure functions in Application layer
5. ✅ Table-driven tests (only pattern)
6. ✅ ≥90% coverage (enforced)
7. ✅ One trace per command
8. ✅ Moon task orchestration
9. ✅ Dave Farley principles throughout
10. ✅ One way to do everything

### Never Do
- ❌ Use Cobra or any other CLI framework
- ❌ Return nil pointers (use Option[T])
- ❌ Return errors (use Result[T])
- ❌ Mutate function parameters
- ❌ Put side effects in pure functions
- ❌ Use global variables
- ❌ Write individual test functions (table-driven only)
- ❌ Mock concrete types (mock interfaces only)
- ❌ Add features beyond what's planned
- ❌ Skip tests or reduce coverage

## Common Operations

### Running Tests
```bash
cd /home/lewis/src/centralized-docs

# Run all tests with coverage
moon run test

# Check coverage percentage
moon run coverage-percent

# View coverage report in browser
moon run coverage-report
open coverage.html

# Enforce 90% minimum
moon run coverage-check
```

### Quality Checks
```bash
# Format code
moon run fmt

# Run linter
moon run lint

# Full quality pipeline
moon run quality

# Full CI pipeline (everything)
moon run ci
```

### Development
```bash
# Fast development loop with watch
moon run dev --watch

# Clean all artifacts
moon run clean
```

### Dependencies
```bash
# Check for outdated deps
moon run deps-check

# Update all dependencies
moon run deps-update

# Regenerate mocks
moon run generate
```

## Implementation Workflow

When implementing a new feature:

1. **Create the port** (interface) in `internal/ports/`
   - Define the contract
   - Return Result[T] or Option[T]

2. **Write failing tests** for the interface
   - Table-driven
   - Mock any dependencies

3. **Implement adapter** in `internal/adapters/`
   - Implement the port interface
   - Keep it focused

4. **Write adapter tests**
   - Use temp directories
   - Test with real I/O

5. **Update Application** in `internal/app/`
   - Compose the new adapter
   - Write tests with mocks

6. **Add handler** in `internal/handlers/`
   - Parse params, inject context, delegate
   - Write handler tests

7. **Register in main.go**
   - Wire dependencies
   - Register with GoFr

8. **Verify coverage**
   - Run `moon run coverage-check`
   - Must be ≥90%

## Files to Review

### For Understanding the System
- `docs/VISION.md` - Why and what
- `docs/ARCHITECTURE.md` - How it's structured
- `/home/lewis/.claude/plans/snoopy-tinkering-patterson.md` - Complete plan with code examples

### For Coding Standards
- `docs/AGENTS.md` - Detailed coding patterns
- `.golangci.yml` - Linting rules
- Existing code in `internal/` - Real examples

### For Configuration
- `.moon/moon.yml` - Workspace setup
- `.moon/tasks.yml` - Task definitions
- `go.mod` - Dependencies

## Error Handling Pattern

```go
// ✅ CORRECT: Result[T] enables FlatMap chains
result := operation1().
    FlatMap(func(val1 T1) Result[T2] {
        return operation2(val1)
    }).
    FlatMap(func(val2 T2) Result[T3] {
        return operation3(val2)
    })

// Handle the final result
result.Match(
    func(err error) {
        ctx.Logger.Errorf("Failed: %v", err)
    },
    func(val T3) {
        ctx.Logger.Infof("Success: %v", val)
    },
)

// ❌ WRONG: Imperative error checking
val1, err := operation1()
if err != nil {
    return nil, err
}
val2, err := operation2(val1)
if err != nil {
    return nil, err
}
// ... boilerplate
```

## Logging Pattern

Every log must include context:

```go
// ✅ CORRECT: Complete context
ctx.Logger.Infof("Processing document",
    "trace_id", cmdCtx.TraceID,
    "command", "import",
    "document_id", doc.ID,
    "size", doc.Size,
    "user", "lewis")

// ❌ WRONG: No context, unsearchable
ctx.Logger.Infof("Processing file: %s", filename)
```

## Making Your First Change

1. Pick a task from the plan (Phases 1-3 are foundational)
2. Read the relevant section in the plan file
3. Read `docs/AGENTS.md` for pattern examples
4. Create failing tests first
5. Implement minimal code to pass
6. Run `moon run coverage-check` (must pass)
7. Commit with clear message

## Questions During Implementation?

1. **How should I handle X?** → Check docs/AGENTS.md patterns
2. **What's the pattern for Y?** → Search existing code in `internal/`
3. **Does this fit the architecture?** → Check docs/ARCHITECTURE.md layers
4. **Is this line correct?** → Compare with plan file examples

---

**Version**: Phase 0 Foundation Complete, Ready for Phase 1
**Last Updated**: 2026-01-03
**Status**: Active Development

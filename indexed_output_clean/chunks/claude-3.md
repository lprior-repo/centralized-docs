---
doc_id: claude
chunk_id: claude#3
chunk_type: prose
heading: Key Design Patterns
token_count: 781
summary: # Claude Code Context. This document provides context for Claude (and other AI tools) working on thi
---

# Claude Code Context

This document provides context for Claude (and other AI tools) working on this codebase.

## ⚠️ MANDATORY: TCR JAIL - READ FIRST

**YOU ARE IN A TCR (Test-Commit-Revert) JAIL. THERE IS NO ESCAPE.**

### Rules - NON-NEGOTIABLE

1. **NEVER use `git commit` directly** - Use `./scripts/tcr.sh "message"` ONLY
2. **NEVER use `git add` followed by `git commit`** - TCR handles this
3. **NEVER skip tests** - Tests MUST pass or changes are auto-reverted
4. **NEVER use `--no-verify`** - Git hooks enforce TCR, cannot be bypassed

### How TCR Works

```bash
# The ONLY way to commit:
./scripts/tcr.sh "Your commit message"

# What happens:
# 1. Code is formatted (go fmt)
# 2. Code is vetted (go vet)
# 3. Tests are run (go test ./...)
# 4. IF tests pass → Commit is created
# 5. IF tests fail → ALL CHANGES ARE REVERTED (no exceptions)
```

### Safe Rollback

If anything goes wrong, the user can always run:
```bash
./scripts/revert-to-safe.sh
# Returns to SAFE_POINT tag, nukes everything
```

### Consequences of Failure

- Tests fail → Your code is GONE
- Vet fails → Your code is GONE
- Format fails → Your code is GONE
- No partial commits allowed
- No "I'll fix it later"

**Write tests FIRST. Make them pass. Then commit.**

---

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


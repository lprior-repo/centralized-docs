# AI-Assisted Development Workflow

## The Bitter Truth

AI is a junior developer that:
- Types infinitely fast
- Never learns from mistakes
- Has PhD-level knowledge it will use to game your constraints
- Optimizes for "looks right" not "is right"

## The Solution: Contract-Driven Development

```
┌─────────────────────────────────────────────────────────────┐
│                      YOU (Human)                            │
│                                                             │
│  Write:                                                     │
│  ├── Tests (the contract)                                   │
│  ├── Architecture decisions                                 │
│  ├── Interface definitions                                  │
│  └── Acceptance criteria                                    │
│                                                             │
│  Review:                                                    │
│  └── AI output for "gaming" the constraints                 │
└─────────────────────────────────────────────────────────────┘
                          │
                          ▼
┌─────────────────────────────────────────────────────────────┐
│                      AI (Claude)                            │
│                                                             │
│  Writes:                                                    │
│  ├── Implementation code                                    │
│  ├── Boilerplate                                            │
│  ├── Refactoring                                            │
│  └── Documentation                                          │
│                                                             │
│  Cannot:                                                    │
│  ├── Commit directly                                        │
│  ├── Modify tests you wrote                                 │
│  └── Bypass quality gates                                   │
└─────────────────────────────────────────────────────────────┘
                          │
                          ▼
┌─────────────────────────────────────────────────────────────┐
│                  TCR Wrapper (Automated)                    │
│                                                             │
│  Layer 1: Correctness                                       │
│  └── go test ./... (your tests, not AI's)                   │
│                                                             │
│  Layer 2: Quality                                           │
│  ├── go vet                                                 │
│  ├── golangci-lint (complexity, patterns, style)            │
│  └── staticcheck                                            │
│                                                             │
│  Layer 3: Consistency                                       │
│  ├── gofmt                                                  │
│  └── goimports                                              │
│                                                             │
│  Result:                                                    │
│  ├── ALL PASS → auto-commit                                 │
│  └── ANY FAIL → revert everything                           │
└─────────────────────────────────────────────────────────────┘
```

## Usage

```bash
# Instead of: claude
# Run:
./scripts/tcr-claude.sh

# With a prompt:
./scripts/tcr-claude.sh "implement the FetchDocs function"
```

## How to Write Unambiguous Tests (AI Can't Game These)

### Bad: Tests AI Can Game

```go
// AI can write code that passes this but breaks on edge cases
func TestFetchDocs(t *testing.T) {
    result, err := FetchDocs("example.com")
    assert.NoError(t, err)
    assert.NotEmpty(t, result)
}
```

### Good: Comprehensive Contract

```go
func TestFetchDocs(t *testing.T) {
    tests := []struct {
        name        string
        url         string
        wantErr     bool
        errContains string
        validate    func(t *testing.T, result []Doc)
    }{
        {
            name: "valid URL returns docs",
            url:  "https://example.com/docs",
            validate: func(t *testing.T, result []Doc) {
                assert.GreaterOrEqual(t, len(result), 1)
                for _, doc := range result {
                    assert.NotEmpty(t, doc.Title)
                    assert.NotEmpty(t, doc.Content)
                    assert.True(t, doc.FetchedAt.Before(time.Now()))
                }
            },
        },
        {
            name:        "invalid URL returns error",
            url:         "not-a-url",
            wantErr:     true,
            errContains: "invalid URL",
        },
        {
            name:        "empty URL returns error",
            url:         "",
            wantErr:     true,
            errContains: "URL required",
        },
        {
            name:        "timeout is handled",
            url:         "https://httpstat.us/200?sleep=30000",
            wantErr:     true,
            errContains: "timeout",
        },
    }

    for _, tt := range tests {
        t.Run(tt.name, func(t *testing.T) {
            result, err := FetchDocs(tt.url)
            if tt.wantErr {
                assert.Error(t, err)
                if tt.errContains != "" {
                    assert.Contains(t, err.Error(), tt.errContains)
                }
                return
            }
            assert.NoError(t, err)
            if tt.validate != nil {
                tt.validate(t, result)
            }
        })
    }
}
```

### Even Better: Property-Based Tests

```go
func TestFetchDocsProperties(t *testing.T) {
    // Property: Any valid URL should return non-nil or error
    rapid.Check(t, func(t *rapid.T) {
        url := rapid.StringMatching(`https?://[a-z]+\.[a-z]+`).Draw(t, "url")
        result, err := FetchDocs(url)

        // Must return either result or error, never both nil
        assert.False(t, result == nil && err == nil)

        // Must not panic (implicit)
    })
}
```

## Preventing AI Gaming

### 1. Protect Your Tests

Add to `.claude/settings.local.json`:

```json
{
  "permissions": {
    "deny": [
      "Edit(*_test.go)",
      "Write(*_test.go)"
    ]
  }
}
```

Now AI can only write implementation, not change the contract.

### 2. Architecture Rules

Create `architecture_test.go`:

```go
func TestArchitectureRules(t *testing.T) {
    // Rule: Handlers can't import repositories directly
    // Must go through services
    result := analysistest.Run(t, ...)
    // ...
}
```

### 3. Code Review Checklist

Even with automation, spot-check AI output for:
- [ ] Did it split one function into many tiny ones just to pass complexity limits?
- [ ] Are the function names meaningful or just `doThing1`, `doThing2`?
- [ ] Did it add unnecessary abstraction?
- [ ] Does the implementation match the *spirit* of the tests?

## The 100x Workflow

1. **You write tests first** (10-20 mins)
   - Define the contract
   - Cover edge cases
   - Property-based for invariants

2. **AI implements** (instant)
   - Give it the failing tests
   - Let it iterate until tests pass

3. **TCR enforces** (automatic)
   - Tests pass
   - Linters pass
   - Code formatted

4. **You review** (5 mins)
   - Check for gaming
   - Verify architecture
   - Accept or reject

**Your time: 15-25 mins per feature**
**AI time: handles the grunt work**
**Quality: enforced by contracts**

## Install Dependencies

```bash
# Required
go install golang.org/x/tools/cmd/goimports@latest

# Recommended
go install github.com/golangci/golangci-lint/cmd/golangci-lint@latest
go install honnef.co/go/tools/cmd/staticcheck@latest

# For property-based testing
go get pgregory.net/rapid
```

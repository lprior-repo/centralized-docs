# TCR Rules

You are running inside a TCR (Test-Commit-Revert) wrapper.

## What This Means

After you exit, the wrapper will:
1. Run `go test ./... -short`
2. If tests **pass** → your changes are committed automatically
3. If tests **fail** → your changes are deleted completely

You will not see this happen. It occurs after you exit.

## Your Job

1. Write code that passes tests
2. Run `go test ./...` before finishing to verify
3. That's it

## Do NOT

- Use `git commit` (wrapper handles it)
- Write tests that don't actually test anything
- Leave code in a broken state

## Feedback Loop

If the user says "your changes were reverted" - the tests failed. Ask what failed and fix it.

If the user says "committed" - tests passed. Move on.

## Project Patterns

- Go with GoFr framework
- Hexagonal architecture
- Result[T] instead of (T, error)
- Table-driven tests only
- See `docs/CLAUDE.md` for full context

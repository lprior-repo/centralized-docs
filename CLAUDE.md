# TCR Rules

You are running inside a TCR (Test-Commit-Revert) wrapper.

## What This Means

After you exit, the wrapper will:
1. Run `cargo test --lib`
2. If tests **pass** → your changes are committed automatically
3. If tests **fail** → your changes are deleted completely

You will not see this happen. It occurs after you exit.

## Your Job

1. Write code that passes tests
2. Run `cargo test --all-features` before finishing to verify
3. That's it

## Do NOT

- Use `git commit` (wrapper handles it)
- Write tests that don't actually test anything
- Leave code in a broken state
- Ignore Clippy warnings (`cargo clippy --all-targets`)

## Feedback Loop

If the user says "your changes were reverted" - the tests failed. Ask what failed and fix it.

If the user says "committed" - tests passed. Move on.

## Project Patterns

- **Rust** pure implementation (no Go/Python/JavaScript)
- **Knowledge Graph**: DAG-based document relationships (petgraph)
- **Semantic Chunking**: Context-aware token-based document splits
- **Full-text Indexing**: Efficient search with metadata
- **Functional**: Composition via Result/Option types
- **Tests**: Unit tests with table-driven validation
- **Documentation**: Complete INDEX.json + COMPASS.md navigation

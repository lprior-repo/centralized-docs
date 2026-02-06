# Contributing to centralized-docs

Thank you for your interest in contributing to centralized-docs!

## Development Setup

### Prerequisites

- Rust 1.75+ (stable)
- Git

### Building

```bash
cd doc_transformer
cargo build --release
```

### Running Tests

```bash
cargo test --lib          # Unit tests (207 tests)
cargo test --tests        # Integration tests
cargo test                # All tests
```

## Code Standards

### Functional Rust Patterns

This project uses functional programming patterns:

- **Result/Option for errors** - No panics in production code
- **Immutability preferred** - Use `let` not `let mut` where possible
- **Composition** - Build complex operations from simple functions
- **Iterator chains** - Prefer `.map()`, `.filter()`, `.fold()` over loops

### Lints

The codebase enforces strict lints:

```rust
#![deny(clippy::unwrap_used)]
#![deny(clippy::panic)]
#![deny(clippy::expect_used)]
```

### Testing

- All new code requires tests
- Use table-driven tests for multiple cases
- Target 90%+ coverage

## Pull Request Process

1. Fork the repository
2. Create a feature branch (`git checkout -b feature/my-feature`)
3. Write tests for your changes
4. Ensure all tests pass (`cargo test`)
5. Run clippy (`cargo clippy --all-targets`)
6. Commit with a clear message
7. Push and open a PR

## Architecture

See [docs/ARCHITECTURE.md](docs/ARCHITECTURE.md) for system design.

### Key Modules

| Module | Purpose |
|--------|---------|
| `discover` | Find markdown files |
| `analyze` | Extract metadata |
| `chunk` | Semantic chunking |
| `graph` | Knowledge DAG |
| `index` | INDEX.json generation |
| `scrape` | Web scraping |
| `search` | BM25 search |

## Reporting Issues

Please include:
- Rust version (`rustc --version`)
- OS and version
- Steps to reproduce
- Expected vs actual behavior

## License

By contributing, you agree that your contributions will be licensed under the MIT License.

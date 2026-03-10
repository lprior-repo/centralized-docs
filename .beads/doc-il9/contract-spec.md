# Contract Specification

## Context
- **Feature:** Static regex compilation for `extract_title` in validation.rs
- **Bead:** doc-il9
- **Problem:** `extract_title` function compiles regex on every call in hot path
- **Domain terms:**
  - `LazyLock<T>` - Rust's standard library lazy initialization primitive
  - `Regex` - Regular expression from the `regex` crate
- **Assumptions:**
  - The regex pattern `r"^#\s+(.+)$"` is valid and well-formed
  - The function should handle empty markdown gracefully
  - The function should fallback to URL-derived title when no H1 found

## Preconditions
- [ ] P1: `markdown` parameter must be a valid UTF-8 string (enforced by Rust type system)
- [ ] P2: `url` parameter must be a valid URL string (enforced by Rust type system)

## Postconditions
- [ ] Q1: Function returns a non-empty String when markdown contains valid H1 header
- [ ] Q2: Function returns URL-derived fallback when no H1 found
- [ ] Q3: Function returns "Untitled" when both markdown and URL are invalid
- [ ] Q4: Regex compilation happens exactly once (on first call), not on every invocation

## Invariants
- [ ] I1: The H1 regex is compiled exactly once and reused for all subsequent calls
- [ ] I2: No panics or unwraps in the hot path

## Error Taxonomy
- No error variants - this function returns String, not Result
- The function uses fallbacks instead of errors

## Contract Signatures
```rust
/// Extract title from markdown content
/// Regex is statically compiled using LazyLock for performance
pub fn extract_title(markdown: &str, url: &str) -> String
```

## Type Encoding
| Precondition | Enforcement Level | Type / Pattern |
|---|---|---|
| markdown is valid UTF-8 | Compile-time | `&str` (Rust guarantee) |
| url is valid string | Compile-time | `&str` (Rust guarantee) |
| Regex is valid | Compile-time | Static LazyLock with expect() |

## Violation Examples
- N/A - This function uses fallbacks, not errors

## Ownership Contracts
- `&str` parameters are borrowed - no ownership transfer
- Returns owned `String` - caller receives ownership
- No mutation of input parameters

## Non-goals
- [ ] Changing the function signature to return `Result<String, Error>`
- [ ] Adding validation for markdown content structure
- [ ] Supporting other header levels (H2, H3, etc.) in extract_title

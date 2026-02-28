# Contract Specification: CLI Error Message Format Standard

## Context

- **Feature**: CLI Error Message Format Consistency
- **Bead ID**: doc-3o2b
- **Domain terms**:
  - `Error` - A failure condition that prevents command completion
  - `Warning` - A non-fatal condition that notifies the user
  - `CLI` - Command-line interface binary (`doc_transformer`)
- **Assumptions**:
  - This is a Rust CLI using `anyhow::Result` for error handling
  - Errors are displayed to stderr via the default anyhow behavior
  - The codebase already has a `DocTransformerError` enum in `errors.rs`
- **Open questions**: None — requirements are clear from the bead specification

---

## Preconditions

- [ ] **P1**: Command encounters an error condition (validation failure, I/O error, missing resource, etc.)
- [ ] **P2**: Error must be representable as a `Result<T, anyhow::Error>` that propagates to main()

---

## Postconditions

- [ ] **Q1**: ALL error messages output to stderr MUST start with the exact prefix `Error: ` (note the capital E and trailing space)
- [ ] **Q2**: Error messages MUST be parseable by external scripts — format must be deterministic
- [ ] **Q3**: Error context MUST be preserved through the error chain (use `.context()` or structured errors)
- [ ] **Q4**: Warnings (non-fatal) MUST start with `Warning: ` (capital W, trailing space)
- [ ] **Q5**: All error messages MUST be actionable — include hints or specific paths when applicable

---

## Invariants

- [ ] **I1**: No error message output by this CLI ever lacks the `Error: ` prefix (exception: clap's own usage errors which are handled by clap)
- [ ] **I2**: All error messages are valid UTF-8
- [ ] **I3**: Error messages do not contain unescaped newlines (use `; ` or ` - ` for compound messages)

---

## Error Taxonomy

All CLI errors fall into these categories:

| Error Category | Display Format | Example |
|----------------|----------------|---------|
| **InvalidInput** | `Error: {context}: {specific_problem}` | `Error: invalid value for --filter: regex pattern too long: 501 chars (max 500)` |
| **NotFound** | `Error: {resource_type} not found: {path_or_id}` | `Error: INDEX.json not found in /path/to/dir` |
| **PermissionDenied** | `Error: permission denied: {path}` | `Error: permission denied: cannot write to output directory '/foo'` |
| **Validation** | `Error: validation failed: {reason}` | `Error: validation failed: query too long (2000 bytes, maximum 1000)` |
| **Operation** | `Error: {operation} failed: {reason}` | `Error: scrape failed: all pages filtered out by query 'x' (threshold: 0.1)` |
| **Configuration** | `Error: configuration error: {details}` | `Error: configuration error: missing required key 'default_category'` |

---

## Contract Signatures

```rust
/// Main entry point returns anyhow::Result<()>
/// Errors propagate to main() and are displayed by anyhow's default handler
fn main() -> anyhow::Result<()> {
    // ... command execution
    // All bail!/anyhow! calls must use the standardized format
}
```

All internal functions that return `Result<T, anyhow::Error>` must format errors as:

```rust
anyhow::bail!("Error: {context}: {specific_problem}")
// OR
anyhow::anyhow!("Error: {context}: {specific_problem}")
```

---

## Type Encoding

| Precondition | Enforcement Level | Type / Pattern |
|--------------|-------------------|----------------|
| Error message format | Compile-time | Use `#[track_caller]` with a formatting helper function |
| Error prefix present | Runtime-checked | `ensure_error_prefix()` validation helper |
| Error message non-empty | Compile-time | Custom `NonEmptyString` newtype where needed |

---

## Violation Examples (REQUIRED)

### VIOLATES Q1 (Missing Error: prefix)

```bash
# Given: program runs with invalid argument
$ doc_transformer index ./nonexistent --output /tmp/out

# Current (BAD - inconsistent):
#   Error: Source not found: ./nonexistent
# (This is actually correct - has prefix)

# Another location in codebase (transform.rs line 109):
#   TRANSFORM ERROR: /path/to/file.md: <message>
#   - This VIOLATES Q1 - missing "Error: " prefix

# Another location (analyze.rs line 115):
#   ANALYZE ERROR: /path/to/file.md: <message>
#   - This VIOLATES Q1 - uses "ANALYZE ERROR:" not "Error:"
```

### VIOLATES Q2 (Non-parseable format)

```rust
// Given: error with embedded newlines
anyhow::bail!("File not found\nPlease check the path");

// Current output:
//   File not found
//   Please check the path
// This VIOLATES Q2 - embedded newline breaks script parsing
```

### VIOLATES Q3 (Lost context)

```rust
// Given: wrapped error loses context
fn process() -> Result<()> {
    let content = std::fs::read_to_string("data.json")?;
    // If read_to_string fails, error is "No such file or directory"
    // without context about WHICH file
}
```

### VIOLATES Q4 (Warning without prefix)

```rust
// Given: warning message in discover.rs line 70
eprintln!("Warning: Skipping path due to I/O error: {e}");
// Current: Has "Warning: " prefix - CORRECT

// But in scrape/mod.rs line 67:
eprintln!("[SCRAPE] Sitemap found, using sitemap strategy...");
// VIOLATES Q4 - not a Warning, should not use eprintln! for info
```

### VIOLATES Q5 (Unactionable error)

```rust
// Given: vague error message
anyhow::bail!("Error: operation failed");
// VIOLATES Q5 - no specific path, reason, or hint
```

---

## Ownership Contracts (Rust-specific)

- **Main function**: Takes no ownership, returns `anyhow::Result<()>` — caller receives `Ok(())` or error
- **Error messages**: Always owned `String` via `.to_string()` or `format!()` to ensure valid UTF-8
- **Error context propagation**: Use `.context()` to preserve the error chain

---

## Implementation Requirements

### 1. Standardized Error Formatting Helper

Create a helper function in `main.rs` or a dedicated module:

```rust
/// Format an error message with the standardized CLI prefix.
/// All error messages from this CLI MUST use this function or equivalent format.
fn cli_error(context: &str, details: &str) -> String {
    format!("Error: {}: {}", context, details)
}

/// Format a warning message with the standardized CLI prefix.
fn cli_warning(message: &str) -> String {
    format!("Warning: {}", message)
}
```

### 2. Migration Guide

Replace existing error formats:

| Location | Current Format | Required Format |
|----------|-----------------|-----------------|
| `transform.rs:109` | `TRANSFORM ERROR: {path}: {msg}` | `Error: transform failed: {path}: {msg}` |
| `analyze.rs:115` | `ANALYZE ERROR: {path}: {msg}` | `Error: analysis failed: {path}: {msg}` |
| `scrape/mod.rs:67` | `[SCRAPE] Sitemap found...` | Use `println!` for info (not error/warning) |
| `discover.rs:70` | `Warning: Skipping path...` | Already correct, verify consistency |

### 3. Clap Integration

Clap's own usage errors (missing required args, invalid flag values) are handled by clap. These are acceptable as-is because they are external to our error handling domain. However, we can customize clap's error format by setting:

```rust
#[derive(Parser, Debug)]
#[command(name = "doc_transformer")]
#[command(error_style = clap::ErrorStyle::Fat)]
struct Cli { ... }
```

This makes clap errors also start with "error:" (lowercase), which is acceptable for usage errors.

### 4. Verification Script

After implementation, verify all error messages:

```bash
# Run commands that should fail and capture stderr
doc_transformer index ./nonexistent --output /tmp/out 2>&1 | head -1 | grep "^Error: " || echo "FAIL"
doc_transformer scrape invalid-url --output /tmp/out 2>&1 | head -1 | grep "^Error: " || echo "FAIL"
doc_transformer search "query" --index /nonexistent 2>&1 | head -1 | grep "^Error: " || echo "FAIL"
```

---

## Non-goals

- [ ] Changing the exit code strategy (currently: 0 = success, 1 = user error, 2 = pipeline error)
- [ ] Modifying the internal `DocTransformerError` enum — that's for library errors, not CLI display
- [ ] Adding internationalization (i18n) support
- [ ] JSON error output as an alternative format (future feature)

---

## Test Coverage Requirements

After implementation, ensure these scenarios are tested:

1. **Invalid input errors** - `--filter` with invalid regex shows `Error: invalid value for --filter: ...`
2. **Not found errors** - Missing source directory shows `Error: source not found: ...`
3. **Permission errors** - Unwritable output shows `Error: permission denied: ...`
4. **Validation errors** - Query too long shows `Error: validation failed: query too long...`
5. **Operation failures** - Scrape/index failures show `Error: {operation} failed: ...`
6. **Warning messages** - Non-fatal conditions show `Warning: ...`

---

## Exit Criteria

- [ ] Every error message output to stderr starts with `Error: ` (or `Warning: ` for warnings)
- [ ] No instance of `{OPERATION} ERROR:` format in the codebase
- [ ] All error messages include actionable context (paths, hints)
- [ ] Error messages use `; ` or ` - ` for compound messages (no embedded newlines)
- [ ] Verification script passes for all commands

# BEAD-004: Unbounded User Regex Input Allows ReDoS Attack

**Epic**: Security
**Severity**: High
**Status**: Open

---

## CONTEXT BLOCK

- **File/Function**: `doc_transformer/src/scrape.rs:138-143`
- **The Smell**: User-provided `--filter` regex is compiled and executed without complexity limits:
  ```rust
  let path_regex = config
      .path_filter
      .as_ref()
      .map(|p| Regex::new(p))  // User input directly compiled!
      .transpose()
      .context("Invalid path filter regex")?;
  ```
  A malicious or accidental pathological regex like `(a+)+$` or `(.*)*$` can cause catastrophic backtracking, freezing the application for minutes or hours on certain inputs.

---

## SPECIFICATION BLOCK

### 1. EARS (Easy Approach to Requirements Syntax)

| Trigger | System | Response |
|---------|--------|----------|
| User provides filter regex | `scrape_site` | Validate regex complexity before compilation |
| Regex takes >100ms to compile | `scrape_site` | Reject with error "Regex too complex" |
| Regex has known ReDoS patterns | `scrape_site` | Reject with warning about backtracking |

### 2. DbC (Design by Contract)

**Preconditions**:
- User provides `--filter` argument as string
- String is non-empty if provided

**Postconditions**:
- Regex compilation completes in <100ms
- Regex matching completes in <10ms per URL
- If validation fails, return clear error message
- Never hang or consume excessive CPU

**Invariants**:
- Maximum regex length: 500 characters
- No unbounded quantifiers on groups: `(a+)+`, `(.*)*`, `(a*)*`
- Use `regex::RegexBuilder` with `size_limit` and `dfa_size_limit`

### 3. Schema & Edge Cases

**Safe Regex Compilation**:
```rust
use regex::RegexBuilder;

fn compile_safe_regex(pattern: &str) -> Result<Regex> {
    // Reject patterns that are too long
    if pattern.len() > 500 {
        anyhow::bail!("Regex pattern too long (max 500 chars)");
    }

    // Check for known ReDoS patterns
    let redos_patterns = [
        r"\(\.\*\)\*",      // (.*)*
        r"\(\.\+\)\+",      // (.+)+
        r"\([^)]+\+\)\+",   // (a+)+
        r"\(\[^",           // Nested character classes
    ];
    for pat in &redos_patterns {
        if Regex::new(pat).ok().map(|r| r.is_match(pattern)).unwrap_or(false) {
            anyhow::bail!("Regex contains potentially slow pattern: {}", pat);
        }
    }

    RegexBuilder::new(pattern)
        .size_limit(1024 * 1024)      // 1MB compiled size limit
        .dfa_size_limit(1024 * 1024)  // 1MB DFA size limit
        .build()
        .context("Invalid or too complex regex pattern")
}
```

**Edge Cases**:
| Input | Expected Behavior |
|-------|-------------------|
| `(a+)+$` | Reject: "potentially slow pattern" |
| `(.*)*` | Reject: "potentially slow pattern" |
| Pattern > 500 chars | Reject: "pattern too long" |
| `^/docs/` (valid) | Accept and use |
| `[` (invalid syntax) | Reject with syntax error |
| Empty string | Accept (matches nothing) |

---

## FIX LOCATIONS

1. `src/scrape.rs:138-143` - Replace `Regex::new(p)` with safe compilation function
2. Add `compile_safe_regex` helper function
3. Consider using `regex::bytes::Regex` with explicit limits

---

## TEST CASES

```rust
#[test]
fn test_redos_pattern_rejected() {
    let config = ScrapeConfig {
        base_url: "https://example.com".to_string(),
        path_filter: Some("(a+)+$".to_string()),
        ..Default::default()
    };
    // Should fail fast, not hang
    let start = std::time::Instant::now();
    let result = tokio_test::block_on(scrape_site(&config));
    assert!(result.is_err());
    assert!(start.elapsed() < std::time::Duration::from_secs(1));
}

#[test]
fn test_long_pattern_rejected() {
    let long_pattern = "a".repeat(1000);
    let config = ScrapeConfig {
        base_url: "https://example.com".to_string(),
        path_filter: Some(long_pattern),
        ..Default::default()
    };
    let result = tokio_test::block_on(scrape_site(&config));
    assert!(result.is_err());
}
```

# BEAD-002: `.expect()` on Regex Captures Bypasses Panic-Free Guarantee

**Epic**: Code Safety
**Severity**: Medium
**Status**: Open

---

## CONTEXT BLOCK

- **File/Function**: Multiple locations using `.expect("capture group N")`:
  - `doc_transformer/src/scrape.rs:287` - `extract_title`
  - `doc_transformer/src/scrape.rs:310-311` - `extract_headers`
  - `doc_transformer/src/scrape.rs:325` - `extract_internal_links`
  - `doc_transformer/src/transform.rs:133, 156, 158` - `fix_headings`
- **The Smell**: The codebase has `#![deny(clippy::unwrap_used)]` but `.expect()` is not caught by this lint. While regex capture groups "should" exist after a successful match, the `.expect()` calls can still panic if:
  1. The regex pattern changes and capture groups shift
  2. Optional groups `()?` are added
  3. Refactoring introduces mismatched group indices

---

## SPECIFICATION BLOCK

### 1. EARS (Easy Approach to Requirements Syntax)

| Trigger | System | Response |
|---------|--------|----------|
| Regex captures a match | `extract_title` | Return title string or fallback value, never panic |
| H1 regex matches line | `extract_headers` | Extract level and text, or skip malformed matches |
| Link regex captures | `extract_internal_links` | Extract href or skip invalid captures |
| Heading regex matches | `fix_headings` | Adjust heading level or preserve original line |

### 2. DbC (Design by Contract)

**Preconditions**:
- Input is valid UTF-8 string
- Regex patterns are valid (tested at compile time via `Lazy`)

**Postconditions**:
- Function returns valid output (never panics)
- If capture group is missing, gracefully skip/fallback
- All `.expect()` calls replaced with `.map()` or `if let`

**Invariants**:
- Add `#![deny(clippy::expect_used)]` to enforce no-panic guarantee

### 3. Schema & Edge Cases

**Safe Pattern Using `if let`**:
```rust
// BEFORE (can panic)
let title = caps.get(1).expect("capture group 1").as_str();

// AFTER (safe)
if let Some(title_match) = caps.get(1) {
    let title = title_match.as_str();
    // use title
}
```

**Safe Pattern Using `map()`**:
```rust
// BEFORE
let level = caps.get(1).expect("capture group 1").as_str().len();

// AFTER
let level = caps.get(1).map(|m| m.as_str().len()).unwrap_or(1);
```

**Edge Cases**:
| Scenario | Expected Behavior |
|----------|-------------------|
| Regex matches but group 1 is empty | Return empty string or default |
| Regex has optional group `()?` | Handle `None` case gracefully |
| Capture group index out of bounds | Skip item, don't panic |

---

## FIX LOCATIONS

1. `src/scrape.rs:287` - Change to `caps.get(1).map(...).unwrap_or_else(...)`
2. `src/scrape.rs:310-311` - Use `if let Some(level) = caps.get(1)`
3. `src/scrape.rs:325` - Use `if let Some(href) = caps.get(2)`
4. `src/transform.rs:133, 156, 158` - Replace all `.expect()` with safe alternatives
5. `src/main.rs:12` - Add `#![deny(clippy::expect_used)]`

---

## TEST CASES

```rust
#[test]
fn test_extract_headers_malformed() {
    // Should not panic on edge cases
    let md = "# \n## Also empty\n###No space";
    let headers = extract_headers(md);
    // Should either extract valid headers or skip malformed ones
    assert!(headers.is_empty() || headers.iter().all(|h| h.level > 0));
}
```

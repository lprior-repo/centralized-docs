# BEAD-006: Lazy Static Regex Initialization Uses `.expect()` Which Can Panic

**Epic**: Code Safety
**Severity**: Low
**Status**: Open

---

## CONTEXT BLOCK

- **File/Function**: Multiple lazy static initializations:
  - `doc_transformer/src/scrape.rs:19-26`
  - `doc_transformer/src/chunk.rs:10-12`
  - `doc_transformer/src/transform.rs:11-21`
  - `doc_transformer/src/validate.rs:9-13`
- **The Smell**: While the codebase has `#![deny(clippy::unwrap_used)]`, all `Lazy::new()` blocks use `.expect()`:
  ```rust
  static H1_TITLE_REGEX: Lazy<Regex> =
      Lazy::new(|| Regex::new(r"^#\s+(.+)$").expect("valid H1 regex"));
  ```
  If a regex pattern is ever invalid (typo, bad escape), the `.expect()` panics at runtime on first access, not at compile time. This bypasses the "no panic" guarantee.

---

## SPECIFICATION BLOCK

### 1. EARS (Easy Approach to Requirements Syntax)

| Trigger | System | Response |
|---------|--------|----------|
| Application starts | Lazy regex initialization | Compile regex without panic risk |
| Regex pattern has syntax error | Initialization | Fail gracefully with context |

### 2. DbC (Design by Contract)

**Preconditions**:
- Regex patterns are hardcoded string literals
- Patterns should be valid at compile time

**Postconditions**:
- All regexes compile successfully
- No runtime panics from regex initialization
- If pattern is invalid, error is caught early in main()

**Invariants**:
- Use `const` regex patterns where possible
- Add `#![deny(clippy::expect_used)]` for consistency

### 3. Schema & Edge Cases

**Option A: Keep Lazy + Add Compile-Time Test**
```rust
static H1_TITLE_REGEX: Lazy<Regex> =
    Lazy::new(|| Regex::new(r"^#\s+(.+)$").expect("valid H1 regex"));

#[cfg(test)]
mod regex_tests {
    use super::*;

    #[test]
    fn test_all_regexes_compile() {
        // Force initialization of all lazy regexes
        let _ = H1_TITLE_REGEX.as_str();
        let _ = HEADER_REGEX.as_str();
        // ... all other regexes
    }
}
```

**Option B: Use `regex_static` or `once_cell` with Result**
```rust
use once_cell::sync::OnceCell;

static H1_TITLE_REGEX: OnceCell<Regex> = OnceCell::new();

fn get_h1_regex() -> &'static Regex {
    H1_TITLE_REGEX.get_or_init(|| {
        Regex::new(r"^#\s+(.+)$")
            .unwrap_or_else(|_| Regex::new(".*").unwrap()) // Fallback
    })
}
```

**Option C: Validate All Regexes at Startup**
```rust
fn validate_regexes() -> Result<()> {
    // Force all lazy regexes to initialize
    let _ = &*H1_TITLE_REGEX;
    let _ = &*HEADER_REGEX;
    let _ = &*LINK_REGEX;
    // If we get here, all compiled successfully
    Ok(())
}

fn main() -> Result<()> {
    validate_regexes()?;  // Early fail if any regex is bad
    // ... rest of main
}
```

**Edge Cases**:
| Scenario | Expected Behavior |
|----------|-------------------|
| All patterns valid | Normal operation |
| Pattern has typo (e.g., `[a-`) | Panic at first use (current), should fail at startup |
| Pattern is empty | Empty regex matches everything |

---

## FIX LOCATIONS

1. Add comprehensive test that forces all lazy regex initialization
2. Add `#![deny(clippy::expect_used)]` to main.rs
3. Consider `validate_regexes()` call at start of main()
4. Document that `.expect()` in lazy statics is acceptable IF covered by tests

---

## TEST CASES

```rust
#[test]
fn test_all_static_regexes_are_valid() {
    // This test will fail at compile time if any regex pattern is invalid
    use crate::scrape::{H1_TITLE_REGEX, HEADER_REGEX, LINK_REGEX};
    use crate::chunk::{H2_REGEX, TABLE_REGEX};
    use crate::transform::{HEADING_REGEX, H1_START_REGEX, H1_LINE_REGEX};
    use crate::validate::{H1_REGEX, TAGS_REGEX};

    // Force initialization - if any panic, test fails
    let _ = H1_TITLE_REGEX.as_str();
    let _ = HEADER_REGEX.as_str();
    let _ = LINK_REGEX.as_str();
    // ... etc
}
```

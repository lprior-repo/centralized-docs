# BEAD-001: String Slicing on Multi-byte Characters Causes Panic

**Epic**: Robustness
**Severity**: Critical
**Status**: Open

---

## CONTEXT BLOCK

- **File/Function**: `doc_transformer/src/chunk.rs:371-377` (`create_summary`)
- **Also in**:
  - `doc_transformer/src/llms.rs:207-214` (`truncate_summary`)
  - `doc_transformer/src/transform.rs:85-86` (`transform_file`)
- **The Smell**: The code assumes `&str[..N]` byte slicing is safe after checking `str.len() > N`, but `len()` returns byte count, not character count. If position N falls in the middle of a multi-byte UTF-8 character (emoji, accented chars, CJK), Rust will **panic** at runtime.

**Reproduction**:
```rust
let s = "Hello 🦀 Rust!";
println!("{}", s.len());        // 17 bytes
println!("{}", &s[..8]);        // PANIC: byte index 8 is not a char boundary
```

---

## SPECIFICATION BLOCK

### 1. EARS (Easy Approach to Requirements Syntax)

- **Format**: "When <Trigger> happens, the <System> shall <Response>."

| Trigger | System | Response |
|---------|--------|----------|
| User provides content with multi-byte UTF-8 characters | `create_summary` | Return a truncated string that respects character boundaries |
| Summary text contains emoji/accented chars | `truncate_summary` | Truncate at the last valid character boundary before max_len |
| First paragraph has multi-byte chars | `transform_file` | Slice content at character boundary, not byte position |

### 2. DbC (Design by Contract)

**Preconditions**:
- Input `text: &str` is valid UTF-8 (guaranteed by Rust's type system)
- `max_len` parameter is a positive integer representing desired character limit

**Postconditions**:
- Output is valid UTF-8 (no panics)
- Output length in characters is <= `max_len`
- Output does NOT end in the middle of a grapheme cluster
- If truncation occurs, "..." is appended

**Invariants**:
- Never use `&str[..byte_index]` without validating `is_char_boundary(byte_index)`

### 3. Schema & Edge Cases

**Safe Truncation Pattern**:
```rust
fn safe_truncate(text: &str, max_chars: usize) -> String {
    let boundary = text.char_indices()
        .take(max_chars)
        .last()
        .map(|(i, c)| i + c.len_utf8())
        .unwrap_or(text.len());
    text[..boundary].to_string()
}
```

**Edge Cases to Handle**:
| Input | Expected Behavior |
|-------|-------------------|
| `""` (empty string) | Return `""` |
| `"Hi"` with max_len=10 | Return `"Hi"` unchanged |
| `"Hello 🦀"` with max_len=6 | Return `"Hello "` (6 chars) |
| `"日本語テスト"` with max_len=3 | Return `"日本語"` |
| Content is all emoji | Truncate respecting emoji boundaries |
| `null`/`undefined` | N/A in Rust (type system prevents) |

---

## FIX LOCATIONS

1. `src/chunk.rs:371-377`: Replace `&summary[..200]` with char-boundary-safe truncation
2. `src/llms.rs:212`: Replace `&cleaned[..max_len.saturating_sub(3)]` with safe version
3. `src/transform.rs:85-86`: Replace `first_paragraph[..max_len]` with safe version

---

## TEST CASES

```rust
#[test]
fn test_truncate_multibyte_content() {
    // These must NOT panic
    let emoji_content = "Hello 🦀 Rust is awesome! 🎉";
    let result = create_summary(emoji_content);
    assert!(result.is_ascii() || result.chars().count() <= 200);

    let cjk_content = "日本語のドキュメントをテストします。";
    let result2 = create_summary(cjk_content);
    assert!(!result2.is_empty());
}
```

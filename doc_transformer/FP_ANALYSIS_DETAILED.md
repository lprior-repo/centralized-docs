# Detailed Functional Programming Analysis
## index.rs and validate.rs

---

## CRITICAL VIOLATION: validate.rs - Unsafe Regex Unwrap

### Location 1: Line 64-67
```rust
64  let h1_count = Regex::new(r"^# [^#]")
65      .unwrap()  // ❌ CRITICAL: Can panic if regex invalid
66      .find_iter(content)
67      .count();
```

**Issue:** Direct `.unwrap()` on `Regex::new()` result
- Will panic if regex pattern is invalid
- Violates FP rule: NO UNWRAP() CALLS
- No way to recover gracefully

**Current Behavior:**
- Regex pattern `r"^# [^#]"` is hardcoded and valid
- So won't panic in normal operation
- But pattern could be modified during maintenance

**Fix Options:**

Option A - Use lazy_static (Best):
```rust
use lazy_static::lazy_static;
use regex::Regex;

lazy_static! {
    static ref H1_REGEX: Regex = Regex::new(r"^# [^#]")
        .expect("H1 regex pattern is valid");
}

// Then use:
let h1_count = H1_REGEX.find_iter(content).count();
```

Option B - Return Result:
```rust
fn validate_file(content: &str) -> Result<(usize, usize), regex::Error> {
    let h1_regex = Regex::new(r"^# [^#]")?;
    // ...
}
```

Option C - Use once_cell (Rust 1.80+):
```rust
use std::sync::OnceLock;
use regex::Regex;

fn h1_regex() -> &'static Regex {
    static CELL: OnceLock<Regex> = OnceLock::new();
    CELL.get_or_init(|| Regex::new(r"^# [^#]").expect("valid regex"))
}
```

---

### Location 2: Line 86-88
```rust
86  if !Regex::new(r"tags:\s*\[[^\]]{10,}\]")
87      .unwrap()  // ❌ CRITICAL: Can panic if regex invalid
88      .is_match(content)
```

**Issue:** Identical to Location 1
- Pattern: `r"tags:\s*\[[^\]]{10,}\]"`
- Same solution applies

---

## MEDIUM VIOLATION: index.rs - Inefficient Option Handling

### Location: Line 242
```rust
240 let node = GraphNode {
241     id: chunk.chunk_id.clone(),
242     node_type: NodeType::Chunk,
243     title: format!(
244         "{} - {}",
245         chunk.doc_title,
246         chunk.heading.as_ref().unwrap_or(&"Intro".to_string())
247     ),
248     category: None,
249 };
```

**Issue:** Creating temporary String allocation in unwrap_or
- `.as_ref().unwrap_or(&"Intro".to_string())`
- Creates new String on every call when heading is None
- Creates &String reference to temporary

**Better Approach:**
```rust
title: format!(
    "{} - {}",
    chunk.doc_title,
    chunk.heading.as_deref().unwrap_or("Intro")
),
```

**Why Better:**
- `as_deref()` converts `Option<String>` to `Option<&str>`
- `unwrap_or("Intro")` uses string literal (no allocation)
- More idiomatic Rust
- More efficient

**Not a Panic Risk:** Unwrap_or always provides default, so safe

---

## PERFORMANCE VIOLATION: index.rs - O(n²) Lookup Complexity

### Location: Lines 275-307
```rust
275 for (i, chunk) in chunks.iter().enumerate() {
276     let chunk_tags = document_tags
277         .iter()
278         .find(|(id, _, _)| id == &chunk.doc_id)  // <-- O(n) lookup
279         .map(|(_, tags, _)| tags.clone())
280         .unwrap_or_default();
281
282     let chunk_category = document_tags
283         .iter()
284         .find(|(id, _, _)| id == &chunk.doc_id)  // <-- DUPLICATE O(n) lookup
285         .map(|(_, _, cat)| cat.clone())
286         .unwrap_or_default();
287
288     let all_chunks_metadata: Vec<(String, Vec<String>, String)> = chunks
289         .iter()
290         .enumerate()
290         .filter(|(j, _)| j != &i)
291         .map(|(_, c)| {
292             let tags = document_tags
293                 .iter()
294                 .find(|(id, _, _)| id == &c.doc_id)  // <-- O(n) lookup in nested loop
295                 .map(|(_, t, _)| t.clone())
296                 .unwrap_or_default();
297
298             let category = document_tags
299                 .iter()
300                 .find(|(id, _, _)| id == &c.doc_id)  // <-- DUPLICATE O(n) lookup
301                 .map(|(_, _, cat)| cat.clone())
302                 .unwrap_or_default();
303
304             (c.chunk_id.clone(), tags, category)
305         })
306         .collect();
307
308     let related = detector.detect_relationships(...)
309 }
```

**Issue:** Multiple linear scans over `document_tags` vector
- Outer loop: `chunks.iter()` - O(c) iterations
- Inner loops: nested vector iteration - O(t²) where t = document_tags length
- Total complexity: O(c * t²) instead of O(c)

**For typical inputs:** 100 chunks × 50 document tags = 100 × 2,500 = 250,000 iterations

**Solution: Build HashMap First**
```rust
// Pre-build lookup map O(t)
let tag_map: HashMap<String, (Vec<String>, String)> = document_tags
    .iter()
    .map(|(id, tags, cat)| {
        (id.clone(), (tags.clone(), cat.clone()))
    })
    .collect();

// Main loop is now O(c) with O(1) lookups
for (i, chunk) in chunks.iter().enumerate() {
    let (chunk_tags, chunk_category) = tag_map
        .get(&chunk.doc_id)
        .map(|x| (x.0.clone(), x.1.clone()))
        .unwrap_or_default();

    let all_chunks_metadata: Vec<_> = chunks
        .iter()
        .enumerate()
        .filter(|(j, _)| j != &i)
        .map(|(_, c)| {
            let (tags, category) = tag_map
                .get(&c.doc_id)
                .map(|x| (x.0.clone(), x.1.clone()))
                .unwrap_or_default();

            (c.chunk_id.clone(), tags, category)
        })
        .collect();

    // ... rest
}
```

**Impact:**
- Reduces from O(c * t²) to O(c * t)
- For typical case: 250,000 → 5,000 operations (50x improvement)

---

## DESIGN ISSUE: validate.rs - Tuple Return Type

### Location: Lines 59, 103
```rust
59  fn validate_file(content: &str) -> (usize, usize) {
    // ... validation logic ...
103     (errors, warnings)
    }
```

**Issue:** Bare tuple return type
- `(usize, usize)` doesn't clearly indicate which field is what
- Caller must remember: first=errors, second=warnings
- Could be confused with other tuples returning `(usize, usize)`
- Not idiomatic Rust for multi-field returns

**Usage Pattern (line 39):**
```rust
let (errors, warnings) = validate_file(&content);
```

**Better Approach: Custom Struct**
```rust
#[derive(Debug, Clone, Copy)]
pub struct FileValidationResult {
    pub errors: usize,
    pub warnings: usize,
}

fn validate_file(content: &str) -> FileValidationResult {
    let errors = 0;
    let warnings = 0;

    // ... validation logic ...

    FileValidationResult { errors, warnings }
}

// Usage becomes clearer:
let result = validate_file(&content);
if result.errors == 0 {
    files_passed += 1;
}
total_errors += result.errors;
total_warnings += result.warnings;
```

**Benefits:**
- Self-documenting
- Type-safe
- Extensible (can add more fields later)
- Better IDE support

---

## VALIDATION RULES TESTED

### Rule V001: Single H1
- **Check:** Regex `^# [^#]` (must have exactly one H1)
- **Line:** 64-67
- **Violations:** Increments errors

### Rule V002: Frontmatter Exists
- **Check:** `content.starts_with("---")`
- **Line:** 73-75
- **Violations:** Increments errors

### Rule V003: Required Fields
- **Check:** First 500 chars must contain: "id:", "title:", "category:", "tags:"
- **Line:** 77-83
- **Violations:** Each missing field increments errors (max 4)

### Rule V006: Minimum Tags
- **Check:** Regex `tags:\s*\[[^\]]{10,}\]` (at least 10 chars between brackets)
- **Line:** 86-91
- **Violations:** Increments warnings

### Rule V007: Context Block
- **Check:** Contains "> **Context**:"
- **Line:** 94-96
- **Violations:** Increments warnings

### Rule V008: See Also Section
- **Check:** Contains "## See Also"
- **Line:** 99-101
- **Violations:** Increments warnings

---

## PASSING PATTERNS

### ✅ Good: Result Type with ?
**Location:** index.rs lines 147, main.rs line 38
```rust
fs::write(index_file, serde_json::to_string_pretty(&index)?)?;
let (files, _discover_manifest) = discover::discover_files(&args.source_dir)?;
```
- Idiomatic error propagation
- No panic risk
- Clear error flow

### ✅ Good: if let for Optional
**Location:** validate.rs line 38
```rust
if let Ok(content) = fs::read_to_string(&path) {
    let (errors, warnings) = validate_file(&content);
```
- Safe optional handling
- Gracefully skips unreadable files
- No panic risk

### ✅ Good: Early Return
**Location:** validate.rs lines 23-30
```rust
if !docs_dir.exists() {
    return Ok(ValidationResult {
        files_checked: 0,
        files_passed: 0,
        total_errors: 0,
        total_warnings: 0,
    });
}
```
- Clear control flow
- Idiomatic Rust pattern
- Proper Result semantics

### ✅ Good: Pure Functions
**Location:** index.rs lines 194-216
```rust
fn extract_tags(analysis: &Analysis) -> Vec<String> { ... }
fn is_stopword(word: &str) -> bool { ... }
```
- No side effects
- Deterministic
- Testable
- Composable

---

## SUMMARY TABLE

| Issue | File | Lines | Severity | Type | FP Violation |
|-------|------|-------|----------|------|--------------|
| Regex unwrap #1 | validate.rs | 64-67 | 🔴 CRITICAL | panic | YES |
| Regex unwrap #2 | validate.rs | 86-88 | 🔴 CRITICAL | panic | YES |
| Inefficient Option | index.rs | 242-246 | 🟡 MEDIUM | performance | NO |
| O(n²) lookup | index.rs | 275-307 | 🟡 MEDIUM | performance | NO |
| Tuple return | validate.rs | 59,103 | 🟡 DESIGN | clarity | NO |

---

## Functional Programming Score Breakdown

### Error Handling: 90/100
- ✅ Uses Result<T> correctly in public APIs
- ✅ Uses ? operator for propagation
- ❌ 2 unsafe unwrap() calls on regex
- ❌ Tuple makes error context less clear

### Type Safety: 85/100
- ✅ Immutable references throughout
- ✅ No unsafe blocks
- ❌ Tuple return is less type-safe
- ⚠️ Option.unwrap_or with allocation

### Performance: 80/100
- ❌ O(n²) complexity in graph building
- ❌ Unnecessary allocations in unwrap_or
- ✅ Otherwise efficient functional style

### Purity: 95/100
- ✅ Pure functions (extract_tags, is_stopword)
- ✅ No global state
- ✅ Immutable data structures
- ⚠️ File I/O (intentional, acceptable)

### Overall FP Compliance: 87/100

# BEAD-003: BM25 Score Division by Zero Risk

**Epic**: Search Reliability
**Severity**: Medium
**Status**: Open

---

## CONTEXT BLOCK

- **File/Function**: `doc_transformer/src/filter.rs:323-347` (`bm25_score`)
- **The Smell**: The BM25 calculation uses `avg_doc_length` as a divisor without validation:
  ```rust
  let denominator = tf + k1 * (1.0 - b + b * (doc_length / avg_doc_length));
  ```
  If `avg_doc_length` is `0.0` (all documents are empty), this produces `Inf` or `NaN`, which propagates through search results and causes sorting failures or incorrect rankings.

**Also in**: `main.rs:470-474` calculates `avg_doc_length`:
```rust
let avg_doc_length = if !documents.is_empty() {
    total_words as f32 / documents.len() as f32
} else {
    100.0  // Good fallback, but what if total_words == 0?
};
```

---

## SPECIFICATION BLOCK

### 1. EARS (Easy Approach to Requirements Syntax)

| Trigger | System | Response |
|---------|--------|----------|
| All indexed documents have 0 words | `bm25_score` | Return score of 0.0, not NaN/Inf |
| Single document has 0 words | `bm25_score` | Skip document or return 0.0 score |
| avg_doc_length parameter is 0 | `bm25_score` | Use default value (e.g., 100.0) |

### 2. DbC (Design by Contract)

**Preconditions**:
- `document` is valid UTF-8 string
- `query` is non-empty string (validated in `run_search`)
- `avg_doc_length` should be > 0.0

**Postconditions**:
- Return value is finite (not NaN, not Inf)
- Return value is >= 0.0
- Function never panics

**Invariants**:
- `avg_doc_length.max(1.0)` should be used to prevent division by zero
- Output is always a valid f32 for comparison

### 3. Schema & Edge Cases

**Safe BM25 Implementation**:
```rust
pub fn bm25_score(document: &str, query: &str, avg_doc_length: f32) -> f32 {
    let k1 = 1.2;
    let b = 0.75;

    // Prevent division by zero
    let safe_avg = if avg_doc_length > 0.0 { avg_doc_length } else { 100.0 };

    let doc_words: Vec<&str> = document.split_whitespace().collect();
    let doc_length = doc_words.len() as f32;

    query
        .split_whitespace()
        .map(|term| {
            let term_lower = term.to_lowercase();
            doc_words.iter()
                .filter(|w| w.to_lowercase() == term_lower)
                .count() as f32
        })
        .filter(|&tf| tf > 0.0)
        .map(|tf| {
            let idf = (10.0_f32).ln();
            let numerator = tf * (k1 + 1.0);
            let denominator = tf + k1 * (1.0 - b + b * (doc_length / safe_avg));
            // Prevent division by zero in denominator
            if denominator > 0.0 {
                idf * (numerator / denominator)
            } else {
                0.0
            }
        })
        .sum()
}
```

**Edge Cases**:
| Input | Expected Result |
|-------|-----------------|
| Empty document, valid query | `0.0` |
| Valid document, empty query | `0.0` |
| avg_doc_length = 0.0 | Use default 100.0 |
| All documents empty | Each score = 0.0 |
| Document with 1 million words | Valid finite score |

---

## FIX LOCATIONS

1. `src/filter.rs:323` - Add guard: `let safe_avg = avg_doc_length.max(1.0);`
2. `src/filter.rs:343` - Add denominator check before division
3. `src/main.rs:470-474` - Ensure fallback is applied when `total_words == 0`

---

## TEST CASES

```rust
#[test]
fn test_bm25_zero_avg_length() {
    let score = bm25_score("rust programming", "rust", 0.0);
    assert!(score.is_finite());
    assert!(score >= 0.0);
}

#[test]
fn test_bm25_empty_document() {
    let score = bm25_score("", "rust", 100.0);
    assert_eq!(score, 0.0);
}

#[test]
fn test_bm25_empty_query() {
    let score = bm25_score("rust programming", "", 100.0);
    assert_eq!(score, 0.0);
}
```

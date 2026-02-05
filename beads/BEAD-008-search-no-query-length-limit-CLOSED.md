# BEAD-008: Search Query Has No Length Limit

**Epic**: Input Validation
**Severity**: Low
**Status**: Open

---

## CONTEXT BLOCK

- **File/Function**: `doc_transformer/src/main.rs:435-520` (`run_search`)
- **The Smell**: The search query is validated for emptiness but has no maximum length limit:
  ```rust
  let query = query.trim();
  if query.is_empty() {
      anyhow::bail!("Query cannot be empty");
  }
  ```
  A user could provide a query with millions of characters, causing:
  1. Excessive memory allocation for `query.split_whitespace()`
  2. O(n*m) BM25 scoring where n=query_words, m=document_words
  3. Potential DoS through resource exhaustion

**Tested**: CLI accepted 100,000 character query without error (just slow).

---

## SPECIFICATION BLOCK

### 1. EARS (Easy Approach to Requirements Syntax)

| Trigger | System | Response |
|---------|--------|----------|
| User provides query > 1000 characters | `run_search` | Return error "Query too long (max 1000 chars)" |
| User provides query with > 100 words | `run_search` | Return error "Query has too many terms (max 100)" |
| Query length is within limits | `run_search` | Process normally |

### 2. DbC (Design by Contract)

**Preconditions**:
- `query` parameter is a string from CLI
- `index_dir` points to valid directory with INDEX.json

**Postconditions**:
- Query is trimmed
- Query is non-empty
- Query length <= 1000 characters
- Query word count <= 100
- Search completes in reasonable time (<5 seconds for typical index)

**Invariants**:
- Memory usage is bounded
- Search time scales with index size, not query size

### 3. Schema & Edge Cases

**Query Validation**:
```rust
fn run_search(query: &str, index_dir: &Path, limit: usize) -> Result<()> {
    const MAX_QUERY_LENGTH: usize = 1000;
    const MAX_QUERY_WORDS: usize = 100;

    let query = query.trim();

    if query.is_empty() {
        anyhow::bail!("Query cannot be empty");
    }

    if query.len() > MAX_QUERY_LENGTH {
        anyhow::bail!(
            "Query too long ({} chars, max {})",
            query.len(),
            MAX_QUERY_LENGTH
        );
    }

    let word_count = query.split_whitespace().count();
    if word_count > MAX_QUERY_WORDS {
        anyhow::bail!(
            "Query has too many terms ({} words, max {})",
            word_count,
            MAX_QUERY_WORDS
        );
    }

    // ... rest of search logic
}
```

**Edge Cases**:
| Input | Expected Result |
|-------|-----------------|
| `""` (empty) | Err: "Query cannot be empty" |
| `"   "` (whitespace only) | Err: "Query cannot be empty" |
| 50 character query | Ok, process normally |
| 1000 character query | Ok, at the limit |
| 1001 character query | Err: "Query too long" |
| 100 word query | Ok, at the limit |
| 101 word query | Err: "too many terms" |
| Query with special chars | Ok, passed to BM25 as-is |
| Unicode query | Ok, character count not byte count |

---

## FIX LOCATIONS

1. `src/main.rs:436-440` - Add length and word count validation

---

## TEST CASES

```rust
#[test]
fn test_search_query_too_long() {
    let long_query = "a".repeat(1001);
    let result = run_search(&long_query, Path::new("."), 10);
    assert!(result.is_err());
    assert!(result.unwrap_err().to_string().contains("too long"));
}

#[test]
fn test_search_query_too_many_words() {
    let many_words = (0..101).map(|_| "word").collect::<Vec<_>>().join(" ");
    let result = run_search(&many_words, Path::new("."), 10);
    assert!(result.is_err());
    assert!(result.unwrap_err().to_string().contains("too many terms"));
}

#[test]
fn test_search_query_at_limits() {
    // Should succeed at exactly the limits
    let max_chars = "a".repeat(1000);
    // Note: requires valid INDEX.json for full test
}
```

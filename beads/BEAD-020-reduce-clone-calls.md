# BEAD-020: 100+ Clone Calls - Reduce Allocations

**Epic**: Performance
**Severity**: Low
**Status**: Open

---

## CONTEXT BLOCK

- **Files**: All source files (100+ `.clone()` calls total)
- **The Smell**: Heavy string and data structure cloning throughout the pipeline. While `.clone()` is idiomatic Rust in many cases, 100+ calls suggest poor ownership design and unnecessary allocations. This impacts performance, especially with large document sets.

**Evidence**:
```bash
$ grep -r "\.clone()" doc_transformer/src | wc -l
100  # One hundred clone calls across codebase

$ for file in doc_transformer/src/*.rs; do
    echo "$file: $(grep -c '\.clone()' $file) clones"
done | sort -t: -k2 -nr
index.rs: 48 clones
scrape.rs: 12 clones
graph.rs: 6 clones
main.rs: 8 clones
chunk.rs: 9 clones
... (total 100)

# Examples of unnecessary cloning:
# doc_transformer/src/index.rs:48
let docs = config.documents.clone();  // Could use &Vec<Document>
# doc_transformer/src/graph.rs:15
let mut graph = self.graph.clone();    // Could use Cow or Rc
# doc_transformer/src/scrape.rs:23
let urls = config.urls.clone();        // Could use iter()
```

**Clone Impact Analysis**:

| Data Type | Clone Cost | Frequency | Estimated Impact |
|-----------|-------------|-----------|------------------|
| `String` | O(n) heap allocation | ~60 calls | Medium |
| `Vec<T>` | O(n) heap allocation + T clones | ~25 calls | Medium-High |
| `HashMap<K,V>` | O(n) heap allocation + K,V clones | ~8 calls | Low-Medium |
| `structs` | Sum of field clones | ~7 calls | Low |

**User Impact**:
- Slower indexing (unnecessary allocations)
- Higher memory usage during pipeline
- Poor cache locality (heap allocations everywhere)
- Not actually "pure functional" - just copying
- Performance degrades with document count
- Could be 10-20% faster with better ownership

---

## SPECIFICATION BLOCK

### 1. EARS (Easy Approach to Requirements Syntax)

| Trigger | System | Response |
|---------|--------|----------|
| Clone count > 50 in a file | Code review | Refactor to use references or Cow |
| Clone large collection multiple times | Performance | Pass by reference, copy on write |
| Need both original and modified copy | Design | Use `Cow<T>` or `Rc<T>` |

### 2. DbC (Design by Contract)

**Preconditions**:
- Functionality unchanged
- All tests pass
- No new panics introduced

**Postconditions**:
- Reduce clone count by at least 50%
- Improve indexing performance by 10-20%
- Maintain safety (no unwraps)
- Keep code readable

**Invariants**:
- Only clone when truly necessary
- Use references where possible
- Use Cow for conditional cloning
- No safety regressions

### 3. Schema & Edge Cases

**Clone Reduction Strategies**:

| Pattern | Before | After | Reduction |
|---------|--------|-------|-----------|
| Read-only access | `let docs = config.docs.clone()` | `let docs = &config.docs` | 100% |
| Modify copy | `let mut graph = self.graph.clone()` | `let graph = Cow::Borrowed(&self.graph)` | 0-100% |
| Iterate | `for url in urls.clone()` | `for url in &urls` | 100% |
| Return new struct | `Struct { field: val.clone() }` | `Struct { field: val }` (move) | 100% |
| Optional clone | `opt.as_ref().map(|x| x.clone())` | `opt.as_ref()` | 100% |

**Refactoring Examples**:

**Example 1: Pass by Reference**
```rust
// BEFORE
fn process_documents(config: &Config) -> Result<()> {
    let docs = config.documents.clone();  // Clone Vec<Document>

    for doc in docs {  // Moves cloned vec
        transform_doc(doc)?;
    }
}

// AFTER
fn process_documents(config: &Config) -> Result<()> {
    for doc in &config.documents {  // No clone, borrow
        transform_doc(doc)?;
    }
}
```

**Example 2: Cow for Conditional Mutations**
```rust
// BEFORE
fn maybe_mutate(graph: &Graph) -> Graph {
    let mut new_graph = graph.clone();  // Always clone
    if needs_mutation(&new_graph) {
        mutate(&mut new_graph);
    }
    new_graph
}

// AFTER
use std::borrow::Cow;

fn maybe_mutate(graph: &Graph) -> Cow<Graph> {
    if needs_mutation(graph) {
        let mut new_graph = graph.clone();  // Clone only when needed
        mutate(&mut new_graph);
        Cow::Owned(new_graph)
    } else {
        Cow::Borrowed(graph)  // No clone
    }
}
```

**Example 3: Iterate Instead of Clone**
```rust
// BEFORE
fn find_best_match(urls: Vec<Url>, query: &str) -> Option<Url> {
    for url in urls.clone() {  // Unnecessary clone
        if url.as_str().contains(query) {
            return Some(url);
        }
    }
    None
}

// AFTER
fn find_best_match(urls: &[Url], query: &str) -> Option<Url> {
    for url in urls {  // Borrow
        if url.as_str().contains(query) {
            return Some(url.clone());  // Clone only when returning
        }
    }
    None
}
```

**Priority Files to Refactor**:

| File | Clone Count | Priority | Expected Gain |
|------|-------------|-----------|---------------|
| `index.rs` | 48 | High | 20-30 clones removable |
| `scrape.rs` | 12 | Medium | 6-8 clones removable |
| `graph.rs` | 6 | Medium | 4-5 clones removable |
| `main.rs` | 8 | Low | 4-6 clones removable |

---

## FIX LOCATIONS

1. **`doc_transformer/src/index.rs`** - Remove unnecessary Vec clones
   - Lines 48, 72, 95, etc.: Change `config.documents.clone()` to `&config.documents`
   - Use iterators instead of cloning for loops
   - Apply Cow where conditional mutations exist

2. **`doc_transformer/src/scrape.rs`** - Remove URL and String clones
   - Lines 23, 67, etc.: Change `config.urls.clone()` to `&config.urls`
   - Use `&str` instead of cloning strings
   - Pass by reference to helpers

3. **`doc_transformer/src/graph.rs`** - Use Cow for graph operations
   - Line 15: Change `self.graph.clone()` to `Cow::Borrowed(&self.graph)`
   - Only clone when mutations needed

4. **`doc_transformer/src/main.rs`** - Reduce config clones
   - Lines where config is cloned: Pass by reference instead
   - Use `&Args` instead of cloning

---

## TEST CASES

```rust
#[test]
fn test_indexing_without_unnecessary_clones() {
    // Use instrumentation to count allocations
    let config = TestConfig::large_dataset();

    let start = Instant::now();
    let result = run_index(&config);
    let duration = start.elapsed();

    assert!(result.is_ok());

    // Should be faster with fewer clones
    assert!(duration < baseline_duration * 0.8); // 20% improvement
}

#[test]
fn test_clone_count_reduced() {
    let source_files = glob("doc_transformer/src/*.rs").unwrap();

    let total_clones: usize = source_files
        .filter_map(Result::ok)
        .filter_map(|path| std::fs::read_to_string(path).ok())
        .map(|content| content.matches(".clone()").count())
        .sum();

    // Reduced from 100 to < 50
    assert!(
        total_clones < 50,
        "Still too many clones: {} (target < 50)",
        total_clones
    );
}

#[test]
fn test_no_unnecessary_vec_clones() {
    let code = std::fs::read_to_string("doc_transformer/src/index.rs").unwrap();

    // Look for patterns like: let x = config.docs.clone()
    let unnecessary_clone_regex = Regex::new(r"let\s+\w+\s*=\s*\w+\.documents\(\)\.clone\(\)")?;

    assert!(
        !unnecessary_clone_regex.is_match(&code),
        "Found unnecessary Vec<Document> clone"
    );
}

#[test]
fn test_cow_usage_for_conditional_mutations() {
    let code = std::fs::read_to_string("doc_transformer/src/graph.rs").unwrap();

    // Should use Cow for conditional mutations
    assert!(
        code.contains("Cow::"),
        "graph.rs should use Cow for conditional mutations"
    );
}
```

---

## VERIFICATION

After refactoring:
```bash
$ grep -r "\.clone()" doc_transformer/src | wc -l
42  # Reduced from 100 (58% reduction)

$ cargo test performance
# test_indexing_without_unnecessary_clones ... ok
# test_clone_count_reduced ... ok
# test_no_unnecessary_vec_clones ... ok
# test_result: ok. passed.

$ time ./target/release/doc_transformer index large_docs/ output/
# Before: 1m 15s
# After:  0m 55s  (20% faster)

# Check specific files
$ for file in doc_transformer/src/*.rs; do
    echo "$file: $(grep -c '\.clone()' $file) clones"
done | sort -t: -k2 -nr
index.rs: 18 clones  (was 48)
scrape.rs: 5 clones  (was 12)
graph.rs: 2 clones  (was 6)
main.rs: 4 clones  (was 8)
```

---

## RECOMMENDATION

Focus on `index.rs` first (48 clones → target 18 clones), then other files. Use references instead of cloning for read-only access. Use `Cow<T>` for conditional mutations.

**Why this matters**:
- 10-20% faster indexing
- Lower memory usage
- Better cache locality
- Still idiomatic Rust (references, not clones)
- Aligns with "safe Rust" goal (efficiency)

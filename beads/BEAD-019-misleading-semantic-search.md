# BEAD-019: "Semantic Search" Without Embeddings is Misleading

**Epic**: Documentation Integrity
**Severity**: Medium
**Status**: Open

---

## CONTEXT BLOCK

- **Files**: `README.md`, `AGENTS.md`, marketing copy
- **The Smell**: Project claims "semantic similarity" and "HNSW semantic similarity search" but doesn't use embeddings. It uses token overlap and Jaccard similarity on text, which is fuzzy matching, not semantic understanding. Real semantic search requires vector embeddings (OpenAI, Cohere, SentenceTransformers).

**Evidence**:
```bash
$ grep -r "semantic" README.md
README.md: 🧠 **Semantic similarity** via HNSW approximate nearest neighbor

$ grep -r "semantic" AGENTS.md
AGENTS.md:* Semantic chunking with context prefix (~512 tokens/chunk)

# Actual implementation:
# doc_transformer/src/similarity.rs
pub fn jaccard_similarity(a: &str, b: &str) -> f32 {
    let set_a: HashSet<&str> = a.split_whitespace().collect();
    let set_b: HashSet<&str> = b.split_whitespace().collect();
    // ... Jaccard index calculation
    // This is token overlap, NOT semantic similarity!
}

# doc_transformer/src/index.rs (HNSW usage)
// HNSW is configured with cosine distance on TF-IDF vectors
// Still not true semantic search (no embeddings)
```

**"Semantic Search" Reality Check**:

| Feature | True Semantic Search | This Implementation |
|---------|-------------------|-------------------|
| Embeddings | OpenAI/Cohere/SentenceTransformers | ❌ None |
| Vector space | High-dimensional (768-1536 dim) | ❌ TF-IDF (vocabulary size) |
| Similarity measure | Cosine similarity on embeddings | ⚠️ Jaccard/Tfidf on tokens |
| Understanding context | Word meanings, synonyms | ❌ Token overlap only |
| Example query | "canine" matches "dog" | ❌ Only exact/similar tokens |

**User Impact**:
- Misleading feature claims (expects real semantic search)
- AI agents don't get semantic understanding
- Poor performance on synonym-heavy queries
- Can't find related concepts with different words
- Marketing overpromises, underdelivers

---

## SPECIFICATION BLOCK

### 1. EARS (Easy Approach to Requirements Syntax)

| Trigger | System | Response |
|---------|--------|----------|
| Documentation claims "semantic search" | Review | Verify embeddings exist or update claim |
| User queries with synonyms | Search behavior | Clarify limitations |
| Adding embedding support | Feature flags | Optional, not default |

### 2. DbC (Design by Contract)

**Preconditions**:
- Documentation accurately describes features
- "Semantic" only used if embeddings are used

**Postconditions**:
- Either: Add real embedding support for semantic search
- Or: Update docs to say "fuzzy matching" or "token similarity"
- Clarify HNSW is for performance, not semantics

**Invariants**:
- "Semantic" implies embeddings exist
- Feature claims match implementation
- Marketing doesn't overpromise

### 3. Schema & Edge Cases

**Documentation Updates Required**:

**README.md (Line 16)**:
```markdown
# BEFORE
🧠 **Semantic similarity** via HNSW approximate nearest neighbor

# AFTER
🧠 **Similarity search** via HNSW with TF-IDF vectors
```

**README.md (Line 98)** - Add disclaimer:
```markdown
# ADD THIS
**Note**: Similarity search uses token-based matching (TF-IDF), not semantic embeddings.
For true semantic understanding, consider integrating OpenAI embeddings or similar.
```

**AGENTS.md** - Update description:
```markdown
# BEFORE
* Semantic chunking with context prefix (~512 tokens/chunk)

# AFTER
* Context-aware chunking with context prefix (~512 tokens/chunk)
```

**Search Help Text** (`doc_transformer/src/search.rs`):
```rust
# Add to search command help
/// Search indexed documentation using BM25
///
/// Note: Search uses keyword matching and TF-IDF similarity.
/// For true semantic search, embeddings would be required.
```

**Feature Comparison**:

| Search Type | Example | Result |
|-------------|---------|--------|
| Current (keyword) | "canine" | ❌ No results |
| Current (keyword) | "dog" | ✅ Results |
| Semantic (embeddings) | "canine" | ✅ Results (matches "dog") |
| Semantic (embeddings) | "automobile" | ✅ Results (matches "car") |

---

## FIX LOCATIONS

1. **`README.md`** - Update feature descriptions
   - Line 16: Change "semantic similarity" to "similarity search"
   - Line 98: Add disclaimer about token-based matching
   - Remove "semantic" where embeddings don't exist

2. **`AGENTS.md`** - Update terminology
   - Change "semantic chunking" to "context-aware chunking"
   - Remove "semantic" from search descriptions

3. **`doc_transformer/src/main.rs`** - Update help text
   - Line 1-8: Remove "semantic" from module doc
   - Add note about keyword matching

4. **`doc_transformer/src/search.rs`** - Clarify implementation
   - Add module-level comment: "Keyword + TF-IDF search, not semantic embeddings"

---

## TEST CASES

```rust
// Test to verify search behavior matches documentation

#[test]
fn test_search_synonyms_not_matched() {
    // Current implementation doesn't match synonyms
    let index = create_test_index_with_content("dog, canine, hound");

    let results = search_index(&index, "canine");
    assert!(results.is_empty()); // No embeddings = no synonym matching

    let results = search_index(&index, "dog");
    assert!(!results.is_empty()); // Exact match works
}

#[test]
fn test_documentation_matches_reality() {
    let docs = vec!["README.md", "AGENTS.md"];

    for doc in docs {
        let content = std::fs::read_to_string(doc).unwrap();

        // If "semantic" appears, should mention embeddings
        if content.to_lowercase().contains("semantic") {
            assert!(
                content.to_lowercase().contains("embedding") ||
                content.to_lowercase().contains("vector") ||
                content.to_lowercase().contains("token-based"),
                "{} mentions 'semantic' without clarifying it's token-based",
                doc
            );
        }
    }
}

#[test]
fn test_similarity_is_jaccard_not_cosine() {
    use crate::similarity::jaccard_similarity;

    let text_a = "dog bark";
    let text_b = "canine bark";

    // Jaccard matches "bark" but not synonyms
    let similarity = jaccard_similarity(text_a, text_b);
    assert!(similarity > 0.0 && similarity < 1.0);

    // Not true semantic understanding (should be high for synonyms)
    assert!(similarity < 0.8, "Jaccard similarity should be low for synonyms");
}
```

---

## VERIFICATION

After updates:
```bash
$ grep -r "semantic" README.md | grep -i "search\|similarity"
# (no "semantic search" or "semantic similarity" - removed)

$ grep -r "token-based\|tf-idf" README.md
README.md: 🧠 **Similarity search** via HNSW with TF-IDF vectors
README.md:**Note**: Similarity search uses token-based matching (TF-IDF)
# ✅ Accurate description

$ grep -r "semantic" AGENTS.md
AGENTS.md:* Context-aware chunking with context prefix
# ✅ Removed misleading claims

$ cargo test search_behavior
# test_search_synonyms_not_matched ... ok
# test_documentation_matches_reality ... ok
# test_result: ok. passed.

# User query behavior documented accurately
$ ./target/release/doc_transformer search "canine" --index-dir ./test_index
# Results: 0 (docs only have "dog")
# (Expected behavior for keyword search, not semantic)
```

---

## OPTIONAL FUTURE: Real Semantic Search

If you want true semantic search later, add:
1. **Embedding generation** (OpenAI API, SentenceTransformers local)
2. **Vector database** (Qdrant, Milvus, or extend HNSW with embeddings)
3. **Hybrid search** (BM25 + semantic similarity fusion)

Example architecture:
```rust
// Future: semantic_search.rs
pub async fn generate_embeddings(text: &str) -> Result<Vec<f32>> {
    // Call OpenAI embeddings API
}

pub fn semantic_search(query: &str, embeddings: &Vec<Vec<f32>>) -> Vec<SearchResult> {
    // Use cosine similarity on embeddings
}
```

---

## RECOMMENDATION

Update documentation to say "similarity search" or "fuzzy matching" instead of "semantic search." Add a disclaimer that current implementation uses token-based matching (TF-IDF), not embeddings.

**Why this matters**:
- Sets correct user expectations
- Avoids confusion when synonyms don't match
- Honest about current capabilities
- Keeps door open for future embedding support
- "Similarity search" is still valuable - just not "semantic"

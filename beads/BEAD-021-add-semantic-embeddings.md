# BEAD-021: Add Real Semantic Search With Embeddings

**Epic**: Feature Enhancement
**Severity**: Low
**Status**: Open

---

## CONTEXT BLOCK

- **Files**: New module `doc_transformer/src/embeddings.rs`, integration with search
- **The Smell**: Current "similarity search" uses TF-IDF and Jaccard similarity on tokens. This is fuzzy matching, not semantic understanding. Users expect "semantic search" to understand synonyms, context, and word meanings (e.g., "canine" matches "dog").

**Evidence**:
```bash
$ ./target/release/doc_transformer search "canine" --index-dir ./index
# Results: 0 (but docs contain "dog")

$ ./target/release/doc_transformer search "dog" --index-dir ./index
# Results: 3 (exact match works)

# Current implementation:
# doc_transformer/src/similarity.rs
pub fn jaccard_similarity(a: &str, b: &str) -> f32 {
    let set_a: HashSet<&str> = a.split_whitespace().collect();
    let set_b: HashSet<&str> = b.split_whitespace().collect();
    // Token overlap, not semantic understanding
}

# What users expect:
"canine" ≈ "dog" (semantic similarity)
"automobile" ≈ "car" (semantic similarity)
"purchase" ≈ "buy" (semantic similarity)
```

**Current vs True Semantic Search**:

| Query | Current (TF-IDF) | Semantic (Embeddings) |
|-------|-------------------|----------------------|
| "canine" | ❌ No match | ✅ Matches "dog" |
| "automobile" | ❌ No match | ✅ Matches "car" |
| "purchase" | ❌ No match | ✅ Matches "buy" |
| "fast" | ❌ No match | ✅ Matches "quick" |
| "create" | ❌ No match | ✅ Matches "make" |

**User Impact**:
- Poor search experience for concept queries
- Can't find related content with different wording
- "Semantic" claim is misleading
- AI agents miss relevant chunks
- Not competitive with modern search tools

---

## SPECIFICATION BLOCK

### 1. EARS (Easy Approach to Requirements Syntax)

| Trigger | System | Response |
|---------|--------|----------|
| User searches with synonyms | Search | Return semantically similar results |
| Generating index | Pipeline | Create embeddings for all chunks |
| Embedding API down | Fallback | Use TF-IDF search instead |
| --semantic flag provided | CLI | Enable embedding search |

### 2. DbC (Design by Contract)

**Preconditions**:
- Embedding model available (OpenAI, SentenceTransformers, or local)
- API key configured if using cloud service
- Sufficient compute for embedding generation

**Postconditions**:
- Query embeddings match semantically similar chunks
- Synonyms and related concepts are found
- Fallback to TF-IDF if embeddings unavailable
- Hybrid scoring (BM25 + semantic) for best results

**Invariants**:
- Embeddings generated once per chunk (cached)
- Query embedding generated per search
- Cosine similarity used for embedding comparison
- Optional feature (can be disabled)

### 3. Schema & Edge Cases

**Embedding Model Options**:

| Model | Provider | Dimensions | Cost | Latency |
|-------|----------|-------------|------|----------|
| text-embedding-3-small | OpenAI | 1536 | $0.02/1M tokens | ~200ms |
| text-embedding-3-large | OpenAI | 3072 | $0.13/1M tokens | ~500ms |
| all-MiniLM-L6-v2 | SentenceTransformers (local) | 384 | Free | ~50ms |
| bge-small-en-v1.5 | SentenceTransformers (local) | 384 | Free | ~40ms |

**Proposed Architecture**:

```rust
// doc_transformer/src/embeddings.rs (NEW MODULE)
use anyhow::Result;

#[derive(Clone, Debug)]
pub enum EmbeddingModel {
    OpenAI { api_key: String, model: String },
    Local { model_name: String },  // SentenceTransformers
}

pub struct EmbeddingGenerator {
    model: EmbeddingModel,
    cache: LruCache<String, Vec<f32>>,
}

impl EmbeddingGenerator {
    pub async fn generate(&self, text: &str) -> Result<Vec<f32>> {
        // Generate embedding with caching
    }
}

#[derive(Clone, Debug)]
pub struct EmbeddingSearch {
    pub chunks: Vec<ChunkWithEmbedding>,
    pub index: HNSWIndex,  // Use existing HNSW
}

impl EmbeddingSearch {
    pub async fn search(&self, query: &str, limit: usize) -> Vec<SearchResult> {
        // Generate query embedding
        let query_embedding = self.generate_embedding(query).await?;

        // Find nearest neighbors using HNSW
        self.index.search(&query_embedding, limit)
    }
}
```

**Configuration Updates**:

```toml
# doc_transformer/Cargo.toml (NEW DEPENDENCIES)
[dependencies]
# Optional embedding dependencies
openai = { version = "0.14", optional = true }
candle-transformers = { version = "0.6", optional = true }  # Local embeddings
candle-nn = { version = "0.6", optional = true }
```

```bash
# CLI Arguments (NEW FLAGS)
$ ./target/release/doc_transformer index ./docs --output ./out \
    --embeddings model=openai \
    --openai-api-key $OPENAI_API_KEY

$ ./target/release/doc_transformer search "canine" \
    --index-dir ./out \
    --semantic  # Use embeddings instead of TF-IDF
```

**Hybrid Search Scoring**:

```rust
pub fn hybrid_score(bm25_score: f32, semantic_score: f32) -> f32 {
    // Combine BM25 (keyword relevance) with semantic (concept similarity)
    let alpha = 0.7;  // Weight for BM25
    let beta = 0.3;     // Weight for semantic

    alpha * bm25_score + beta * semantic_score
}
```

**Fallback Strategy**:

```rust
pub async fn search_with_fallback(query: &str, config: &SearchConfig) -> Vec<SearchResult> {
    if config.use_semantic {
        match generate_embeddings(query).await {
            Ok(embedding) => semantic_search(&embedding).await,
            Err(e) => {
                eprintln!("Embeddings failed: {}, falling back to TF-IDF", e);
                bm25_search(query).await
            }
        }
    } else {
        bm25_search(query).await
    }
}
```

---

## FIX LOCATIONS

1. **Create `doc_transformer/src/embeddings.rs`** (NEW MODULE, ~300 lines)
   - `EmbeddingModel` enum (OpenAI, Local)
   - `EmbeddingGenerator` struct with caching
   - `generate_embedding()` function
   - `cosine_similarity()` helper

2. **`doc_transformer/src/index.rs`** - Generate embeddings during indexing
   - Add optional embedding generation step
   - Store embeddings in INDEX.json or separate file
   - Update PipelineConfig with embedding options

3. **`doc_transformer/src/search.rs`** - Add semantic search mode
   - `search_semantic()` function (uses embeddings)
   - `search_hybrid()` function (BM25 + semantic)
   - Update CLI flags for `--semantic`

4. **`doc_transformer/src/main.rs`** - Add embedding CLI flags
   - `--embeddings model=openai|local`
   - `--openai-api-key KEY`
   - `--semantic` flag for search

5. **`doc_transformer/Cargo.toml`** - Add optional dependencies
   - `openai` (optional feature)
   - `candle-transformers`, `candle-nn` (optional feature for local embeddings)

---

## TEST CASES

```rust
#[test]
fn test_semantic_search_finds_synonyms() {
    let mut generator = EmbeddingGenerator::local("all-MiniLM-L6-v2").unwrap();

    let dog_embedding = generator.generate("dog").await.unwrap();
    let canine_embedding = generator.generate("canine").await.unwrap();

    // Semantic similarity should be high for synonyms
    let similarity = cosine_similarity(&dog_embedding, &canine_embedding);
    assert!(similarity > 0.8, "Synonyms should be semantically similar");
}

#[test]
fn test_keyword_search_no_match_synonyms() {
    // TF-IDF doesn't match synonyms
    let index = create_bm25_index(&["dog", "cat", "bird"]);

    let results = bm25_search(&index, "canine");
    assert!(results.is_empty(), "TF-IDF shouldn't match synonyms");
}

#[test]
fn test_hybrid_search() {
    let config = SearchConfig {
        use_hybrid: true,
        bm25_weight: 0.7,
        semantic_weight: 0.3,
        ..Default::default()
    };

    let results = hybrid_search(&config, "canine").await;
    assert!(!results.is_empty(), "Hybrid search should find results");
}

#[test]
fn test_fallback_to_bm25() {
    let config = SearchConfig {
        use_semantic: true,
        model: EmbeddingModel::OpenAI { api_key: "invalid".into(), .. },
    };

    let results = search_with_fallback(&config, "query").await;
    // Should fallback to BM25 instead of erroring
    assert!(!results.is_empty());
}

#[test]
fn test_embedding_caching() {
    let mut generator = EmbeddingGenerator::local("test-model").unwrap();

    let text = "test content";
    let _ = generator.generate(text).await;

    // Second call should be cached (much faster)
    let start = Instant::now();
    let _ = generator.generate(text).await;
    let cached_duration = start.elapsed();

    assert!(cached_duration < Duration::from_millis(10));
}
```

---

## VERIFICATION

After implementation:
```bash
# Index with embeddings
$ ./target/release/doc_transformer index ./docs --output ./out \
    --embeddings model=local \
    --model-name all-MiniLM-L6-v2

# [STEP 1] DISCOVER: Found 36 files
# [STEP 8] GENERATE EMBEDDINGS: 1202/1202 chunks (384-dim vectors)
# [STEP 9] BUILD HNSW INDEX: Built index with 1202 embeddings
# ======================================================================
# COMPLETE

# Semantic search works
$ ./target/release/doc_transformer search "canine" \
    --index-dir ./out \
    --semantic

# Results:
# 1. [ops] dog behavior (semantic score: 0.92)
# 2. [ops] canine training guide (semantic score: 0.88)
# ======================================================================
# Found 2 semantically similar results

# Hybrid search combines both
$ ./target/release/doc_transformer search "dog" \
    --index-dir ./out \
    --hybrid

# Results:
# 1. [ops] dog behavior (BM25: 3.5, semantic: 0.95, hybrid: 2.4)
# 2. [ops] canine training (BM25: 0.0, semantic: 0.88, hybrid: 0.26)
# ======================================================================
# Found 2 results (BM25 + semantic)

# Fallback works
$ ./target/release/doc_transformer search "canine" \
    --index-dir ./out \
    --semantic  # But embedding model unavailable

# Fallback: Using TF-IDF search (embeddings unavailable)
# Results: 0
```

---

## OPTIONAL: Performance Benchmarks

```bash
$ cargo bench semantic_search
# running 4 tests
# test bm25_search                    ... bench:  12,345 ns/iter (+/- 200)
# test semantic_search (local)        ... bench:  45,678 ns/iter (+/- 500)
# test semantic_search (openai)       ... bench: 234,567 ns/iter (+/- 5000)
# test hybrid_search                  ... bench:  67,890 ns/iter (+/- 700)
```

---

## RECOMMENDATION

Implement as **optional feature** (not default). Start with local SentenceTransformers models (free, fast) and add OpenAI as alternative. Use hybrid scoring for best results.

**Why this matters**:
- True semantic understanding (synonyms, concepts)
- Better search quality for AI agents
- Competitive with modern search tools
- "Semantic" claim becomes accurate
- Optional (not forced on users)

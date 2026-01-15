# v8.0 Vector Embeddings Design

**Status:** Design Document (Pre-Implementation)
**Target:** v8.0 release
**Priority:** P0 (Core Feature)
**Date:** 2026-01-15

---

## Overview

Replace keyword-based semantic search with true vector embeddings for superior semantic similarity matching.

### Current State (v6.0/v7.0)

- **Semantic Search:** Text-based fallback using keyword matching
- **Related Documents:** Jaccard similarity on tags
- **Limitations:**
  - No understanding of semantic meaning
  - Poor performance on paraphrased queries
  - Limited to exact keyword matches

### Target State (v8.0)

- **Semantic Search:** Vector embeddings with cosine similarity
- **Related Documents:** True semantic similarity via embeddings
- **Benefits:**
  - Understanding of concept similarity ("API" ≈ "endpoint" ≈ "interface")
  - Robust to paraphrasing and synonyms
  - Better retrieval quality (35%+ improvement expected)

---

## Architecture Decision: Embedding Model

### Requirements

1. **Performance:** < 50ms latency for document chunk embedding
2. **Quality:** State-of-the-art semantic understanding
3. **Size:** Reasonable model size (< 500MB)
4. **License:** Permissive for commercial use
5. **Rust Support:** Available via ONNX or native Rust

### Candidates Evaluated

#### 1. sentence-transformers (Python)
**Model:** `all-MiniLM-L6-v2`
- **Dimensions:** 384
- **Speed:** ~20ms/chunk (GPU), ~80ms/chunk (CPU)
- **Quality:** Good for general text
- **License:** Apache 2.0 ✅
- **Rust:** Via ONNX Runtime ✅

**Pros:**
- Industry standard
- Excellent quality/speed tradeoff
- Well-documented
- ONNX export available

**Cons:**
- Requires Python bridge or ONNX conversion
- Larger than specialized models

#### 2. FastEmbed (Rust-native)
**Model:** `BAAI/bge-small-en-v1.5`
- **Dimensions:** 384
- **Speed:** ~30ms/chunk (CPU)
- **Quality:** State-of-the-art for retrieval
- **License:** MIT ✅
- **Rust:** Native via `fastembed-rs` ✅

**Pros:**
- Pure Rust implementation
- No Python dependency
- Optimized for retrieval tasks
- Easy integration

**Cons:**
- Newer library (less battle-tested)
- Fewer model options currently

#### 3. OpenAI Embeddings (API)
**Model:** `text-embedding-3-small`
- **Dimensions:** 1536
- **Speed:** ~100-200ms (network latency)
- **Quality:** Excellent
- **License:** API terms
- **Rust:** Via HTTP client

**Pros:**
- Highest quality
- No local compute needed
- Automatic updates

**Cons:**
- Network dependency (not offline)
- Cost per API call
- Latency variability
- Privacy concerns (sends data to OpenAI)

### Recommendation: FastEmbed (Rust-native)

**Choice:** `fastembed-rs` with `BAAI/bge-small-en-v1.5`

**Rationale:**
1. **Pure Rust:** Aligns with project philosophy (zero Python deps)
2. **Offline:** No network dependency, works in airgapped environments
3. **Performance:** Excellent CPU performance (~30ms/chunk)
4. **Quality:** State-of-the-art for retrieval (outperforms MiniLM)
5. **Licensing:** MIT license, fully permissive
6. **Integration:** Simple API, drop-in replacement

**Dependencies:**
```toml
[dependencies]
fastembed = "3.0"  # Rust-native embeddings
ndarray = "0.15"   # Vector operations
```

---

## Architecture Decision: Vector Database

### Requirements

1. **Performance:** < 10ms query latency for similarity search
2. **Scale:** Support 100K+ vectors efficiently
3. **Rust Support:** Native Rust client or stable bindings
4. **Features:** Cosine similarity, filtering, batch operations
5. **Deployment:** Embeddable or simple server

### Candidates Evaluated

#### 1. Qdrant
**Type:** Dedicated vector database server

**Pros:**
- Purpose-built for vectors
- Excellent performance (HNSW index)
- Rich filtering capabilities
- gRPC + HTTP APIs
- Rust client available

**Cons:**
- Requires separate server process
- Operational complexity
- Overkill for small datasets

**Fit:** Good for production, but complex for standalone tool

#### 2. Milvus
**Type:** Cloud-native vector database

**Pros:**
- Highly scalable
- Multiple index types (IVF, HNSW)
- Strong community

**Cons:**
- Complex deployment (multiple services)
- Heavy resource requirements
- No embedded mode
- Limited Rust support

**Fit:** Excellent for scale, poor for standalone use

#### 3. HNSW-rs (In-Memory)
**Type:** Pure Rust HNSW implementation

**Pros:**
- Already in use (src/similarity.rs)
- Pure Rust, zero deps
- Embeddable
- Fast (O(log n) search)
- No server required

**Cons:**
- In-memory only (not persistent)
- Manual serialization needed
- No built-in filtering

**Fit:** Perfect for current architecture

#### 4. LanceDB
**Type:** Embedded vector database

**Pros:**
- Embeddable (no server)
- Disk-backed (persistent)
- Rust support
- Arrow-based (efficient)

**Cons:**
- Newer project (less mature)
- Larger binary size
- More complex than HNSW

**Fit:** Good middle ground

### Recommendation: Enhanced HNSW-rs (v8.0) → LanceDB (v8.1)

**Phase 1 (v8.0):** Enhance existing HNSW implementation
- Add vector storage alongside current similarity index
- Serialize/deserialize to INDEX.json
- Minimal changes to architecture
- Drop-in replacement for semantic_search

**Phase 2 (v8.1):** Migrate to LanceDB (if needed)
- Evaluate performance with real-world data
- If HNSW serialization becomes bottleneck, migrate to LanceDB
- Provides upgrade path without blocking v8.0

**Rationale:**
1. **Pragmatic:** Reuse existing HNSW infrastructure
2. **Fast:** No new dependencies for v8.0
3. **Incremental:** Upgrade path defined but not required
4. **Simple:** Maintain standalone binary philosophy

---

## Implementation Plan

### Phase 1: Core Infrastructure (Week 1-2)

**File:** `src/embeddings.rs` (new)

```rust
use fastembed::{EmbeddingModel, InitOptions, TextEmbedding};
use anyhow::Result;

/// Embedding generator using fastembed
pub struct EmbeddingGenerator {
    model: TextEmbedding,
}

impl EmbeddingGenerator {
    /// Initialize with default model (bge-small-en-v1.5)
    pub fn new() -> Result<Self> {
        let model = TextEmbedding::try_new(InitOptions {
            model_name: fastembed::EmbeddingModel::BGESmallENV15,
            show_download_message: false,
            ..Default::default()
        })?;

        Ok(Self { model })
    }

    /// Generate embeddings for a batch of texts
    pub fn embed_batch(&self, texts: &[String]) -> Result<Vec<Vec<f32>>> {
        let embeddings = self.model.embed(texts.to_vec(), None)?;
        Ok(embeddings)
    }

    /// Generate embedding for a single text
    pub fn embed(&self, text: &str) -> Result<Vec<f32>> {
        let result = self.embed_batch(&[text.to_string()])?;
        result.into_iter().next()
            .ok_or_else(|| anyhow::anyhow!("No embedding generated"))
    }
}
```

### Phase 2: Integration with Chunking (Week 2-3)

**Modify:** `src/chunk.rs`

```rust
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Chunk {
    pub chunk_id: String,
    pub content: String,
    pub embedding: Option<Vec<f32>>,  // NEW: Vector embedding
    // ... existing fields
}

/// Generate chunks with embeddings
pub fn chunk_with_embeddings(
    doc: &DocumentAnalysis,
    embedding_gen: &EmbeddingGenerator,
) -> Result<Vec<Chunk>> {
    let chunks = chunk_document(doc)?;  // Existing chunking

    // Generate embeddings for all chunks
    let texts: Vec<String> = chunks.iter()
        .map(|c| c.content.clone())
        .collect();

    let embeddings = embedding_gen.embed_batch(&texts)?;

    // Attach embeddings to chunks
    let chunks_with_embeddings = chunks.into_iter()
        .zip(embeddings.into_iter())
        .map(|(mut chunk, embedding)| {
            chunk.embedding = Some(embedding);
            chunk
        })
        .collect();

    Ok(chunks_with_embeddings)
}
```

### Phase 3: Vector Search (Week 3-4)

**Modify:** `src/similarity.rs`

```rust
use ndarray::Array1;

/// Cosine similarity between two vectors
fn cosine_similarity(a: &[f32], b: &[f32]) -> f32 {
    let a_arr = Array1::from_vec(a.to_vec());
    let b_arr = Array1::from_vec(b.to_vec());

    let dot_product = a_arr.dot(&b_arr);
    let norm_a = a_arr.mapv(|x| x * x).sum().sqrt();
    let norm_b = b_arr.mapv(|x| x * x).sum().sqrt();

    if norm_a == 0.0 || norm_b == 0.0 {
        return 0.0;
    }

    dot_product / (norm_a * norm_b)
}

/// Vector-based semantic search
pub fn semantic_search_vectors(
    query: &str,
    chunks: &[Chunk],
    embedding_gen: &EmbeddingGenerator,
    limit: usize,
) -> Result<Vec<(Chunk, f32)>> {
    // Embed query
    let query_embedding = embedding_gen.embed(query)?;

    // Calculate similarities
    let mut similarities: Vec<(Chunk, f32)> = chunks.iter()
        .filter_map(|chunk| {
            chunk.embedding.as_ref().map(|emb| {
                let score = cosine_similarity(&query_embedding, emb);
                (chunk.clone(), score)
            })
        })
        .collect();

    // Sort by similarity (descending)
    similarities.sort_by(|a, b| b.1.partial_cmp(&a.1).unwrap_or(std::cmp::Ordering::Equal));

    // Return top-k
    Ok(similarities.into_iter().take(limit).collect())
}
```

### Phase 4: MCP Integration (Week 4-5)

**Modify:** `src/bin/mcp_server.rs`

```rust
/// Global embedding generator (lazy init)
static EMBEDDING_GEN: OnceLock<EmbeddingGenerator> = OnceLock::new();

fn get_embedding_generator() -> Result<&'static EmbeddingGenerator> {
    EMBEDDING_GEN.get_or_try_init(|| EmbeddingGenerator::new())
        .map_err(|e| McpError::InvalidIndex(format!("Failed to init embeddings: {}", e)))
}

/// Enhanced semantic search using vectors
fn semantic_search(
    query: &str,
    limit: usize,
    threshold: Option<f32>,
    chunks: &[ChunkMetadata],
) -> Result<Value, McpError> {
    let emb_gen = get_embedding_generator()?;

    let results = semantic_search_vectors(query, chunks, emb_gen, limit)
        .map_err(|e| McpError::SearchFailed(e.to_string()))?;

    // Apply threshold if provided
    let filtered: Vec<_> = if let Some(thresh) = threshold {
        results.into_iter()
            .filter(|(_, score)| *score >= thresh)
            .collect()
    } else {
        results
    };

    Ok(json!({
        "results": filtered.iter().map(|(chunk, score)| {
            json!({
                "chunk_id": chunk.chunk_id,
                "doc_id": chunk.doc_id,
                "score": score,
                "summary": chunk.summary,
            })
        }).collect::<Vec<_>>()
    }))
}
```

### Phase 5: Serialization & Caching (Week 5-6)

**Modify:** `src/index.rs`

```rust
/// Chunk metadata with optional vector embedding
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct ChunkMetadata {
    pub chunk_id: String,
    pub doc_id: String,
    pub summary: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub embedding: Option<Vec<f32>>,  // NEW: Persisted to INDEX.json
    // ... existing fields
}

/// Optimization: Compress embeddings (f32 → f16) to reduce INDEX.json size
#[serde(serialize_with = "compress_embedding")]
#[serde(deserialize_with = "decompress_embedding")]
pub embedding_compressed: Option<Vec<u16>>,  // f16 as u16
```

**INDEX.json Size Impact:**
- Before: ~500 KB for 1000 chunks
- After (f32): ~2 MB (+4x, 384 dims * 4 bytes * 1000)
- After (f16): ~1 MB (+2x, 384 dims * 2 bytes * 1000)

**Mitigation:** Optional compression, external embedding file

---

## Performance Targets

### Embedding Generation

| Operation | Target | Notes |
|-----------|--------|-------|
| Single chunk | < 50ms | CPU (single-threaded) |
| Batch (100 chunks) | < 2s | Parallelizable |
| Full index (1000 chunks) | < 20s | One-time cost |

### Vector Search

| Operation | Target | Notes |
|-----------|--------|-------|
| Query embedding | < 50ms | Same as chunk embedding |
| Similarity search | < 10ms | HNSW index (1000 chunks) |
| End-to-end query | < 60ms | Embedding + search |

### Memory Footprint

| Component | Size | Notes |
|-----------|------|-------|
| Embedding model | ~100 MB | Loaded at startup |
| Vector index (1000 chunks) | ~1.5 MB | 384 dims * 4 bytes * 1000 |
| Total overhead | ~105 MB | Acceptable for server use |

---

## Migration Strategy

### Backward Compatibility

**v8.0 must support:**
1. **Old indexes:** Load INDEX.json without embeddings
2. **Gradual migration:** Generate embeddings on-demand
3. **Feature flag:** `--no-embeddings` to disable vector search

**INDEX.json versioning:**
```json
{
  "version": "8.0",
  "embeddings_enabled": true,
  "embedding_model": "bge-small-en-v1.5",
  "embedding_dims": 384,
  "chunks": [...]
}
```

### Upgrade Path

**From v7.0 → v8.0:**
1. Run `doc_transformer regenerate-embeddings <index_dir>`
2. Embeds all chunks, updates INDEX.json
3. Preserves all existing metadata
4. ~20s for 1000 chunks

**Rollback:**
- INDEX.json with embeddings readable by v7.0 (ignored fields)
- No data loss

---

## Testing Strategy

### Unit Tests

```rust
#[test]
fn test_embedding_generation() {
    let gen = EmbeddingGenerator::new().unwrap();
    let emb = gen.embed("test document").unwrap();
    assert_eq!(emb.len(), 384);  // BGE-small dimension
}

#[test]
fn test_cosine_similarity() {
    let a = vec![1.0, 0.0, 0.0];
    let b = vec![1.0, 0.0, 0.0];
    assert_eq!(cosine_similarity(&a, &b), 1.0);

    let c = vec![0.0, 1.0, 0.0];
    assert_eq!(cosine_similarity(&a, &c), 0.0);
}

#[test]
fn test_vector_search_relevance() {
    // Create chunks with known semantic similarity
    let chunks = vec![
        Chunk { content: "Rust programming language", ... },
        Chunk { content: "Python scripting", ... },
        Chunk { content: "Rust systems programming", ... },
    ];

    let results = semantic_search_vectors("Rust development", &chunks, &gen, 2).unwrap();

    // Rust chunks should rank higher than Python
    assert_eq!(results[0].content.contains("Rust"), true);
}
```

### Integration Tests

- Compare keyword vs vector search quality
- Measure retrieval accuracy (precision@k)
- Benchmark query latency
- Test with real documentation corpus

### Quality Metrics

**Expected Improvements (v8.0 vs v7.0):**
- **Precision@5:** 65% → 85% (+20%)
- **Recall@10:** 50% → 75% (+25%)
- **Query Latency:** 5ms → 60ms (acceptable tradeoff)
- **Paraphrase Robustness:** 30% → 80% (+50%)

---

## Dependencies & Licensing

### New Dependencies

```toml
[dependencies]
fastembed = "3.0"              # MIT License ✅
ndarray = "0.15"               # MIT/Apache-2.0 ✅
half = "2.3"                   # MIT/Apache-2.0 ✅ (for f16 compression)
```

### Model Licenses

- **BAAI/bge-small-en-v1.5:** MIT License ✅
- No licensing restrictions for commercial use

### Binary Size Impact

- **Before:** ~15 MB (release build)
- **After:** ~120 MB (+model weights)
- **Mitigation:** External model download, optional feature flag

---

## Risks & Mitigations

### Risk 1: Model Size

**Issue:** 100MB model increases binary size significantly

**Mitigation:**
- Download model on first run (similar to rustc components)
- Store in `~/.cache/doc_transformer/models/`
- Provide `--model-path` flag for custom locations

### Risk 2: Performance Regression

**Issue:** Embedding generation adds latency to indexing

**Mitigation:**
- Parallel batch embedding (leverage all CPU cores)
- Optional: Disable embeddings for small doc sets
- Cache embeddings aggressively

### Risk 3: Quality Concerns

**Issue:** Embeddings may not improve retrieval for technical docs

**Mitigation:**
- A/B test with real queries
- Combine with keyword search (hybrid approach)
- Fine-tune model on technical documentation (v8.1)

---

## Future Enhancements (v8.1+)

### 1. Hybrid Search

Combine keyword + vector search:
- RRF (Reciprocal Rank Fusion)
- Weighted scores: 0.7 * vector + 0.3 * keyword
- Adaptive weighting based on query type

### 2. Model Fine-Tuning

Train custom model on technical docs:
- Collect labeled query-document pairs
- Fine-tune bge-small on domain data
- Expected improvement: +10-15% accuracy

### 3. Multi-Vector Representations

Generate multiple embeddings per chunk:
- Code examples → code embedding
- Natural language → text embedding
- Combine via late interaction (ColBERT-style)

### 4. Incremental Embedding

Only embed new/changed chunks:
- Hash-based change detection
- Reuse existing embeddings
- 10x faster incremental updates

---

## Success Criteria

### v8.0 Launch

- [ ] fastembed integration complete
- [ ] Vector search 20% more accurate than keyword
- [ ] < 60ms query latency (embedding + search)
- [ ] Backward compatible with v7.0 indexes
- [ ] All 557+ tests passing

### v8.1 Goals

- [ ] Hybrid search (keyword + vector)
- [ ] Fine-tuned model for technical docs
- [ ] < 30ms query latency (optimizations)
- [ ] Published to crates.io with embedding support

---

## References

- **fastembed-rs:** https://github.com/Anush008/fastembed-rs
- **BAAI/bge embeddings:** https://github.com/FlagOpen/FlagEmbedding
- **HNSW algorithm:** https://arxiv.org/abs/1603.09320
- **Retrieval benchmarks:** https://github.com/beir-cellar/beir

---

## Appendix: Code Snippets

### Example: End-to-End Vector Search

```rust
// 1. Initialize embedding generator
let emb_gen = EmbeddingGenerator::new()?;

// 2. Chunk document with embeddings
let chunks = chunk_with_embeddings(&doc_analysis, &emb_gen)?;

// 3. Build vector index
let index = build_hnsw_index(&chunks)?;

// 4. Query
let query = "How do I configure logging?";
let results = semantic_search_vectors(query, &chunks, &emb_gen, 10)?;

// 5. Results
for (chunk, score) in results {
    println!("{}: {:.3} - {}", chunk.chunk_id, score, chunk.summary);
}
```

**Output:**
```
chunk-42: 0.873 - Logging configuration via YAML
chunk-15: 0.842 - Log level settings and rotation
chunk-89: 0.791 - Structured logging with JSON format
...
```

---

**Status:** Ready for implementation in v8.0
**Next Steps:** Prototype fastembed integration, benchmark on test corpus

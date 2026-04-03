# Contract Specification

```
bead_id: cdocs-dji
bead_title: action: capture transform artifacts by source path and reuse archived transforms
phase: state-1-contract
updated_at: 2026-04-02T14:00:00Z
```

## Context

- **Feature:** Capture the output of the transform stage as source-path-keyed state artifacts
  persisted via the existing `DocCache` (redb `TRANSFORM_TABLE`). When `run_index` reaches
  STEP 4 (TRANSFORM), the system shall retain a deterministic mapping from each source path
  to its fully-transformed markdown output. On subsequent runs, unchanged documents shall be
  served from cache instead of re-transformed.
- **Domain terms:**
  - **TransformArtifact** -- the serializable unit of work produced by the transform stage:
    the final frontmatter-wrapped markdown string, keyed by a deterministic cache key derived
    from `(source_path, content_hash, link_map_fingerprint)`.
  - **Source Path** -- the `Analysis::source_path` string that uniquely identifies a document
    within a single indexing run (e.g. `"concepts/architecture.md"`).
  - **Content Hash** -- `SHA-256(file_bytes)`, computed via `ContentHash::compute`.
  - **Link Map Fingerprint** -- a deterministic hash of the `HashMap<String, IdMapping>` so
    that changes to ID assignments invalidate the cached transform.
  - **Cache Key** -- `SHA-256(source_path_bytes + content_hash_bytes + link_map_fingerprint_bytes)`,
    computed via `composite_hash`.
- **Assumptions:**
  1. `DocCache` with `CacheType::Transform` and the `TRANSFORM_TABLE` table definition already
     exist and are production-hardened.
  2. `transform_all` currently writes files to disk but does NOT persist artifacts to cache.
  3. The pipeline in `run_index` runs sequentially: discover -> analyze -> assign -> transform
     -> chunk -> validate -> index.
  4. `Analysis::content` (`Arc<str>`) holds the original (pre-transform) markdown source.
  5. `IdMapping` is `Serialize + Deserialize` (serde derives present).
  6. The transform stage is the single most expensive CPU-bound phase (rayon-parallel AST
     operations), making it the highest-value cache target.
- **Open questions:**
  - Should broken-link warnings be included in the cached artifact, or recomputed on load?
    **Resolution for this contract:** Broken links are NOT part of the artifact. They are
    informational warnings recomputed only when a transform actually executes. Cached artifacts
    carry only the final markdown content.
  - Should the contract cover incremental/partial runs (e.g., only new files)? **Resolution:**
    Yes. The contract must support mixed runs where some source paths are cache hits and
    others require fresh computation.

## Domain Types

### `TransformArtifact`

```rust
/// A single persisted transform output, keyed by source path.
///
/// This is the value stored in the `TRANSFORM_TABLE` redb table.
/// The cache key is `composite_hash(&[source_path_bytes, content_hash_bytes, link_map_fp_bytes])`.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct TransformArtifact {
    /// The source path this artifact was produced from (e.g. "concepts/architecture.md").
    pub source_path: String,
    /// SHA-256 of the original file bytes at the time of transformation.
    pub content_hash: ContentHash,
    /// SHA-256 fingerprint of the link_map used during transformation.
    pub link_map_fingerprint: ContentHash,
    /// The fully-transformed markdown output (frontmatter + content).
    pub transformed_markdown: String,
}
```

### `TransformArtifactKey`

```rust
/// Deterministic cache key for a transform artifact.
///
/// Computed as `composite_hash(&[source_path_bytes, content_hash_bytes, link_map_fp_bytes])`.
/// This is the `&[u8]` key used in `DocCache::put_transform` / `DocCache::get_transform`.
///
/// Construction is infallible given valid inputs -- the key is a pure function of its parts.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct TransformArtifactKey(Vec<u8>);

impl TransformArtifactKey {
    /// Compute the artifact key from its constituent parts.
    ///
    /// # Preconditions
    /// - `source_path` is non-empty and valid UTF-8.
    /// - `content_hash` is the SHA-256 of the original file bytes.
    /// - `link_map_fingerprint` is the SHA-256 of the serialized link_map.
    ///
    /// # Postconditions
    /// - The returned key is exactly 32 bytes (SHA-256 output).
    /// - Deterministic: identical inputs always produce identical keys.
    #[must_use]
    pub fn compute(
        source_path: &str,
        content_hash: &ContentHash,
        link_map_fingerprint: &ContentHash,
    ) -> Self {
        let hash = composite_hash(&[
            source_path.as_bytes(),
            content_hash.as_bytes(),
            link_map_fingerprint.as_bytes(),
        ]);
        Self(hash.as_bytes().to_vec())
    }

    /// Return the raw bytes for use as a cache key.
    #[must_use]
    pub fn as_bytes(&self) -> &[u8] {
        &self.0
    }
}
```

### `link_map_fingerprint`

```rust
/// Compute a deterministic fingerprint of the link_map for cache invalidation.
///
/// The link_map is serialized to a canonical JSON representation and then SHA-256 hashed.
/// This ensures that any change to ID assignments invalidates the cached transform.
///
/// # Determinism
///
/// The `HashMap` iteration order is non-deterministic, so entries MUST be sorted
/// by key before serialization.
///
/// # Preconditions
/// - `link_map` may be empty (no links mapped), but must not contain entries with
///   empty source_path keys.
///
/// # Postconditions
/// - Returns a `ContentHash` that is deterministic for identical link_map contents.
/// - Different link_map contents produce different fingerprints (with overwhelming probability).
fn compute_link_map_fingerprint(link_map: &HashMap<String, IdMapping>) -> ContentHash {
    let mut sorted_entries: Vec<(&String, &IdMapping)> = link_map.iter().collect();
    sorted_entries.sort_by_key(|(k, _)| *k);
    let serialized = serde_json::to_string(&sorted_entries)
        .expect("IdMapping is Serialize; Vec serialization is infallible");
    ContentHash::compute(serialized.as_bytes())
}
```

## Preconditions

### P-01: Source path is non-empty
Every `source_path` passed to artifact lookup or storage MUST be a non-empty string.

```
FOR ALL source_path IN artifact_operations:
    source_path.is_empty() == false
```

### P-02: Content hash matches file bytes
The `content_hash` stored in a `TransformArtifact` MUST be the SHA-256 of the actual
file bytes at the time the transform was computed.

```
FOR ALL artifact IN stored_artifacts:
    SHA-256(read_bytes(artifact.source_path)) == artifact.content_hash
        AT THE TIME THE ARTIFACT WAS CREATED
```

### P-03: Link map fingerprint is deterministic
The link map fingerprint MUST be computed from a canonical (sorted-key) serialization
of the link_map, ensuring that semantically identical link_maps always produce the same
fingerprint regardless of HashMap iteration order.

### P-04: DocCache is open and TRANSFORM_TABLE is initialized
Before any `get_transform` / `put_transform` calls, `DocCache::open` MUST have succeeded
and `TRANSFORM_TABLE` MUST be accessible.

### P-05: Analysis and IdMapping are complete
Before computing a `TransformArtifactKey`, the caller MUST possess both a complete
`Analysis` (with non-empty `source_path` and `content`) and a valid `IdMapping` for
that source path in the `link_map`.

## Postconditions

### POST-01: Stored artifact contains complete transformed output
After `put_transform`, the stored `TransformArtifact.transformed_markdown` MUST be
identical to what `transform_file` would write to disk -- including frontmatter,
heading fixes, link rewrites, context injection, see-also, and tag generation.

### POST-02: Cache hit returns byte-identical output
When a `TransformArtifact` is retrieved via cache hit, its `transformed_markdown` MUST
be byte-identical to what a fresh transform of the same inputs would produce.

### POST-03: Cache miss triggers fresh computation
When `get_transform` returns `None` for a given key, the system MUST compute a fresh
transform for that source path and store the resulting artifact.

### POST-04: Mixed run produces complete output
When some source paths are cache hits and others are cache misses, the final
`TransformResult` counts MUST reflect ALL source paths (hits + misses), and ALL
output files MUST be written to the output directory (from cache for hits, freshly
computed for misses).

### POST-05: Cache key collision resistance
No two distinct `(source_path, content_hash, link_map_fingerprint)` tuples SHALL map
to the same cache key. SHA-256 collision resistance provides this guarantee with
overwhelming probability.

## Invariants

### INV-01: Source-path uniqueness
Within a single `run_index` invocation, each `source_path` maps to exactly one
`TransformArtifact`. Multiple source paths never share a cache entry.

### INV-02: Cache key determinism
`TransformArtifactKey::compute(a, b, c) == TransformArtifactKey::compute(a, b, c)` for
all valid inputs `a`, `b`, `c`. The function is referentially transparent.

### INV-03: Artifact integrity on read
A `TransformArtifact` retrieved from cache MUST satisfy:
- `source_path` matches the source path used to construct the lookup key.
- `content_hash` matches the SHA-256 of the file bytes at time of creation.
- `transformed_markdown` is non-empty.

### INV-04: Write-then-read consistency
After a successful `put_transform(key, artifact)`, a subsequent `get_transform(key)`
on the same `DocCache` instance MUST return `Some(artifact)` where
`artifact.transformed_markdown` is byte-identical to what was stored.

### INV-05: No partial writes visible
If `put_transform` fails (I/O error, serialization error), the cache MUST NOT contain
a partially-written entry for that key. redb's ACID transactions guarantee this at the
storage layer.

### INV-06: Forward progress
The transform stage MUST make progress: it MUST NOT indefinitely return stale artifacts.
When inputs change (different content_hash or link_map_fingerprint), the new key will
not match the old key, forcing re-computation.

## Error Taxonomy

```rust
/// Errors specific to transform artifact capture and reuse.
///
/// This enum covers ALL failure modes of the artifact persistence subsystem.
/// Every fallible operation in this module returns `Result<T, TransformArtifactError>`.
#[non_exhaustive]
#[derive(Debug, thiserror::Error, Clone, PartialEq, Eq)]
pub enum TransformArtifactError {
    /// The source path is empty or invalid.
    /// Precondition P-01 violated.
    #[error("empty source path: source path must be a non-empty string")]
    EmptySourcePath,

    /// The link map does not contain an entry for the given source path.
    /// Precondition P-05 violated: caller must ensure mapping exists before lookup.
    #[error("no IdMapping found for source path: {source_path}")]
    MissingIdMapping {
        source_path: String,
    },

    /// The link map fingerprint could not be computed.
    /// This is a serialization failure -- IdMapping must be Serializable.
    #[error("failed to serialize link map for fingerprinting: {message}")]
    LinkMapFingerprintFailed {
        message: String,
    },

    /// Cache read failed during artifact lookup.
    /// Wraps the underlying CacheError or redb error.
    #[error("cache read failed for transform artifact (source: {source_path}): {message}")]
    CacheReadFailed {
        source_path: String,
        message: String,
    },

    /// Cache write failed during artifact storage.
    /// Wraps the underlying CacheError or redb error.
    #[error("cache write failed for transform artifact (source: {source_path}): {message}")]
    CacheWriteFailed {
        source_path: String,
        message: String,
    },

    /// The cached artifact failed deserialization.
    /// Indicates data corruption or schema mismatch.
    #[error("cached artifact deserialization failed for source path {source_path}: {message}")]
    DeserializationFailed {
        source_path: String,
        message: String,
    },

    /// The file could not be read for content hashing.
    /// I/O error at the boundary between filesystem and cache subsystem.
    #[error("failed to read file for content hashing: {source_path}: {message}")]
    FileReadFailed {
        source_path: String,
        message: String,
    },

    /// The fresh transform computation failed.
    /// Delegated from the existing transform module.
    #[error("transform computation failed for source path {source_path}: {message}")]
    TransformComputationFailed {
        source_path: String,
        message: String,
    },

    /// The output file could not be written.
    /// I/O error when materializing the cached artifact to the output directory.
    #[error("failed to write output file for source path {source_path}: {message}")]
    OutputWriteFailed {
        source_path: String,
        message: String,
    },
}
```

## Contract Signatures

### Core Functions

```rust
/// Compute a deterministic cache key for a transform artifact.
///
/// Pure function. No side effects. No I/O.
///
/// # Preconditions
/// - P-01: source_path is non-empty
/// - P-05: analysis has valid content, link_map has entry for source_path
///
/// # Postconditions
/// - POST-05: key is collision-resistant
/// - INV-02: deterministic for identical inputs
fn compute_artifact_key(
    source_path: &str,
    content_hash: &ContentHash,
    link_map_fingerprint: &ContentHash,
) -> TransformArtifactKey;
```

```rust
/// Compute the SHA-256 fingerprint of a link_map for cache invalidation.
///
/// Pure function. No side effects. No I/O.
///
/// # Preconditions
/// - P-03: link_map entries have non-empty keys
///
/// # Postconditions
/// - Deterministic for identical link_map contents
fn compute_link_map_fingerprint(
    link_map: &HashMap<String, IdMapping>,
) -> ContentHash;
```

```rust
/// Attempt to load a cached transform artifact for a single source path.
///
/// I/O boundary: reads from DocCache.
///
/// # Preconditions
/// - P-01: source_path is non-empty
/// - P-04: DocCache is open
/// - P-05: content_hash and link_map_fingerprint are valid
///
/// # Postconditions
/// - Returns Ok(Some(artifact)) if cache hit, where artifact satisfies INV-03
/// - Returns Ok(None) if cache miss (no entry for this key)
/// - Returns Err on cache read failure or deserialization failure
///
/// # Errors
/// - TransformArtifactError::CacheReadFailed
/// - TransformArtifactError::DeserializationFailed
fn load_cached_artifact(
    cache: &DocCache,
    source_path: &str,
    content_hash: &ContentHash,
    link_map_fingerprint: &ContentHash,
) -> Result<Option<TransformArtifact>, TransformArtifactError>;
```

```rust
/// Persist a transform artifact to cache.
///
/// I/O boundary: writes to DocCache.
///
/// # Preconditions
/// - P-01: artifact.source_path is non-empty
/// - P-04: DocCache is open
/// - artifact.transformed_markdown is non-empty
///
/// # Postconditions
/// - POST-01: stored artifact contains complete transformed output
/// - INV-04: subsequent get returns the same artifact
/// - INV-05: no partial writes on failure
///
/// # Errors
/// - TransformArtifactError::CacheWriteFailed
fn store_artifact(
    cache: &DocCache,
    artifact: &TransformArtifact,
    link_map_fingerprint: &ContentHash,
) -> Result<(), TransformArtifactError>;
```

```rust
/// Transform all analyses with caching support.
///
/// For each analysis:
///   1. Compute content hash of original file bytes.
///   2. Compute artifact key from (source_path, content_hash, link_map_fingerprint).
///   3. Check cache: if hit, write cached markdown to output file.
///   4. If miss, run fresh transform, store artifact to cache, write to output file.
///
/// Replaces the current `transform_all` call in `run_index` STEP 4.
///
/// # Preconditions
/// - P-04: DocCache is open
/// - P-05: Every analysis has a corresponding entry in link_map
/// - analyses is non-empty (guaranteed by earlier pipeline steps)
/// - output directory is writable (validated by acquire_output_lock)
///
/// # Postconditions
/// - POST-04: TransformResult reflects ALL source paths
/// - Every source path has a corresponding output file in output/docs/
/// - Cache contains an artifact for every successfully transformed source path
/// - INV-01: no duplicate cache entries per source path (per run)
///
/// # Errors
/// - TransformArtifactError::MissingIdMapping (if analysis has no link_map entry)
/// - TransformArtifactError::FileReadFailed (if source file cannot be read for hashing)
/// - TransformArtifactError::TransformComputationFailed (if fresh transform fails)
/// - TransformArtifactError::CacheWriteFailed (if artifact cannot be stored)
/// - TransformArtifactError::OutputWriteFailed (if output file cannot be written)
fn transform_all_cached(
    analyses: &[Analysis],
    link_map: &HashMap<String, IdMapping>,
    output_dir: &Path,
    cache: &DocCache,
) -> Result<TransformResult, TransformArtifactError>;
```

```rust
/// Write a cached artifact's markdown to the output directory.
///
/// I/O boundary: writes to filesystem.
///
/// # Preconditions
/// - artifact.transformed_markdown is non-empty
/// - link_map has an entry for artifact.source_path
/// - output_dir/docs/ exists and is writable
///
/// # Postconditions
/// - File exists at output_dir/docs/{mapping.filename}
/// - File contents are byte-identical to artifact.transformed_markdown
///
/// # Errors
/// - TransformArtifactError::MissingIdMapping
/// - TransformArtifactError::OutputWriteFailed
fn write_artifact_to_output(
    artifact: &TransformArtifact,
    link_map: &HashMap<String, IdMapping>,
    docs_dir: &Path,
) -> Result<(), TransformArtifactError>;
```

## Integration Point: run_index Pipeline

The contract requires modifying `run_index` (in `cmd/index.rs`) at STEP 4:

**Before:**
```rust
let transform_result = transform::transform_all(&analyses, &link_map, output)?;
```

**After:**
```rust
let cache = DocCache::open(CacheConfig::default())?;
let transform_result = transform::transform_all_cached(
    &analyses, &link_map, output, &cache
)?;
```

The `DocCache` instance SHOULD be created once at the top of `run_index` and shared
across all stages that need caching (currently only transform, but the architecture
allows future stages to use the same instance).

## Non-goals

- This contract does NOT change the transform logic itself. The six AST transformations
  (heading fix, link rewrite, H1 enforcement, context injection, see-also, frontmatter)
  remain unchanged.
- This contract does NOT implement a `StateReadSession` abstraction. The cache is accessed
  directly via `DocCache` typed methods. A session abstraction may be introduced in a
  future bead.
- This contract does NOT handle cache eviction or TTL. redb is unbounded in size; cache
  invalidation is solely via key mismatch (content or link_map changes).
- This contract does NOT cover parallel cache access from multiple processes. The existing
  `acquire_output_lock` ensures single-process access to the output directory; the same
  lock implicitly serializes cache writes.

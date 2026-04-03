# Architecture Spec: State Database with Bulk In-Memory Operations

> "Perfection is achieved when there is nothing left to take away." — Antoine de Saint-Exupéry

## 1. Goal

Every `ctd` command is idempotent. redb is the **source of truth** (like `tfstate`).
All state operations are **bulk**: load everything into memory at startup, diff in-memory, write everything back at shutdown.

**2 redb transactions per run. One read. One write. Zero per-file overhead.**

## 2. Library Stack

| Layer | Library | Role | Status | Blessed? |
|-------|---------|------|--------|----------|
| Source of truth | **redb** | ACID MVCC database. Bulk load at startup, bulk write at shutdown. | existing | No (but battle-tested) |
| Fixed-size state | **bytemuck** | Zero-copy Pod casts for FileState/UrlState. No serde, no deserialize. Just a memcpy. | new | ✅ Yes |
| Variable-size outputs | **rkyv** | Zero-copy deserialization for Analysis, Chunks, Strings. Read bytes as typed struct — no deserialize. | new | ✅ Yes |
| Hashing | **sha2** | SHA-256 content fingerprinting. | existing | ✅ Yes |
| In-memory state | **std HashMap** | Bulk-loaded from redb. No new dependency. | stdlib | — |
| Parallel hashing | **rayon** | SHA-256 all files in parallel. | existing | ✅ Yes |

**Two new deps. Both blessed.rs. Both zero-copy.**

### Why NOT moka

moka is a cache with LRU eviction. Our pipeline:
1. Loads ALL state into memory at startup (we WANT everything, not an LRU subset)
2. Passes data forward through `Vec<Analysis>` (each key read exactly once per run)
3. Writes ALL changes at shutdown

A cache for single-read-then-forward is overhead with no benefit. moka's eviction would ANTI-perform — evicting entries forces re-reads from redb. We want all state in memory always. `std::HashMap` wins.

### Why NOT bincode (for state)

bincode deserializes by allocating new `Vec<u8>`, `String`, `HashMap` etc. For 1000 Analysis structs that's 1000 heap allocations.

rkyv gives zero-copy archived access after copying redb value bytes into an owned archive wrapper. The archived root is then read by pointer cast over owned bytes, so callers never depend on redb transaction lifetimes.

For `FileState` (fixed 200 bytes, all `[u8; 32]` and `u64`), we go even further: **bytemuck `Pod` cast**. No serde at all. Just `pod_read_unaligned::<FileStateRaw>(&bytes)`. A single memcpy of 200 bytes onto the stack.

## 3. The tfstate Model

### redb Tables

```
file_state        key: source_path (&str)   → FileStateRaw (200 bytes, Pod)
analysis_outputs  key: [u8; 32]             → rkyv(Analysis)
transform_outputs key: [u8; 32]             → rkyv(String)
chunk_outputs     key: [u8; 32]             → rkyv(Vec<Chunk>)
url_state         key: url (&str)           → UrlStateRaw (120 bytes, Pod)
scrape_outputs    key: [u8; 32]             → rkyv(ScrapedPage)
snapshots         key: [u8; 32]             → rkyv(Snapshot)
metadata          key: &str                 → &str
```

### FileStateRaw — Pod, 200 bytes, zero-copy

```rust
/// Fixed-size file state. All fields are Pod (plain old data).
/// Stored directly in redb. Read via bytemuck::pod_read_unaligned.
/// Zero deserialize. Zero allocate. Just a memcpy.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(C)]
struct FileStateRaw {
    content_hash: [u8; 32],       // SHA-256 of file bytes
    config_hash: [u8; 32],        // SHA-256 of category config (or zeroed)
    analysis_hash: [u8; 32],      // key into analysis_outputs table
    transform_hash: [u8; 32],     // key into transform_outputs table
    chunk_hash: [u8; 32],         // key into chunk_outputs table
    last_processed_secs: u64,     // unix timestamp
    _reserved: [u8; 32],          // reserved capacity to reach 200 bytes, future-proof
}
// Total: 32*5 + 8 + 32 = 200 bytes.
```

**bytemuck safety**: All fields are `[u8; N]` or `u64`. No padding bytes with undefined values (we use explicit `_reserved`). `#[repr(C)]` guarantees layout. Safe to `Pod` cast.

### UrlStateRaw — Pod, 120 bytes, zero-copy

```rust
/// Fixed-size URL state. Pod. Zero-copy read from redb.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(C)]
struct UrlStateRaw {
    content_hash: [u8; 32],       // SHA-256 of scraped markdown content
    url_hash: [u8; 32],           // SHA-256 of URL (key into scrape_outputs)
    last_fetched_secs: u64,       // unix timestamp
    status_code: u16,             // last HTTP status (200, 304, etc.)
    _reserved: [u8; 46],          // padding + future ETag/Last-Modified slot
}
// Total: 32*2 + 8 + 2 + 46 = 120 bytes.
```

**ETag is NOT stored for now.** spider-rs does not expose HTTP headers. We use fetch+hash (Option 1). The `_reserved` bytes give us room to add ETag support later without a schema migration.

### Variable-size outputs — rkyv zero-copy

`Analysis`, `Vec<Chunk>`, transformed markdown `String`, `ScrapedPage`, and `Snapshot` are all archived with `rkyv` and moved through a small owned wrapper instead of returning archived values directly:

```rust
#[derive(Debug)]
pub struct OwnedArchive<T: rkyv::Archive> {
    bytes: Box<[u8]>,
    _marker: std::marker::PhantomData<T>,
}

impl<T: rkyv::Archive> OwnedArchive<T> {
    pub fn archived(&self) -> &T::Archived;
}

impl<T> OwnedArchive<T>
where
    T: rkyv::Archive + rkyv::Deserialize<T, rkyv::api::high::HighDeserializer<rkyv::rancor::Error>>,
{
    pub fn deserialize(&self) -> Result<T>;
}
```

Writing: `rkyv::to_bytes::<rkyv::rancor::Error>(&value)` → `Vec<u8>` → store in redb.
Reading: copy the redb value bytes into `OwnedArchive<T>`, then call `archived()` for zero-copy access or `deserialize()` when an owned value is required.

**Why the wrapper exists:** the archived view stays tied to owned bytes, so callers can hold cached outputs for the whole run without returning raw archived values with transaction-scoped lifetimes.

## 4. The Two-Transaction Architecture

### `ctd index` flow

```
┌─ STARTUP (Transaction 1: READ) ──────────────────────────┐
│                                                           │
│  let read = state_db.begin_read()?                        │
│  └─→ read.load_file_states()                              │
│       → bytemuck::cast_slice bytes → &[FileStateRaw]     │
│       → HashMap<String, FileStateRaw> (one memcpy each)  │
│  └─→ compute config_hash from category config file        │
│                                                           │
└───────────────────────────────────────────────────────────┘
                            │
                            ▼
┌─ DIFF (pure in-memory, zero I/O) ────────────────────────┐
│                                                           │
│  rayon::par_iter over discovered files:                   │
│    current_hash = sha256(file_bytes)  // parallel         │
│    match HashMap.get(source_path):                        │
│      None         → NEW                                   │
│      Some(state)  →                                       │
│        state.content_hash != current_hash → CHANGED       │
│        state.config_hash != config_hash   → CHANGED       │
│        otherwise                          → UNCHANGED     │
│                                                           │
│  for each HashMap key NOT in discovered:                  │
│    → DELETED                                              │
│                                                           │
└───────────────────────────────────────────────────────────┘
                            │
                            ▼
┌─ BULK LOAD OUTPUTS (Transaction 1 continued) ────────────┐
│                                                           │
│  For UNCHANGED files only, using the same read session:   │
│    read.load_analyses(analysis_hashes)                    │
│    → HashMap<[u8; 32], OwnedArchive<Analysis>>           │
│    read.load_transforms(transform_hashes)                 │
│    → HashMap<[u8; 32], OwnedArchive<String>>             │
│    read.load_chunks(chunk_hashes)                         │
│    → HashMap<[u8; 32], OwnedArchive<Vec<Chunk>>>         │
│                                                           │
│  All unchanged cached outputs are now available in memory │
│  under the single read transaction for this run.          │
│                                                           │
└───────────────────────────────────────────────────────────┘
                            │
                            ▼
┌─ PIPELINE (in-memory, zero redb I/O) ────────────────────┐
│                                                           │
│  UNCHANGED files:                                         │
│    cached analysis/transform/chunk outputs are reused     │
│    deserialize only when a stage needs owned mutation     │
│                                                           │
│  CHANGED + NEW files:                                     │
│    → analyze_single_file()  → Analysis                   │
│    → transform_file()       → written to disk             │
│    → chunk_file()           → written to disk             │
│                                                           │
│  All stages operate on Vec<Analysis> (in memory)          │
│  assign_ids stays unchanged; transform/chunk only run      │
│  for changed and new files while cached outputs are reused │
│                                                           │
└───────────────────────────────────────────────────────────┘
                            │
                            ▼
┌─ SHUTDOWN (Transaction 2: WRITE) ────────────────────────┐
│                                                           │
│  redb.begin_write()                                       │
│  └─→ write FileStateRaw for all changed/new files         │
│       → bytemuck::bytes_of(&state) → &[u8] (zero-copy)   │
│  └─→ rkyv::to_bytes for new analysis/transform/chunk      │
│  └─→ delete DELETED file state entries                    │
│  commit()                                                 │
│                                                           │
│  ONE transaction. ACID. If it fails, old state preserved. │
│                                                           │
└───────────────────────────────────────────────────────────┘
```

### Performance profile (1000-file repo, 3 changed)

| Operation | Time | Method |
|-----------|------|--------|
| Bulk load 1000 FileStateRaw from redb | ~0.5ms | bytemuck Pod cast (memcpy) |
| SHA-256 of 1000 files (parallel) | ~6ms | rayon + sha2 |
| HashMap lookups for 1000 files | ~0.1ms | std HashMap |
| rkyv access 997 ArchivedAnalysis | ~0.1ms | pointer cast on owned archived bytes |
| Deserialize 3 changed Analysis | negligible | only 3 files |
| Compute 3 changed files | actual work | markdown parsing |
| rkyv serialize 3 new outputs | ~0.01ms | rkyv to_bytes |
| bytemuck write 3 FileStateRaw | ~0.01ms | memcpy into redb |
| **Total state overhead** | **~7ms** | |

On the next run with zero changes: ~7ms total. Skip all computation.

### `ctd scrape` flow — Fetch + Hash Compare

spider-rs does NOT expose HTTP headers (ETag, Last-Modified). So we use the honest approach:

```
STARTUP:  bulk load url_state → HashMap<String, UrlStateRaw>
FETCH:    spider-rs fetches ALL pages (network cost paid — unavoidable)
HASH:     for each scraped page:
            hash = sha256(markdown_content)
            match HashMap.get(url):
              None         → NEW page → process
              Some(state)  →
                state.content_hash == hash → UNCHANGED (skip ALL CPU)
                state.content_hash != hash → CHANGED → reprocess
PROCESS:  only process CHANGED + NEW pages
          UNCHANGED pages: reuse cached output from redb
SHUTDOWN: write updated url_state + scrape_outputs to redb
```

**What we save:** ALL CPU work for unchanged pages (no markdown conversion, no analysis, no transform, no chunk, no index rebuild). The HTTP download already happened — that's spider-rs's responsibility and can't be avoided without bypassing it.

**Future optimization (not now):** Replace spider-rs with reqwest for known URLs. Send conditional GET with stored ETag. Get 304 → skip download entirely. Requires bypassing spider-rs.

### `ctd watch` / `ctd apply`

Already uses redb for snapshots. Migrate `DocCache` calls to `StateDb`. Same logic, new types.

## 5. The `StateDb` API

```rust
pub struct StateDb {
    db: Database,
}

pub struct StateReadSession<'db> {
    read_txn: redb::ReadTransaction<'db>,
}

pub struct OwnedArchive<T: rkyv::Archive> {
    bytes: Box<[u8]>,
    _marker: std::marker::PhantomData<T>,
}

impl StateDb {
    /// Open the state database (creates if not exists).
    pub fn open(path: &Path) -> Result<Self>;

    /// Open the one shared read transaction for the entire command.
    pub fn begin_read(&self) -> Result<StateReadSession<'_>>;

    /// Commit all state changes in one write transaction after the read session is dropped.
    pub fn commit_changes(&self, changes: &StateChanges) -> Result<()>;
}

impl<'db> StateReadSession<'db> {
    /// Bulk load all file states via bytemuck Pod cast.
    pub fn load_file_states(&self) -> Result<HashMap<String, FileStateRaw>>;

    /// Bulk load all URL states via bytemuck Pod cast.
    pub fn load_url_states(&self) -> Result<HashMap<String, UrlStateRaw>>;

    /// Bulk load archived analysis outputs for the requested hashes.
    pub fn load_analyses(
        &self,
        hashes: &[[u8; 32]],
    ) -> Result<HashMap<[u8; 32], OwnedArchive<Analysis>>>;

    /// Bulk load archived transform outputs for the requested hashes.
    pub fn load_transforms(
        &self,
        hashes: &[[u8; 32]],
    ) -> Result<HashMap<[u8; 32], OwnedArchive<String>>>;

    /// Bulk load archived chunk outputs for the requested hashes.
    pub fn load_chunks(
        &self,
        hashes: &[[u8; 32]],
    ) -> Result<HashMap<[u8; 32], OwnedArchive<Vec<Chunk>>>>;

    /// Bulk load archived scrape outputs for the requested hashes.
    pub fn load_scrapes(
        &self,
        hashes: &[[u8; 32]],
    ) -> Result<HashMap<[u8; 32], OwnedArchive<ScrapedPage>>>;

    /// Bulk load archived snapshots for watch/apply.
    pub fn load_snapshots(
        &self,
        hashes: &[[u8; 32]],
    ) -> Result<HashMap<[u8; 32], OwnedArchive<Snapshot>>>;
}

/// Batch of state changes to commit atomically.
pub struct StateChanges {
    pub updated_files: Vec<(String, FileStateRaw)>,
    pub deleted_files: Vec<String>,
    pub new_analyses: Vec<([u8; 32], Vec<u8>)>,      // rkyv serialized bytes
    pub new_transforms: Vec<([u8; 32], Vec<u8>)>,     // rkyv serialized bytes
    pub new_chunks: Vec<([u8; 32], Vec<u8>)>,         // rkyv serialized bytes
    pub updated_urls: Vec<(String, UrlStateRaw)>,
    pub deleted_urls: Vec<String>,
    pub new_scrapes: Vec<([u8; 32], Vec<u8>)>,        // rkyv serialized bytes
    pub new_snapshots: Vec<([u8; 32], Vec<u8>)>,
    pub deleted_snapshots: Vec<[u8; 32]>,
}
```

## 6. Wiring into `run_index`

```rust
pub fn run_index(source: &Path, output: &Path, config: &IndexConfig) -> Result<()> {
    let (files, manifest) = discover::discover_files(source, config.path_filter.as_deref())?;

    let state_db = StateDb::open(&output.join(".state/ctd_state.redb"))?;
    let read = state_db.begin_read()?;
    let known_states = read.load_file_states()?;
    let config_hash = compute_config_hash(config.category_config.as_deref());
    let diff = compute_file_diff(&files, source, &config_hash, &known_states)?;
    print_diff_stats(&diff);

    let analysis_hashes = diff.unchanged.iter().map(|(_, state)| state.analysis_hash).collect::<Vec<_>>();
    let transform_hashes = diff.unchanged.iter().map(|(_, state)| state.transform_hash).collect::<Vec<_>>();
    let chunk_hashes = diff.unchanged.iter().map(|(_, state)| state.chunk_hash).collect::<Vec<_>>();

    let cached_analyses = read.load_analyses(&analysis_hashes)?;
    let cached_transforms = read.load_transforms(&transform_hashes)?;
    let cached_chunks = read.load_chunks(&chunk_hashes)?;

    let mut analyses: Vec<Analysis> = diff.unchanged.iter()
        .map(|(_, state)| cached_analyses[&state.analysis_hash].deserialize())
        .collect::<Result<_>>()?;
    let mut transformed: Vec<String> = diff.unchanged.iter()
        .map(|(_, state)| cached_transforms[&state.transform_hash].deserialize())
        .collect::<Result<_>>()?;
    let mut chunks: Vec<Vec<Chunk>> = diff.unchanged.iter()
        .map(|(_, state)| cached_chunks[&state.chunk_hash].deserialize())
        .collect::<Result<_>>()?;

    drop(read);

    let analyzed = analyze::analyze_files(&diff.changed_and_new(), source, config.category_config.as_deref())?;
    let transformed_new = transform::transform_files(&analyzed.analyses, output)?;
    let chunked_new = chunk::chunk_files(&transformed_new, output)?;

    analyses.extend(analyzed.analyses);
    transformed.extend(transformed_new.into_iter());
    chunks.extend(chunked_new.into_iter());
    analyses.sort_by(|a, b| a.source_path.cmp(&b.source_path));

    let changes = build_state_changes(&diff, &analyses, &transformed, &chunks, &config_hash);
    state_db.commit_changes(&changes)?;

    write_manifest(output, &manifest)
}
```

## 7. The `compute_file_diff` Function

```rust
pub struct FileDiff {
    pub unchanged: Vec<(DiscoveryFile, FileStateRaw)>,
    pub changed: Vec<DiscoveryFile>,
    pub new_files: Vec<DiscoveryFile>,
    pub deleted: Vec<String>,
}

impl FileDiff {
    pub fn changed_and_new(&self) -> Vec<DiscoveryFile> {
        self.changed.iter().chain(self.new_files.iter()).cloned().collect()
    }
}

pub fn compute_file_diff(
    discovered: &[DiscoveryFile],
    source_dir: &Path,
    config_hash: &[u8; 32],
    known_states: &HashMap<String, FileStateRaw>,
) -> Result<FileDiff> {
    // Parallel SHA-256 of all discovered files
    let file_hashes: HashMap<String, [u8; 32]> = discovered
        .par_iter()
        .map(|file| {
            let bytes = fs::read(source_dir.join(&file.source_path))?;
            Ok((file.source_path.clone(), sha256(&bytes)))
        })
        .collect::<Result<_>>()?;

    let mut unchanged = Vec::new();
    let mut changed = Vec::new();
    let mut new_files = Vec::new();
    let mut seen = HashSet::new();

    for file in discovered {
        seen.insert(file.source_path.clone());
        let current_hash = file_hashes
            .get(&file.source_path)
            .ok_or_else(|| Error::missing_discovered_hash(&file.source_path))?;

        match known_states.get(&file.source_path) {
            None => new_files.push(file.clone()),
            Some(state) if state.content_hash != *current_hash => changed.push(file.clone()),
            Some(state) if state.config_hash != *config_hash => changed.push(file.clone()),
            Some(state) => unchanged.push((file.clone(), *state)),
        }
    }

    let deleted: Vec<String> = known_states.keys()
        .filter(|k| !seen.contains(*k))
        .cloned()
        .collect();

    Ok(FileDiff { unchanged, changed, new_files, deleted })
}
```

## 8. Cargo.toml Changes

### Add
```toml
bytemuck = { version = "1", features = ["derive"] }  # blessed.rs, zero-copy Pod casts
rkyv = { version = "0.8", features = ["bytecheck"] } # blessed.rs, zero-copy deserialization
```

### Keep (unchanged)
```toml
redb = "2"           # source of truth
sha2 = "0.10"        # content hashing (blessed.rs)
rayon = "1.11.0"     # parallel hashing (blessed.rs)
serde = { version = "1", features = ["derive"] }  # still used for non-cache types
bincode = "1.3"      # keep for any remaining non-rkyv serialization
lru = "0.16.3"       # remove after migration
parking_lot = "0.12" # used elsewhere
```

### Remove (after migration)
```toml
lru = "0.16.3"       # replaced by bulk HashMap
serde_json = "1"     # replaced by rkyv for state, keep for other uses
```

## 9. Implementation Order

1. Add bytemuck + rkyv to Cargo.toml
2. Add `FileStateRaw` and `UrlStateRaw` Pod types with bytemuck derives
3. Add rkyv derives to `Analysis`, `Chunk`, and nested types (`Heading`, `Link`, `LinkKind`)
4. Add `file_state`, `url_state` tables to redb schema
5. Add `StateDb`, `StateReadSession`, and `OwnedArchive<T>`
6. Add `compute_file_diff()` function
7. Wire diff into `run_index()` with one shared read session per run
8. Load archived analysis, transform, and chunk outputs for unchanged files
9. Wire scrape URL diff and cached scrape output loading
10. Add explicit snapshot load and write paths for watch/apply
11. Remove LRU backend from `CacheBackendInner`
12. Run `cargo test`
13. Manual integration test: `ctd index` twice on same source

## 10. Telemetry

```
[STEP 2] STATE DIFF
  Unchanged: 997 files (loading from state)
  Changed:      2 files (reprocessing)
  New:          1 file  (processing)
  Deleted:      0 files
  Total:    1000 files (99.7% reuse)

[STEP 3] ANALYZE
  Cached:    997 files (rkyv zero-copy from state)
  Computed:    3 files (2 changed + 1 new)
```

## 11. Decisions Log

| Decision | Choice | Rationale |
|----------|--------|-----------|
| Source of truth | redb | Like tfstate. ACID. Already in use. Persistent. |
| In-memory acceleration | std HashMap (NOT moka) | Pipeline reads each key once. Cache for single-read is overhead. |
| Fixed-size state serialization | bytemuck Pod cast | 200 bytes, memcpy, zero deserialize. Blessed.rs. |
| Variable-size output serialization | rkyv | Zero-copy. Read bytes as typed struct. Blessed.rs. |
| Scrape change detection | Fetch + hash compare | spider-rs doesn't expose ETags. Honest about the limitation. |
| Scrape ETag support | Not now (reserved bytes) | Requires bypassing spider-rs. Future optimization. |
| Transactions | 2 per run (read + write) | Bulk operations. No per-entry overhead. |
| rkyv derives needed on | Analysis, Chunk, Heading, Link, LinkKind, ScrapedPage, Snapshot | All types stored in redb outputs. |

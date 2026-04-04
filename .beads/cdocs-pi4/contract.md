bead_id: cdocs-pi4
bead_title: data: remove LRU backend from `CacheBackendInner` after state migration
phase: state-1-contract
updated_at: 2026-04-03T00:00:00Z

# Contract Specification: Remove LRU Backend from `CacheBackendInner`

## Context

- **Feature**: Delete the obsolete `Lru` variant from `CacheBackendInner`, replace `CacheBackend::Memory` internals with redb's `InMemoryBackend`, and remove the `lru` and `parking_lot` crate dependencies.
- **Domain terms**:
  - `CacheBackendInner` — private enum inside `cache::mod` with variants `Lru` and `Redb`.
  - `CacheBackend` — public enum (`Memory`, `File(path)`). `Memory` currently constructs an LRU-backed cache despite its doc comment claiming it uses "redb's `InMemoryBackend`".
  - `DocCache` — the public thread-safe cache struct. Every method has match arms for both backends.
  - `get_from_lru` / `put_to_lru` — private LRU-specific read/write helpers.
  - `DEFAULT_LRU_CAPACITY` — const `10_000`, used only to construct the LRU cache.
- **Assumptions**:
  - `CacheConfig::in_memory()` is called exclusively from test code (16 call sites in `chunking_adapter.rs` tests, 12 in `cache::mod` tests). No production code path uses it.
  - redb v2 provides `redb::backends::InMemoryBackend` which implements the `Backend` trait, allowing `Database::create(backend)` to produce an in-memory database with the same API as file-backed.
  - `parking_lot` is used only for the `RwLock<LruCache<...>>` inside the `Lru` variant. No other file in the crate imports `parking_lot`.
  - `NonZeroUsize` is imported only for LRU capacity construction.
- **Open questions**: None. The scope is fully determined by the codebase audit.

## Preconditions

- **PRE-1**: `CacheBackendInner::Lru` exists in `centralized-docs/src/cache/mod.rs` at line 47.
- **PRE-2**: `lru = "0.16.3"` is declared in `[dependencies]` of `centralized-docs/Cargo.toml` at line 94.
- **PRE-3**: `parking_lot = "0.12.5"` is declared in `[dependencies]` at line 95.
- **PRE-4**: `use lru::LruCache`, `use parking_lot::RwLock`, `use std::num::NonZeroUsize` are imported in `cache/mod.rs`.
- **PRE-5**: All call sites of `CacheConfig::in_memory()` are within `#[test]` functions only (verified by grep audit).
- **PRE-6**: `get_from_lru` and `put_to_lru` are private functions used exclusively by `CacheBackendInner::Lru` match arms.

## Postconditions

- **POST-1**: `CacheBackendInner::Lru` variant is removed. The enum either (a) collapses to a single-variant newtype wrapping `Database`, or (b) retains the `Redb(Database)` variant as the sole arm. Exhaustive matches compile without `Lru` arms.
- **POST-2**: `CacheBackend::Memory` constructs a redb `Database` via `Database::create(InMemoryBackend::new())` instead of `LruCache::new(NonZeroUsize::new(DEFAULT_LRU_CAPACITY))`.
- **POST-3**: `get_from_lru` and `put_to_lru` functions are deleted entirely.
- **POST-4**: `DEFAULT_LRU_CAPACITY` constant is deleted.
- **POST-5**: The following imports are removed from `cache/mod.rs`:
  - `use lru::LruCache;`
  - `use parking_lot::RwLock;`
  - `use std::num::NonZeroUsize;`
- **POST-6**: `lru = "0.16.3"` is removed from `[dependencies]` in `Cargo.toml`.
- **POST-7**: `parking_lot = "0.12.5"` is removed from `[dependencies]` in `Cargo.toml`.
- **POST-8**: All `match &self.inner { CacheBackendInner::Lru(..) => ..., CacheBackendInner::Redb(..) => ... }` blocks are simplified to operate directly on the redb `Database` without branching.
- **POST-9**: `DocCache::open()` for `CacheBackend::Memory` calls `self.initialize_tables()` (same as the file path), since redb InMemoryBackend also requires table creation.
- **POST-10**: `cargo test` passes with zero compilation errors and zero test failures.
- **POST-11**: `cargo build 2>&1 | grep -i lru` returns no matches — no production or test code references the `lru` crate.
- **POST-12**: `CacheConfig::in_memory()` remains a public, stable API. All 28 existing call sites compile and behave identically.

## Invariants

- **INV-1**: redb remains the sole storage backend. `Database` is the only type held inside `CacheBackendInner`.
- **INV-2**: The public API of `DocCache` is unchanged: `get`, `put`, `get_or_compute`, `get_document`, `put_document`, `get_scrape`, `put_scrape`, `get_transform`, `put_transform`, `get_snapshot`, `put_snapshot`, `clear_all`, `stats` — all retain their current signatures and return types.
- **INV-3**: `CacheConfig::in_memory()` and `CacheConfig::new(path)` both produce a fully functional `DocCache`. In-memory caches are non-persistent (drop on exit); file caches persist to disk.
- **INV-4**: Key/value size validation (`MAX_KEY_SIZE`, `MAX_VALUE_SIZE`) is preserved for all code paths.
- **INV-5**: `CacheError` taxonomy is unchanged — `KeyTooLarge`, `ValueTooLarge`, `BackendError` remain the only variants.
- **INV-6**: `ContentHash`, `content_hash`, `url_hash`, `path_hash`, `composite_hash` are untouched.
- **INV-7**: `CacheType` enum and `CacheStats` struct are untouched.
- **INV-8**: The `#[cfg(test)] mod tests` block retains all existing test functions with identical semantics.
- **INV-9**: `EnabledTypes` and `CacheConfig::disable`/`enable` behavior is unchanged.
- **INV-10**: `table_for_type` mapping and all `TableDefinition` constants are unchanged.

## Error Taxonomy

No new error variants are introduced. Existing `CacheError` variants cover all failure modes:

| Variant | Trigger | Unchanged? |
|---|---|---|
| `CacheError::KeyTooLarge { size, max }` | Key exceeds `MAX_KEY_SIZE` (256 bytes) | Yes |
| `CacheError::ValueTooLarge { size, max }` | Serialized value exceeds `MAX_VALUE_SIZE` (50 MB) | Yes |
| `CacheError::BackendError { operation, message }` | redb I/O failure (open_table, begin_read, begin_write, insert, get, commit, delete_table) | Yes |

The LRU removal eliminates an entire class of potential errors: LRU capacity saturation and silent eviction. Under the new design, in-memory redb has no capacity limit (bounded only by process memory), matching the file-backed behavior.

## Contract Signatures (Affected Functions)

These are the function signatures that MUST change implementation but NOT change public signature:

```rust
// BEFORE (internal branching on two backends):
pub fn open(config: CacheConfig) -> Result<Self>
// AFTER (single path — both Memory and File create a Database):
pub fn open(config: CacheConfig) -> Result<Self>

// BEFORE (match on Lru vs Redb for every method):
pub fn get<V: DeserializeOwned>(&self, cache_type: CacheType, key: &[u8]) -> Result<Option<V>>
pub fn put<V: Serialize>(&self, cache_type: CacheType, key: &[u8], value: &V) -> Result<()>
pub fn get_snapshot<V: DeserializeOwned>(&self, key: &[u8]) -> Result<Option<V>>
pub fn put_snapshot<V: Serialize>(&self, key: &[u8], value: &V) -> Result<()>
pub fn clear_all(&self) -> Result<()>
pub fn stats(&self) -> Result<CacheStats>
// AFTER (all directly operate on Database, no match branching):
// Signatures identical; implementation simplified to single code path.

// DELETED entirely:
fn get_from_lru<V: DeserializeOwned>(cache: &RwLock<LruCache<Vec<u8>, Vec<u8>>>, key: &[u8]) -> Result<Option<V>>
fn put_to_lru<V: Serialize>(cache: &RwLock<LruCache<Vec<u8>, Vec<u8>>>, key: &[u8], value: &V) -> Result<()>

// DELETED entirely:
const DEFAULT_LRU_CAPACITY: usize = 10_000;

// CHANGED internally (Memory now uses redb InMemoryBackend):
pub fn in_memory() -> Self { ... }  // CacheConfig method
```

## Entities to Remove (Exhaustive List)

| Item | Location | Reason |
|---|---|---|
| `use lru::LruCache;` | `cache/mod.rs:18` | No LRU usage |
| `use parking_lot::RwLock;` | `cache/mod.rs:19` | No RwLock usage |
| `use std::num::NonZeroUsize;` | `cache/mod.rs:23` | No NonZeroUsize usage |
| `DEFAULT_LRU_CAPACITY` | `cache/mod.rs:37` | LRU-only constant |
| `CacheBackendInner::Lru(RwLock<LruCache<Vec<u8>, Vec<u8>>>)` | `cache/mod.rs:47` | Variant removed |
| `Self::Lru(_) => write!(f, "CacheBackendInner::Lru(..)")` | `cache/mod.rs:55` | Debug arm removed |
| `get_from_lru` function | `cache/mod.rs:299-309` | LRU-only helper |
| `put_to_lru` function | `cache/mod.rs:312-322` | LRU-only helper |
| `CacheBackend::Memory => { ... LruCache::new(...) }` block | `cache/mod.rs:412-415` | Replaced with InMemoryBackend |
| `CacheBackendInner::Lru(cache) => ...` arms (8 total) | `cache/mod.rs:459,473,534,545,562,586` + Debug:55 + open:415 | All match arms |
| `lru = "0.16.3"` | `Cargo.toml:94` | Unused dependency |
| `parking_lot = "0.12.5"` | `Cargo.toml:95` | Unused dependency |
| `#[allow(clippy::expect_used)]` annotation | `cache/mod.rs:409` | Only needed for LRU NonZeroUsize expect |

## Entities to Modify (Exhaustive List)

| Item | Location | Change |
|---|---|---|
| `CacheBackendInner` enum | `cache/mod.rs:45-50` | Remove `Lru` variant; either single-variant or `Redb(Database)` only |
| `CacheBackendInner::Debug` impl | `cache/mod.rs:52-59` | Remove `Lru` arm |
| `CacheBackend::Memory` doc comment | `cache/mod.rs:149` | Already says "redb's InMemoryBackend" — now truthful |
| `DocCache::open` | `cache/mod.rs:410-429` | `Memory` arm: `Database::create(InMemoryBackend::new())` |
| `DocCache::initialize_tables` | `cache/mod.rs:431-447` | Remove early return for LRU; always initialize |
| `DocCache::get` | `cache/mod.rs:454-465` | Remove Lru match arm; direct redb path |
| `DocCache::put` | `cache/mod.rs:468-481` | Remove Lru match arm; direct redb path |
| `DocCache::get_snapshot` | `cache/mod.rs:532-540` | Remove Lru match arm; direct redb path |
| `DocCache::put_snapshot` | `cache/mod.rs:543-553` | Remove Lru match arm; direct redb path |
| `DocCache::clear_all` | `cache/mod.rs:560-581` | Remove Lru match arm; direct redb path |
| `DocCache::stats` | `cache/mod.rs:584-609` | Remove Lru match arm; direct redb path |
| Module-level doc comment | `cache/mod.rs:1-14` | Remove LRU references |
| Comment "bounded lru crate" | `cache/mod.rs:40` | Remove or rewrite |
| `DocCache` doc comment | `cache/mod.rs:390-393` | Remove "blessed lru (memory)" reference |

## Non-goals

- **NG-1**: Do NOT change any public API signatures on `DocCache`, `CacheConfig`, `CacheType`, `CacheStats`, `ContentHash`, or free functions.
- **NG-2**: Do NOT modify the `CacheError` enum or error module.
- **NG-3**: Do NOT refactor the table definitions (`DOCUMENT_TABLE`, `SCRAPE_TABLE`, etc.) or `table_for_type`.
- **NG-4**: Do NOT change `EnabledTypes` or the `disable`/`enable` builder API.
- **NG-5**: Do NOT alter any test assertions or test semantics — tests must pass as-is.
- **NG-6**: Do NOT add new tests (that is the scope of State 2). This contract only covers the cleanup itself.
- **NG-7**: Do NOT touch any files outside `cache/mod.rs` and `Cargo.toml`.
- **NG-8**: Do NOT modify `MAX_KEY_SIZE`, `MAX_VALUE_SIZE`, or validation functions.

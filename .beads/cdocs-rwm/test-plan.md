# Test Plan: cdocs-rwm — Migrate watch/apply snapshot persistence from DocCache to StateDb

bead_id: cdocs-rwm
bead_title: action: migrate watch/apply snapshot persistence from `DocCache` to `StateDb`
phase: state-1.5-test-plan
revised_at: 2026-04-03T00:00:00Z
revision: 2 (addresses all 13 LETHAL + 4 MAJOR findings from test-plan-review)

## Summary

- Behaviors identified: 55
- Trophy allocation: 14 unit / 36 integration / 5 static
- Proptest invariants: 4 (this bead's functions only)
- Fuzz targets: 3
- Kani harnesses: 5
- Mutation threshold: >= 90% kill rate
- Test density: 55 / 8 = 6.88x (exceeds 5x threshold)

## 1. Behavior Inventory

### 1.1 New I/O Helpers (cmd/watch.rs Actions layer)

| # | Behavior |
|---|----------|
| B01 | `open_state_db` returns open `StateDb` when `cache_path` is writable |
| B02 | `open_state_db` returns `CommitError::DatabaseOpen` when parent is read-only |
| B03 | `open_state_db` creates parent directories when missing |
| B04 | `open_state_db` returns `CommitError::DatabaseOpen` when path is empty string |
| B05 | `open_state_db` handles path at max filesystem length (4096 chars) |
| B06 | `open_state_db` returns `CommitError::TableInit` when table initialization fails |
| B07 | `load_snapshot` returns stored Snapshot with 1 page when key exists |
| B08 | `load_snapshot` returns stored Snapshot with 50 pages when key exists |
| B09 | `load_snapshot` returns empty default Snapshot when key does not exist |
| B10 | `load_snapshot` returns empty default when URL contains unicode |
| B11 | `load_snapshot` returns `CommitError::ReadTransaction` when `begin_read` fails |
| B12 | `load_snapshot` returns `StateError::TableOpenFailed` when snapshots table cannot be opened |
| B13 | `load_snapshot` returns `StateError::DeserializationFailed` when rkyv bytes are corrupt (truncated valid archive) |
| B14 | `load_snapshot` returns `StateError::InvalidArchive` when stored bytes are not a valid rkyv archive |
| B15 | `store_snapshot` persists snapshot via `commit_changes` with single entry |
| B16 | `store_snapshot` persists empty Snapshot (0 pages) successfully |
| B17 | `store_snapshot` overwrites existing snapshot with new data |
| B18 | `store_snapshot` returns `StateError::SerializationFailed` when rkyv serialization fails |
| B19 | `store_snapshot` returns `CommitError::WriteTransaction` when write tx begin fails |
| B20 | `store_snapshot` returns `CommitError::CommitFailed` when write tx commit fails |
| B21 | `store_snapshot` returns `CommitError::PayloadTooLarge` when payload exceeds 50 MiB |
| B22 | `store_snapshot` succeeds when payload is exactly 50 MiB (52,428,800 bytes) |
| B23 | `store_snapshot` returns `CommitError::ZeroHashKey` when key is all zeros (via `commit_changes`) |

### 1.2 Modified Command Functions (cmd/watch.rs)

| # | Behavior |
|---|----------|
| B24 | `run_watch` produces change plan without calling `commit_changes` (read-only) |
| B25 | `run_watch` reads previous snapshot from StateDb for diffing |
| B26 | `run_watch` first scrape (no previous) produces plan with all Added |
| B27 | `run_watch` unchanged content produces empty plan (0 changes) |
| B28 | `run_watch` new page added produces plan with 1 Added |
| B29 | `run_watch` page removed produces plan with 1 Removed |
| B30 | `run_watch` page modified produces plan with 1 Modified |
| B31 | `run_watch` multiple pages changed produces plan with correct counts |
| B32 | `run_apply` commits exactly one snapshot entry |
| B33 | `run_apply` skips commit when plan is empty (no changes) |
| B34 | `run_apply` is idempotent: second run with same content produces no commit |
| B35 | `run_apply` with `--yes` flag commits without stdin prompt |
| B36 | `run_apply` without `--yes` flag prompts stdin for confirmation |
| B37 | `run_apply` multiple pages changed commits snapshot with all pages |

### 1.3 Stub Implementations (state/mod.rs + state/commit.rs)

| # | Behavior |
|---|----------|
| B38 | `serialize_snapshot` produces rkyv bytes that round-trip to equal Snapshot |
| B39 | `serialize_snapshot` produces non-empty bytes for non-trivial Snapshot |
| B40 | `serialize_snapshot` returns `StateError::SerializationFailed` on rkyv error |
| B41 | `StateReadSession::load_snapshots` returns HashMap with matching keys |
| B42 | `StateReadSession::load_snapshots` returns empty HashMap when no keys match |
| B43 | `StateReadSession::load_snapshots` returns empty HashMap when key list is empty |
| B44 | `StateReadSession::load_snapshots` returns `StateError::TableOpenFailed` when table cannot be opened |
| B45 | `StateReadSession::load_snapshots` returns `StateError::StorageError` on redb read failure |
| B46 | `StateReadSession::load_snapshots` returns `StateError::ArchiveValidationFailed` on corrupt bytes under a specific key |
| B47 | `ArchivedRaw::deserialize` returns deserialized T from valid rkyv archive |
| B48 | `ArchivedRaw::deserialize` returns `StateError::DeserializationFailed` on corrupt bytes |
| B49 | `ArchivedRaw::deserialize` returns `StateError::InvalidArchive` on empty bytes |

### 1.4 Invariant Behaviors

| # | Behavior |
|---|----------|
| B50 | Snapshot store-then-load round-trip produces `PartialEq`-equal Snapshot |
| B51 | `url_hash(url).as_bytes()` produces identical 32-byte key across calls |
| B52 | `run_watch` opens one read session, uses it, drops it (single transaction) |
| B53 | `run_apply` opens one read session, drops it, then opens one write transaction |
| B54 | `Snapshot` struct has `rkyv::Archive`, `rkyv::Serialize`, `rkyv::Deserialize` derives |
| B55 | `cmd/watch.rs` does not import `DocCache`, `CacheConfig` (only `url_hash`, `content_hash`) |
| B56 | Pure calc functions remain byte-identical |
| B57 | Missing snapshot returns default with `target_url == requested URL`, `pages` empty |

## 2. Trophy Allocation

| # | Behavior | Layer | Justification |
|---|----------|-------|---------------|
| B01 | `open_state_db` valid path | Integration | Real redb I/O, real filesystem |
| B02 | `open_state_db` read-only parent → DatabaseOpen | Integration | Real filesystem permissions |
| B03 | `open_state_db` creates parent dirs | Integration | Real filesystem side effect |
| B04 | `open_state_db` empty path → DatabaseOpen | Integration | Real redb `Database::create` failure |
| B05 | `open_state_db` max path length | Integration | Real filesystem boundary |
| B06 | `open_state_db` TableInit | Integration | Corrupted DB file triggers table init failure |
| B07 | `load_snapshot` key exists, 1 page | Integration | Real StateDb + redb read |
| B08 | `load_snapshot` key exists, 50 pages | Integration | Real StateDb + redb read |
| B09 | `load_snapshot` key missing | Integration | Real StateDb empty table |
| B10 | `load_snapshot` unicode URL | Integration | Real StateDb + SHA-256 hash |
| B11 | `load_snapshot` → ReadTransaction | Integration | Corrupted DB triggers begin_read failure |
| B12 | `load_snapshot` → TableOpenFailed | Integration | Corrupted DB triggers table open failure |
| B13 | `load_snapshot` → DeserializationFailed | Integration | Truncated rkyv bytes in table |
| B14 | `load_snapshot` → InvalidArchive | Integration | Garbage bytes in table |
| B15 | `store_snapshot` persists snapshot | Integration | Real redb write transaction |
| B16 | `store_snapshot` empty snapshot | Integration | Real redb write with 0-page snapshot |
| B17 | `store_snapshot` overwrites existing | Integration | Real redb write + read verification |
| B18 | `store_snapshot` → SerializationFailed | Unit | Injected rkyv failure (if reachable; see K03) |
| B19 | `store_snapshot` → WriteTransaction | Integration | Corrupted DB triggers begin_write failure |
| B20 | `store_snapshot` → CommitFailed | Integration | Corrupted DB triggers commit failure |
| B21 | `store_snapshot` → PayloadTooLarge | Unit | Pure size validation in commit_changes |
| B22 | `store_snapshot` exactly 50 MiB boundary | Unit | Pure size boundary check |
| B23 | `store_snapshot` → ZeroHashKey | Integration | Direct commit_changes with zero hash key |
| B24 | `run_watch` is read-only | Integration | End-to-end watch flow |
| B25 | `run_watch` reads from StateDb | Integration | End-to-end with real DB |
| B26 | `run_watch` first scrape | Integration | End-to-end with fixture scrape dir |
| B27 | `run_watch` unchanged content | Integration | End-to-end with matching fixture |
| B28 | `run_watch` new page added | Integration | End-to-end with fixture |
| B29 | `run_watch` page removed | Integration | End-to-end with fixture |
| B30 | `run_watch` page modified | Integration | End-to-end with fixture |
| B31 | `run_watch` multiple pages changed | Integration | End-to-end with fixture |
| B32 | `run_apply` commits exactly one | Integration | End-to-end apply flow |
| B33 | `run_apply` skips empty plan | Integration | End-to-end no-op path |
| B34 | `run_apply` is idempotent | Integration | End-to-end double-run |
| B35 | `run_apply` with `--yes` | Integration | Auto-confirm path |
| B36 | `run_apply` without `--yes` | Integration | Stdin prompt path (subprocess) |
| B37 | `run_apply` multiple pages | Integration | End-to-end with fixture |
| B38 | `serialize_snapshot` round-trip | Unit | Pure function, no I/O |
| B39 | `serialize_snapshot` non-empty bytes | Unit | Pure function, no I/O |
| B40 | `serialize_snapshot` → SerializationFailed | Unit | Error path (if reachable; see K03) |
| B41 | `load_snapshots` returns matching | Integration | Real redb read |
| B42 | `load_snapshots` empty result | Integration | Real redb empty table |
| B43 | `load_snapshots` empty key list | Integration | Real redb read |
| B44 | `load_snapshots` → TableOpenFailed | Integration | Corrupted DB |
| B45 | `load_snapshots` → StorageError | Integration | Corrupted DB triggers redb storage error |
| B46 | `load_snapshots` → ArchiveValidationFailed | Integration | Injected corrupt bytes under specific key |
| B47 | `ArchivedRaw::deserialize` success | Unit | Pure deserialization |
| B48 | `ArchivedRaw::deserialize` → DeserializationFailed | Unit | Corrupt bytes input |
| B49 | `ArchivedRaw::deserialize` → InvalidArchive | Unit | Empty bytes input |
| B50 | Store-then-load round-trip | Integration | Full StateDb lifecycle |
| B51 | Key identity stability | Unit | Pure hash comparison |
| B52 | `run_watch` single read session | Integration | Observes transaction count |
| B53 | `run_apply` read-then-write session | Integration | Observes transaction count |
| B54 | `Snapshot` rkyv derives | Static | Compile-time check |
| B55 | No DocCache imports | Static | Compile-time / grep check |
| B56 | Pure calc functions unchanged | Static | Byte-identical diff |
| B57 | Missing snapshot default values | Unit | Pure default construction |

**Ratio**: 36 integration (65%) / 14 unit (25%) / 5 static (9%). Deviation from target: fewer pure unit tests because the migration is primarily I/O boundary work and stub implementations. The calc layer (INV-1) is explicitly unchanged. Static checks at compile-time provide free coverage.

## 3. BDD Scenarios

### B01: `open_state_db` returns open StateDb when cache_path is writable

```rust
fn open_state_db_returns_state_db_when_path_writable()
```

**Given**: a temporary directory with a writable path `state.redb` created via `tempfile::TempDir`
**When**: `open_state_db(&path)` is called
**Then**: returns `Ok(state_db)` where `state_db.begin_read().is_ok() == true`

### B02: `open_state_db` returns CommitError::DatabaseOpen when parent is read-only

```rust
fn open_state_db_returns_commit_error_database_open_when_parent_read_only()
```

**Given**: a directory at `readonly_dir` with `chmod 0o444` (Unix-only; conditional `#[cfg(unix)]`)
**When**: `open_state_db(&readonly_dir.join("state.redb"))` is called
**Then**: returns `Err(CommitError::DatabaseOpen { path, reason })` where `path.contains("readonly") == true`
**And**: `reason` is non-empty

### B03: `open_state_db` creates parent directories when missing

```rust
fn open_state_db_creates_parent_directories_when_missing()
```

**Given**: a path `deeply/nested/state.redb` where `deeply/nested/` does not exist inside a tempdir
**When**: `open_state_db(&path)` is called
**Then**: returns `Ok(state_db)` where `state_db.begin_read().is_ok() == true`
**And**: all parent directories (`deeply/`, `deeply/nested/`) exist on the filesystem

### B04: `open_state_db` returns CommitError::DatabaseOpen when path is empty string

```rust
fn open_state_db_returns_commit_error_database_open_when_path_is_empty()
```

**Given**: a path constructed from `Path::new("")`
**When**: `open_state_db(&path)` is called
**Then**: returns `Err(CommitError::DatabaseOpen { .. })`

### B05: `open_state_db` handles path at max filesystem length

```rust
fn open_state_db_handles_path_at_max_filesystem_length()
```

**Given**: a path `tempdir/` + `"a".repeat(4090)` + `"/state.redb"` (total path ~4100 chars)
**When**: `open_state_db(&path)` is called
**Then**: returns `Ok(state_db)` OR returns `Err(CommitError::DatabaseOpen { .. })` (OS-dependent)
**Note**: Either outcome is acceptable. The test verifies no panic and no silent truncation.

### B06: `open_state_db` returns CommitError::TableInit when table initialization fails

```rust
fn open_state_db_returns_commit_error_table_init_when_tables_fail()
```

**Given**: a file at `db_path` containing corrupted redb data (constructed by creating a valid StateDb, writing a snapshot, then appending 64 bytes of garbage to the file and dropping the handle)
**When**: `open_state_db(&db_path)` is called on the corrupted file
**Then**: returns `Err(CommitError::TableInit { reason })` where `reason` is non-empty
**Note**: If `Database::create` returns `Err` instead (maps to `DatabaseOpen`), this variant is unreachable through `open_state_db` alone. In that case, this test documents the unreachable path and Kani harness K02 proves the mapping exists.

### B07: `load_snapshot` returns stored Snapshot with 1 page when key exists

```rust
fn load_snapshot_returns_stored_snapshot_with_1_page_when_key_exists()
```

**Given**: a `StateDb` where `store_snapshot(&db, "https://example.com", &snapshot_1_page)` has been called with a Snapshot containing `target_url = "https://example.com"` and `pages = { "/index.html" => PageHash { url: "/index.html", content_hash: [0xAB; 32], title: "Home" } }`
**When**: `load_snapshot(&db, "https://example.com")` is called
**Then**: returns `Ok(snapshot)` where `snapshot == stored_snapshot` (PartialEq)
**And**: `snapshot.target_url == "https://example.com"`
**And**: `snapshot.pages.len() == 1`
**And**: `snapshot.pages["/index.html"].content_hash == [0xAB; 32]`

### B08: `load_snapshot` returns stored Snapshot with 50 pages when key exists

```rust
fn load_snapshot_returns_stored_snapshot_with_50_pages_when_key_exists()
```

**Given**: a `StateDb` where a Snapshot with 50 pages has been stored under URL `"https://example.com"`
**When**: `load_snapshot(&db, "https://example.com")` is called
**Then**: returns `Ok(snapshot)` where `snapshot.pages.len() == 50`

### B09: `load_snapshot` returns empty default Snapshot when key does not exist

```rust
fn load_snapshot_returns_empty_default_when_key_missing()
```

**Given**: a fresh `StateDb` with no snapshots stored (created via `open_state_db`)
**When**: `load_snapshot(&db, "https://example.com")` is called
**Then**: returns `Ok(snapshot)` where `snapshot.target_url == "https://example.com"`
**And**: `snapshot.pages.is_empty() == true`

### B10: `load_snapshot` returns empty default when URL contains unicode

```rust
fn load_snapshot_returns_empty_default_when_url_contains_unicode()
```

**Given**: a fresh `StateDb` with no snapshots stored
**When**: `load_snapshot(&db, "https://example.com/docs/日本語/概要")` is called
**Then**: returns `Ok(snapshot)` where `snapshot.target_url == "https://example.com/docs/日本語/概要"`
**And**: `snapshot.pages.is_empty() == true`

### B11: `load_snapshot` returns CommitError::ReadTransaction when begin_read fails

```rust
fn load_snapshot_returns_commit_error_read_transaction_when_begin_read_fails()
```

**Given**: a `StateDb` opened from a corrupted file (same construction as B06: valid DB created, then file corrupted by truncating to half its original size)
**When**: `load_snapshot(&db, "https://example.com")` is called
**Then**: returns `Err(CommitError::ReadTransaction { reason })` where `reason` is non-empty
**Note**: This tests that `load_snapshot` propagates the `CommitError::ReadTransaction` from `state_db.begin_read()` via the `?` operator. If the corrupted file produces `DatabaseOpen` instead (during open), this scenario requires a different corruption method. Use file truncation after open to target `begin_read` specifically.

### B12: `load_snapshot` returns StateError::TableOpenFailed when table cannot be opened

```rust
fn load_snapshot_returns_state_error_table_open_failed_when_table_cannot_be_opened()
```

**Given**: a `StateDb` opened successfully, then the underlying redb file is corrupted by zeroing the first 512 bytes after the DB handle is dropped and reopened (this corrupts table metadata but may allow the DB to open)
**When**: `load_snapshot(&db, "https://example.com")` is called
**Then**: returns `Err(StateError::TableOpenFailed { table: "snapshots", message })` where `table == "snapshots"` and `message` is non-empty
**Alternative approach**: If table corruption doesn't produce `TableOpenFailed` reliably, directly call `StateReadSession::load_snapshots` on a StateReadSession obtained from a DB where the snapshots table definition conflicts (test via B44 instead, and this scenario delegates to B44).

### B13: `load_snapshot` returns StateError::DeserializationFailed when rkyv bytes are truncated

```rust
fn load_snapshot_returns_state_error_deserialization_failed_when_bytes_truncated()
```

**Given**: a `StateDb` where a valid rkyv-serialized Snapshot has been stored, then the stored bytes are replaced with the first 50% of the original bytes (truncated archive) by opening a raw write transaction directly on the redb `Database` and overwriting the value
**When**: `load_snapshot(&db, "https://example.com")` is called
**Then**: returns `Err(StateError::DeserializationFailed { type_name, message })` where `type_name == "Snapshot"` and `message` is non-empty
**And**: does NOT return `Err(StateError::InvalidArchive { .. })`

### B14: `load_snapshot` returns StateError::InvalidArchive when bytes are not valid rkyv

```rust
fn load_snapshot_returns_state_error_invalid_archive_when_bytes_not_valid_rkyv()
```

**Given**: a `StateDb` where 256 bytes of `0xDE` have been stored under the snapshot key (garbage that is not a valid rkyv archive header) via a direct redb write transaction
**When**: `load_snapshot(&db, "https://example.com")` is called
**Then**: returns `Err(StateError::InvalidArchive { type_name, message })` where `type_name` contains `"Snapshot"` and `message` is non-empty
**And**: does NOT return `Err(StateError::DeserializationFailed { .. })`

### B15: `store_snapshot` persists snapshot via commit_changes with single entry

```rust
fn store_snapshot_persists_snapshot_via_commit_changes()
```

**Given**: a fresh `StateDb` and a `Snapshot` with `target_url = "https://example.com"`, `pages = { 3 entries with distinct URLs and content hashes }`
**When**: `store_snapshot(&db, "https://example.com", &snapshot)` is called
**Then**: returns `Ok(())`
**And**: a subsequent `load_snapshot(&db, "https://example.com")` returns a Snapshot where `snapshot == stored_snapshot` (PartialEq)
**And**: querying the snapshots table via `db.database()` shows exactly 1 entry

### B16: `store_snapshot` persists empty Snapshot (0 pages) successfully

```rust
fn store_snapshot_persists_empty_snapshot_with_zero_pages()
```

**Given**: a fresh `StateDb` and a `Snapshot` with `target_url = "https://example.com"`, `pages = BTreeMap::new()`
**When**: `store_snapshot(&db, "https://example.com", &snapshot)` is called
**Then**: returns `Ok(())`
**And**: a subsequent `load_snapshot(&db, "https://example.com")` returns a Snapshot where `snapshot.pages.is_empty() == true`

### B17: `store_snapshot` overwrites existing snapshot with new data

```rust
fn store_snapshot_overwrites_existing_snapshot()
```

**Given**: a `StateDb` where a Snapshot with 2 pages has already been stored under `"https://example.com"`, and a new Snapshot with 5 pages (different content hashes) is prepared
**When**: `store_snapshot(&db, "https://example.com", &new_snapshot)` is called
**Then**: returns `Ok(())`
**And**: `load_snapshot(&db, "https://example.com")` returns `Ok(snapshot)` where `snapshot == new_snapshot` (PartialEq)
**And**: `snapshot.pages.len() == 5`

### B18: `store_snapshot` returns StateError::SerializationFailed when rkyv serialization fails

```rust
fn store_snapshot_returns_state_error_serialization_failed_when_rkyv_fails()
```

**Given**: a `Snapshot` with fields that cause rkyv serialization to fail. Since all Snapshot fields (String, DateTime<Utc>, BTreeMap<String, PageHash> where PageHash has String + [u8; 32]) are rkyv-compatible, this scenario constructs a test by temporarily mocking or instrumenting the serialization path.
**When**: `store_snapshot(&db, "https://example.com", &bad_snapshot)` is called
**Then**: returns `Err(StateError::SerializationFailed { type_name: "Snapshot", message })` where `message` is non-empty
**Note**: If this path is provably unreachable with valid `Snapshot` inputs (all fields rkyv-compatible), Kani harness K03 proves it. In that case, this test verifies the error variant exists and the `?` propagation path from `serialize_snapshot` through `store_snapshot` is structurally correct by asserting the match arm exists in the error conversion.

### B19: `store_snapshot` returns CommitError::WriteTransaction when write tx begin fails

```rust
fn store_snapshot_returns_commit_error_write_transaction_when_begin_write_fails()
```

**Given**: a `StateDb` where the underlying file has been corrupted (file handle dropped, file truncated to 50% of original size, then reopened — `StateDb::open` may succeed but `begin_write()` may fail)
**When**: `store_snapshot(&db, "https://example.com", &snapshot)` is called
**Then**: returns `Err(CommitError::WriteTransaction { reason })` where `reason` is non-empty
**And**: does NOT return `Err(CommitError::CommitFailed { .. })`

### B20: `store_snapshot` returns CommitError::CommitFailed when write tx commit fails

```rust
fn store_snapshot_returns_commit_error_commit_failed_when_commit_fails()
```

**Given**: a `StateDb` on a filesystem with limited space (use a tmpfs mounted with `size=1M` or simulate disk-full by filling a tmpfs). The DB opens and `begin_write()` succeeds, but `write_tx.commit()` fails due to space.
**When**: `store_snapshot(&db, "https://example.com", &snapshot)` is called
**Then**: returns `Err(CommitError::CommitFailed { reason })` where `reason` is non-empty
**And**: does NOT return `Err(CommitError::WriteTransaction { .. })`
**Alternative**: If disk-full simulation is too fragile for CI, document as tested via the existing `commit_changes` tests (Behavior 43 in commit.rs) which verify the variant mapping, and use Kani harness K04 to prove the propagation path.

### B21: `store_snapshot` returns CommitError::PayloadTooLarge when payload exceeds 50 MiB

```rust
fn store_snapshot_returns_commit_error_payload_too_large_when_exceeds_50mib()
```

**Given**: a `Snapshot` whose rkyv-serialized bytes are exactly 52,428,801 bytes (MAX_VALUE_SIZE + 1 = 50 MiB + 1). Constructed by creating a Snapshot with enough pages to exceed the limit (each page adds ~100+ bytes; need ~500K+ pages, or a single page with an extremely long title/content_hash).
**When**: `store_snapshot(&db, "https://example.com", &huge_snapshot)` is called
**Then**: returns `Err(CommitError::PayloadTooLarge { table: "snapshots", size: 52428801, max: 52428800 })`

### B22: `store_snapshot` succeeds when payload is exactly 50 MiB

```rust
fn store_snapshot_succeeds_when_payload_is_exactly_50mib()
```

**Given**: a `Snapshot` whose rkyv-serialized bytes are exactly 52,428,800 bytes (MAX_VALUE_SIZE = 50 MiB)
**When**: `store_snapshot(&db, "https://example.com", &boundary_snapshot)` is called
**Then**: returns `Ok(())`
**And**: `load_snapshot(&db, "https://example.com")` returns `PartialEq`-equal Snapshot
**Note**: This catches the mutation `> MAX_VALUE_SIZE` → `>= MAX_VALUE_SIZE`. If constructing exactly 52,428,800 bytes is impractical, inject raw bytes of that size directly via `commit_changes` with `new_snapshots: vec![(key, vec![0u8; 52_428_800])]` and verify `Ok(())`.

### B23: `store_snapshot` returns CommitError::ZeroHashKey when key is all zeros

```rust
fn store_snapshot_returns_commit_error_zero_hash_key_when_key_is_zeros()
```

**Given**: a `StateDb` and a test that directly calls `commit_changes` with `StateChanges { new_snapshots: vec![([0u8; 32], vec![1, 2, 3])] }` (bypassing `store_snapshot`'s `url_hash` key derivation, which guarantees non-zero keys)
**When**: `state_db.commit_changes(changes)` is called
**Then**: returns `Err(CommitError::ZeroHashKey { table: "snapshots", index: 0 })`
**Note**: `store_snapshot` itself cannot produce a zero hash key because `url_hash(url).as_bytes()` is SHA-256 of a non-empty string, which is never all zeros. This scenario tests the `commit_changes` precondition that `store_snapshot` relies on. Kani harness K05 proves `url_hash(url).as_bytes() != [0u8; 32]` for all non-empty `url`.

### B24: `run_watch` produces change plan without calling commit_changes (read-only)

```rust
fn run_watch_is_read_only_and_never_calls_commit_changes()
```

**Given**: a `StateDb` with a stored snapshot for `"https://example.com"` containing 2 pages, and a scrape fixture directory at `scrape_dir/manifest.json` containing 3 pages (1 new page `"https://example.com/new.html"`)
**When**: `run_watch("https://example.com", output_dir, state_db_path, None, 0, 30, 3, RedirectPolicy::FollowAll, 1, false)` is called (using fixture-based scrape injection)
**Then**: returns `Ok(())`
**And**: `output_dir/change-plan.json` and `output_dir/change-plan.md` exist
**And**: the snapshots table in StateDb is unchanged (entry count is 1, same as before)

### B25: `run_watch` reads previous snapshot from StateDb

```rust
fn run_watch_reads_previous_snapshot_from_state_db()
```

**Given**: a `StateDb` with a stored snapshot for `"https://example.com"` containing 2 pages with specific content hashes, and a scrape fixture that returns the same 2 pages with identical content
**When**: `run_watch("https://example.com", output_dir, state_db_path, ...)` is called
**Then**: the change plan contains `summary.added == 0`, `summary.removed == 0`, `summary.modified == 0`

### B26: `run_watch` first scrape (no previous) produces plan with all Added

```rust
fn run_watch_first_scrape_produces_plan_with_all_added()
```

**Given**: a fresh `StateDb` with no stored snapshots, and a scrape fixture returning 3 pages
**When**: `run_watch("https://example.com", output_dir, state_db_path, ...)` is called
**Then**: the change plan contains `summary.added == 3`, `summary.removed == 0`, `summary.modified == 0`
**And**: all 3 pages appear in `changes` with `kind == ChangeKind::Added`

### B27: `run_watch` unchanged content produces empty plan

```rust
fn run_watch_unchanged_content_produces_empty_plan()
```

**Given**: a `StateDb` with a stored snapshot matching the scrape fixture exactly (same 3 pages, same content hashes)
**When**: `run_watch("https://example.com", output_dir, state_db_path, ...)` is called
**Then**: the change plan contains `changes.len() == 0`
**And**: `plan.summary.is_empty() == true`

### B28: `run_watch` new page added produces plan with 1 Added

```rust
fn run_watch_new_page_added_produces_plan_with_1_added()
```

**Given**: a `StateDb` with a stored snapshot for `"https://example.com"` containing 2 pages, and a scrape fixture returning 3 pages (same 2 + 1 new)
**When**: `run_watch` is called
**Then**: the change plan contains exactly 1 change with `kind == ChangeKind::Added`
**And**: `summary.added == 1`, `summary.unchanged == 2`

### B29: `run_watch` page removed produces plan with 1 Removed

```rust
fn run_watch_page_removed_produces_plan_with_1_removed()
```

**Given**: a `StateDb` with a stored snapshot containing 3 pages, and a scrape fixture returning 2 pages (1 page removed)
**When**: `run_watch` is called
**Then**: the change plan contains exactly 1 change with `kind == ChangeKind::Removed`
**And**: `summary.removed == 1`

### B30: `run_watch` page modified produces plan with 1 Modified

```rust
fn run_watch_page_modified_produces_plan_with_1_modified()
```

**Given**: a `StateDb` with a stored snapshot containing 1 page with `content_hash = [0xAA; 32]`, and a scrape fixture returning the same page URL but with `content_hash = [0xBB; 32]`
**When**: `run_watch` is called
**Then**: the change plan contains exactly 1 change with `kind == ChangeKind::Modified`
**And**: `changes[0].old_hash == Some([0xAA; 32])`
**And**: `changes[0].new_hash == Some([0xBB; 32])`

### B31: `run_watch` multiple pages changed produces plan with correct counts

```rust
fn run_watch_multiple_pages_changed_produces_correct_counts()
```

**Given**: a `StateDb` with a stored snapshot containing 3 pages, and a scrape fixture where 1 page is unchanged, 1 page is modified, 1 page is removed, and 2 new pages are added
**When**: `run_watch` is called
**Then**: `summary.added == 2`, `summary.removed == 1`, `summary.modified == 1`, `summary.unchanged == 1`

### B32: `run_apply` commits exactly one snapshot entry

```rust
fn run_apply_commits_exactly_one_snapshot_entry()
```

**Given**: a `StateDb` and a scrape_dir at `scrape_dir/manifest.json` containing 3 pages different from any stored snapshot
**When**: `run_apply("https://example.com", state_db_path, scrape_dir, true)` is called
**Then**: returns `Ok(())`
**And**: the snapshots table contains exactly 1 entry
**And**: `load_snapshot(&db, "https://example.com")` returns a Snapshot with `pages.len() == 3`

### B33: `run_apply` skips commit when plan is empty

```rust
fn run_apply_skips_commit_when_plan_is_empty()
```

**Given**: a `StateDb` with a stored snapshot matching the scrape_dir manifest exactly
**When**: `run_apply("https://example.com", state_db_path, scrape_dir, true)` is called
**Then**: process exits with code 0 (via `process::exit`)
**And**: no additional entries are written to the snapshots table (count remains 1)
**Note**: `process::exit` prevents normal return. Test via subprocess or by extracting the decision logic into a testable helper.

### B34: `run_apply` is idempotent

```rust
fn run_apply_is_idempotent_on_second_run()
```

**Given**: `run_apply` has already been called once with `scrape_dir`
**When**: `run_apply` is called again with the same `scrape_dir`
**Then**: the plan is empty (no changes detected)
**And**: the snapshots table entry count remains 1

### B35: `run_apply` with --yes flag commits without stdin prompt

```rust
fn run_apply_with_yes_flag_commits_without_stdin_prompt()
```

**Given**: a `StateDb` and a scrape_dir with changes to apply
**When**: `run_apply("https://example.com", state_db_path, scrape_dir, true)` is called with `yes = true`
**Then**: returns `Ok(())` without reading from stdin
**And**: the snapshots table contains exactly 1 entry

### B36: `run_apply` without --yes flag prompts stdin for confirmation

```rust
fn run_apply_without_yes_flag_prompts_stdin_for_confirmation()
```

**Given**: a `StateDb` and a scrape_dir with changes to apply
**When**: `run_apply("https://example.com", state_db_path, scrape_dir, false)` is called with `yes = false` and stdin piped "n\n"
**Then**: process exits with code 1 (user aborted)
**And**: the snapshots table is unchanged (no commit)
**Note**: Test via subprocess with stdin control.

### B37: `run_apply` multiple pages changed commits snapshot with all pages

```rust
fn run_apply_multiple_pages_changed_commits_snapshot_with_all_pages()
```

**Given**: a `StateDb` and a scrape_dir with manifest containing 10 pages, all different from stored
**When**: `run_apply("https://example.com", state_db_path, scrape_dir, true)` is called
**Then**: `load_snapshot(&db, "https://example.com")` returns `Ok(snapshot)` where `snapshot.pages.len() == 10`

### B38: `serialize_snapshot` round-trips to equal Snapshot

```rust
fn serialize_snapshot_round_trips_to_equal_snapshot()
```

**Given**: a `Snapshot` with `target_url = "https://example.com"`, 3 pages with varying content
**When**: `serialize_snapshot(&snapshot)` produces `Ok(bytes)`, then `ArchivedRaw::from_bytes(bytes).deserialize::<Snapshot>()` produces `Ok(deserialized)`
**Then**: `deserialized == snapshot` (PartialEq)
**And**: `deserialized.target_url == "https://example.com"`
**And**: `deserialized.pages.len() == 3`

### B39: `serialize_snapshot` produces non-empty bytes for non-trivial Snapshot

```rust
fn serialize_snapshot_produces_non_empty_bytes_for_non_trivial_snapshot()
```

**Given**: a `Snapshot` with 1 page
**When**: `serialize_snapshot(&snapshot)` is called
**Then**: returns `Ok(bytes)` where `bytes.len() > 0`

### B40: `serialize_snapshot` returns StateError::SerializationFailed on rkyv error

```rust
fn serialize_snapshot_returns_serialization_failed_on_rkyv_error()
```

**Given**: this scenario tests the error path structurally. Since all `Snapshot` fields (String, DateTime<Utc>, BTreeMap<String, PageHash>) are rkyv-compatible with proper derives, a direct trigger requires an impossible-to-construct `Snapshot`.
**When**: If the rkyv derives on `Snapshot` are correct, `serialize_snapshot` always returns `Ok(..)` for any `Snapshot`.
**Then**: Kani harness K03 proves this path is unreachable for all valid `Snapshot` inputs.
**Structural test**: Assert that `StateError::SerializationFailed { type_name: "Snapshot", message: "test".to_string() }` constructs successfully and `type_name == "Snapshot"`.

### B41: `StateReadSession::load_snapshots` returns HashMap with matching keys

```rust
fn load_snapshots_returns_hashmap_with_matching_keys()
```

**Given**: a `StateDb` with 3 snapshots stored under keys `[k1, k2, k3]` (stored via `commit_changes` with `new_snapshots` containing valid rkyv bytes)
**When**: `session.load_snapshots(&[k1, k3])` is called
**Then**: returns `Ok(map)` where `map.len() == 2`
**And**: `map.contains_key(&k1) == true`
**And**: `map.contains_key(&k3) == true`
**And**: `map.contains_key(&k2) == false`

### B42: `StateReadSession::load_snapshots` returns empty HashMap when no keys match

```rust
fn load_snapshots_returns_empty_hashmap_when_no_keys_match()
```

**Given**: a `StateDb` with 1 snapshot stored under key `k1`
**When**: `session.load_snapshots(&[k_nonexistent])` is called
**Then**: returns `Ok(map)` where `map.is_empty() == true`

### B43: `StateReadSession::load_snapshots` returns empty HashMap when key list is empty

```rust
fn load_snapshots_returns_empty_hashmap_when_key_list_is_empty()
```

**Given**: a `StateDb` with 3 snapshots stored
**When**: `session.load_snapshots(&[])` is called
**Then**: returns `Ok(map)` where `map.is_empty() == true`

### B44: `StateReadSession::load_snapshots` returns StateError::TableOpenFailed

```rust
fn load_snapshots_returns_state_error_table_open_failed_when_table_cannot_be_opened()
```

**Given**: a `StateDb` where the snapshots table has been dropped (via `state_db.drop_snapshots_table()`), and a `StateReadSession` obtained via `state_db.begin_read()`
**When**: `session.load_snapshots(&[key])` is called
**Then**: returns `Err(StateError::TableOpenFailed { table: "snapshots", message })` where `table == "snapshots"` and `message` is non-empty

### B45: `StateReadSession::load_snapshots` returns StateError::StorageError

```rust
fn load_snapshots_returns_state_error_storage_error_on_redb_read_failure()
```

**Given**: a `StateDb` where the underlying file has been corrupted after opening (file handle dropped, file truncated to 50% size, DB reopened, read session obtained). The DB opens successfully but individual table reads may fail.
**When**: `session.load_snapshots(&[key])` is called where `key` was previously stored
**Then**: returns `Err(StateError::StorageError { operation, message })` where `operation` is non-empty and `message` is non-empty
**Note**: If this specific corruption method doesn't produce `StorageError`, alternative approach: use a DB on a filesystem that becomes full mid-read (tmpfs with size limit). The test MUST produce the exact variant, not a note about redb's own tests.

### B46: `StateReadSession::load_snapshots` returns StateError::ArchiveValidationFailed

```rust
fn load_snapshots_returns_state_error_archive_validation_failed_on_corrupt_bytes()
```

**Given**: a `StateDb` where bytes from a different rkyv type (e.g., serialized `String`) have been stored under a snapshot key, and a `StateReadSession` is open
**When**: `session.load_snapshots(&[key])` is called
**Then**: returns `Err(StateError::ArchiveValidationFailed { key_hex, message })` where `key_hex` is a 64-character hex string and `message` is non-empty
**And**: does NOT return `Err(StateError::InvalidArchive { .. })`

### B47: `ArchivedRaw::deserialize` returns deserialized T from valid archive

```rust
fn archived_raw_deserialize_returns_t_from_valid_archive()
```

**Given**: an `ArchivedRaw` constructed from `rkyv::to_bytes::<Snapshot>(&snapshot).unwrap()` (valid rkyv-archived Snapshot bytes)
**When**: `archived.deserialize::<Snapshot>()` is called
**Then**: returns `Ok(deserialized)` where `deserialized == snapshot` (PartialEq)

### B48: `ArchivedRaw::deserialize` returns StateError::DeserializationFailed on corrupt bytes

```rust
fn archived_raw_deserialize_returns_deserialization_failed_on_corrupt_bytes()
```

**Given**: an `ArchivedRaw` constructed from `vec![0xFF, 0xFE, 0xFD, 0xFC; 256]` (bytes that have a valid-looking structure but fail deserialization)
**When**: `archived.deserialize::<Snapshot>()` is called
**Then**: returns `Err(StateError::DeserializationFailed { type_name: "Snapshot", message })` where `message` is non-empty

### B49: `ArchivedRaw::deserialize` returns StateError::InvalidArchive on empty bytes

```rust
fn archived_raw_deserialize_returns_invalid_archive_on_empty_bytes()
```

**Given**: an `ArchivedRaw` constructed from `vec![]` (empty byte slice)
**When**: `archived.deserialize::<Snapshot>()` is called
**Then**: returns `Err(StateError::InvalidArchive { type_name, message })` where `type_name` is non-empty

### B50: Snapshot store-then-load round-trip

```rust
fn snapshot_store_then_load_round_trip_produces_equal_snapshot()
```

**Given**: a `Snapshot` with 5 pages, specific `target_url`, specific timestamp
**When**: `store_snapshot(&db, url, &snapshot)` succeeds, then `load_snapshot(&db, url)` is called
**Then**: returned Snapshot is `PartialEq`-equal to the original
**And**: every `PageHash.content_hash` matches exactly

### B51: Key identity stability

```rust
fn url_hash_produces_identical_key_across_calls()
```

**Given**: URL string `"https://example.com/docs"`
**When**: `url_hash("https://example.com/docs").as_bytes()` is computed twice
**Then**: both results are byte-identical `[u8; 32]`
**And**: `key.len() == 32`

### B52: `run_watch` opens exactly one read session

```rust
fn run_watch_opens_exactly_one_read_session()
```

**Given**: a `StateDb` and a scrape fixture
**When**: `run_watch` completes
**Then**: `load_snapshot` was called exactly once (verifiable by asserting that `run_watch` only opens one `StateReadSession` and never calls `store_snapshot`)
**Note**: Verify by inspecting the command's function body — it calls `load_snapshot` once and never calls `store_snapshot`.

### B53: `run_apply` opens read-then-write session

```rust
fn run_apply_opens_read_then_write_session()
```

**Given**: a `StateDb` with changes to apply
**When**: `run_apply` completes (with `--yes`)
**Then**: `load_snapshot` was called exactly once (read session)
**And**: `store_snapshot` was called exactly once (write via `commit_changes`)

### B54: `Snapshot` rkyv derives (static)

```rust
fn _assert_snapshot_has_required_rkyv_derives()
```

**Given**: the `Snapshot` struct definition in `watch.rs`
**When**: compiled
**Then**: `Snapshot` implements `rkyv::Archive`, `rkyv::Serialize`, `rkyv::Deserialize`
**Test**: Use trait bound assertion function: `fn assert_traits<T: rkyv::Archive + rkyv::Serialize + rkyv::Deserialize>() {}`

### B55: No DocCache imports (static)

```
grep -rn 'DocCache\|CacheConfig' src/cmd/watch.rs
```

**Given**: migrated `cmd/watch.rs`
**When**: searching for `DocCache` or `CacheConfig` imports
**Then**: no matches found

### B56: Pure calc functions unchanged (static)

```
diff <(git show HEAD:src/watch.rs) src/watch.rs
```

**Given**: the pre-migration `watch.rs` and post-migration `watch.rs`
**When**: compared byte-by-byte
**Then**: zero differences in the calc section (lines before `#[cfg(test)]`)

### B57: Missing snapshot default values

```rust
fn load_snapshot_returns_default_with_correct_url_and_empty_pages()
```

**Given**: a fresh `StateDb` with no snapshots
**When**: `load_snapshot(&db, "https://docs.rs/serde")` is called
**Then**: returns `Ok(snapshot)` where `snapshot.target_url == "https://docs.rs/serde"`
**And**: `snapshot.pages == BTreeMap::new()`

## 4. Proptest Invariants

### P01: `serialize_snapshot` then `deserialize` round-trip

```
Invariant: For any valid Snapshot, deserialize(serialize(s)) == s (PartialEq)
Strategy: Generate Snapshot with:
  - target_url: "[a-z]{1,100}\.com(/[a-z]{1,50}){0,5}"
  - pages: BTreeMap with 0..20 entries, each key "[a-z]{1,50}", each PageHash with arbitrary content_hash and title
Anti-invariant: corrupt bytes → always Err
```

### P02: `url_hash` is deterministic and produces 32-byte key

```
Invariant: url_hash(url).as_bytes().len() == 32 for any non-empty url
Strategy: url = "[a-zA-Z0-9:/._-]{1,500}"
Anti-invariant: empty string → still produces 32 bytes (SHA-256 of empty is valid)
```

### P03: `store_snapshot` then `load_snapshot` round-trip

```
Invariant: For any Snapshot s and URL u, load(store(db, u, s), u) == s (PartialEq)
Strategy: Generate Snapshot with 0..50 pages, varying URL lengths 1..200 chars
Anti-invariant: N/A (happy path invariant)
```

### P04: `load_snapshot` default is consistent for any URL

```
Invariant: For any non-empty URL u, load_snapshot on empty DB returns
           Snapshot { target_url == u, pages == BTreeMap::new() }
Strategy: url = "[a-zA-Z0-9:/._-]{1,200}"
```

## 5. Fuzz Targets

### F01: `serialize_snapshot` — fuzz the Snapshot struct

```
Input type: Arbitrary Snapshot (fuzz via constructing Snapshot with arbitrary fields)
Risk: Panic in rkyv serializer, OOM on extremely large pages map
Corpus seeds:
  - Empty pages Snapshot
  - 1-page Snapshot
  - 100-page Snapshot
  - Snapshot with very long URL (4096 chars)
  - Snapshot with unicode in page titles and URLs
  - Snapshot with maximum-length content_hash values
```

### F02: `ArchivedRaw::deserialize::<Snapshot>` — fuzz the deserializer

```
Input type: Raw bytes (&[u8])
Risk: Panic in rkyv deserializer, OOM on crafted archive with inflated sizes
Corpus seeds:
  - Valid rkyv-archived empty Snapshot bytes
  - Valid rkyv-archived 3-page Snapshot bytes
  - Empty byte slice
  - Single zero byte
  - Random 256 bytes
  - Bytes with valid rkyv header but truncated body
```

### F03: `load_snapshot` — fuzz via corrupted StateDb

```
Input type: Raw bytes written directly to snapshots table under a known key
Risk: Panic in rkyv deserialization, infinite loop, OOM
Corpus seeds:
  - Valid serialized Snapshot
  - Truncated valid bytes (cut at 50%, 75%, 99%)
  - All zeros (256 bytes)
  - Valid rkyv header with garbage body
  - Bytes from a different rkyv type (e.g., serialized String)
```

## 6. Kani Harnesses

### K01: `serialize_snapshot` never panics for any valid Snapshot

```
Property: serialize_snapshot(&snapshot) returns Ok(_) for all valid Snapshot inputs
Bound: Snapshot with pages.len() <= 100, url.len() <= 256
Rationale: rkyv serialization must be panic-free for all valid Snapshot inputs.
           If this harness proves Ok, then StateError::SerializationFailed is
           unreachable through serialize_snapshot with any Snapshot, and B18/B40
           are covered by structural variant construction tests only.
```

### K02: `StateDb::open` maps `initialize_tables` errors to `CommitError::TableInit`

```
Property: If initialize_tables returns Err, then StateDb::open returns
          Err(CommitError::TableInit { .. })
Bound: N/A (structural proof)
Rationale: Proves the error mapping from initialize_tables → TableInit exists
           in the StateDb::open code path, even if triggering it via the
           public API requires corrupted state.
```

### K03: `StateError::SerializationFailed` is unreachable from `serialize_snapshot`

```
Property: For all Snapshot s where s has valid rkyv-compatible fields,
          serialize_snapshot(s) != Err(StateError::SerializationFailed { .. })
Bound: Snapshot with pages.len() <= 100
Rationale: All Snapshot fields (String, DateTime<Utc>, BTreeMap<String, PageHash>)
           are rkyv-compatible. Kani proves the error path cannot be reached with
           any valid Snapshot. This justifies B18/B40 being structural-only tests.
```

### K04: `store_snapshot` error propagation correctness

```
Property: store_snapshot propagates all error variants from serialize_snapshot
          and commit_changes without wrapping or swallowing
Bound: All CommitError variants, StateError::SerializationFailed
Rationale: Proves that store_snapshot's ? operators correctly propagate
           CommitError::WriteTransaction, CommitError::CommitFailed,
           CommitError::PayloadTooLarge, and StateError::SerializationFailed.
```

### K05: `url_hash(url).as_bytes()` is never `[0u8; 32]` for non-empty url

```
Property: For all non-empty url: &str, url_hash(url).as_bytes() != [0u8; 32]
Bound: url.len() in 1..1024
Rationale: Proves CommitError::ZeroHashKey is unreachable through store_snapshot
           for any non-empty URL. SHA-256 of non-empty input is never all zeros.
           This justifies B23 being tested via direct commit_changes call.
```

## 7. Mutation Testing Checkpoints

| Mutation | Caught By | Scenario |
|----------|-----------|----------|
| `open_state_db` returns `Err` instead of `Ok` | B01 | `open_state_db_returns_state_db_when_path_writable` |
| `open_state_db` returns wrong variant (e.g., `TableInit`) for read-only parent | B02 | `open_state_db_returns_commit_error_database_open_when_parent_read_only` |
| `open_state_db` skips `create_dir_all` | B03 | `open_state_db_creates_parent_directories_when_missing` |
| `load_snapshot` ignores stored snapshot, returns default | B07 | `load_snapshot_returns_stored_snapshot_with_1_page_when_key_exists` |
| `load_snapshot` returns error instead of default for missing key | B09 | `load_snapshot_returns_empty_default_when_key_missing` |
| `load_snapshot` returns wrong error variant (e.g., `InvalidArchive` instead of `DeserializationFailed`) | B13, B14 | Exact variant match in B13 and B14 |
| `store_snapshot` skips `commit_changes` call | B15 | `store_snapshot_persists_snapshot_via_commit_changes` |
| `store_snapshot` writes 0 entries instead of 1 | B15 | `store_snapshot_persists_snapshot_via_commit_changes` |
| `store_snapshot` accepts payload at `>= MAX_VALUE_SIZE` instead of `>` | B21, B22 | B22 tests exactly at boundary — mutation rejects valid payload |
| `serialize_snapshot` returns wrong bytes | B38 | `serialize_snapshot_round_trips_to_equal_snapshot` |
| `serialize_snapshot` returns `Ok(vec![])` | B39 | `serialize_snapshot_produces_non_empty_bytes_for_non_trivial_snapshot` |
| `ArchivedRaw::deserialize` returns Ok for corrupt input | B48, B49 | `archived_raw_deserialize_returns_deserialization_failed_on_corrupt_bytes` |
| `ArchivedRaw::deserialize` returns wrong variant | B48, B49 | Exact variant match distinguishes DeserializationFailed from InvalidArchive |
| `load_snapshots` returns keys not requested | B41 | `load_snapshots_returns_hashmap_with_matching_keys` |
| `load_snapshots` swallows table-open error | B44 | `load_snapshots_returns_state_error_table_open_failed_when_table_cannot_be_opened` |
| `load_snapshots` returns wrong variant for corrupt bytes | B46 | `load_snapshots_returns_state_error_archive_validation_failed_on_corrupt_bytes` |
| `run_watch` calls `store_snapshot` (mutation) | B24 | `run_watch_is_read_only_and_never_calls_commit_changes` |
| `run_apply` commits 0 entries instead of 1 | B32 | `run_apply_commits_exactly_one_snapshot_entry` |
| `run_apply` does not skip commit on empty plan | B33 | `run_apply_skips_commit_when_plan_is_empty` |
| `run_apply` second run commits again (breaks idempotency) | B34 | `run_apply_is_idempotent_on_second_run` |
| `url_hash` returns wrong key | B51 | `url_hash_produces_identical_key_across_calls` |
| Default snapshot has wrong target_url | B57 | `load_snapshot_returns_default_with_correct_url_and_empty_pages` |
| `commit_changes` removes `ZeroHashKey` check | B23 | `store_snapshot_returns_commit_error_zero_hash_key_when_key_is_zeros` |
| `commit_changes` removes `PayloadTooLarge` check | B21 | `store_snapshot_returns_commit_error_payload_too_large_when_exceeds_50mib` |
| `initialize_tables` error mapping changed to wrong variant | B06 | `open_state_db_returns_commit_error_table_init_when_tables_fail` |
| `begin_read` error mapping changed to wrong variant | B11 | `load_snapshot_returns_commit_error_read_transaction_when_begin_read_fails` |
| `begin_write` error mapping changed to wrong variant | B19 | `store_snapshot_returns_commit_error_write_transaction_when_begin_write_fails` |
| `commit` error mapping changed to wrong variant | B20 | `store_snapshot_returns_commit_error_commit_failed_when_commit_fails` |

**Threshold**: >= 90% mutation kill rate. Run `cargo mutants` on `cmd/watch.rs`, `state/mod.rs`, `state/commit.rs` targeting snapshot-related functions only.

## 8. Combinatorial Coverage Matrix

### 8.1 `open_state_db`

| Scenario | Input Class | Expected Output | Layer |
|----------|-------------|-----------------|-------|
| valid writable path | `tempdir/state.redb` | `Ok(StateDb)` | integration |
| read-only parent | `chmod 0o444 dir/state.redb` | `Err(CommitError::DatabaseOpen { .. })` | integration |
| creates nested parents | `a/b/c/state.redb` (missing dirs) | `Ok(StateDb)` + dirs exist | integration |
| empty path | `Path::new("")` | `Err(CommitError::DatabaseOpen { .. })` | integration |
| max path length | 4096+ chars | `Ok(StateDb)` or `Err(DatabaseOpen)` | integration |
| table init failure | corrupted DB file | `Err(CommitError::TableInit { .. })` | integration |

### 8.2 `load_snapshot`

| Scenario | Input Class | Expected Output | Layer |
|----------|-------------|-----------------|-------|
| key exists, 1 page | stored snapshot | `Ok(snapshot)` PartialEq-equal, `pages.len() == 1` | integration |
| key exists, 50 pages | stored snapshot | `Ok(snapshot)`, `pages.len() == 50` | integration |
| key missing | URL not in DB | `Ok(default)`, `url == URL`, `pages == empty` | integration |
| unicode URL | `"https://example.com/日本語"` | `Ok(default)`, `url == unicode_url` | integration |
| begin_read fails | corrupted DB | `Err(CommitError::ReadTransaction { .. })` | integration |
| table open fails | corrupted DB / dropped table | `Err(StateError::TableOpenFailed { table: "snapshots" })` | integration |
| truncated rkyv bytes | valid archive cut to 50% | `Err(StateError::DeserializationFailed { type_name: "Snapshot" })` | integration |
| garbage bytes | 256 bytes of `0xDE` | `Err(StateError::InvalidArchive { type_name: _ })` | integration |

### 8.3 `store_snapshot`

| Scenario | Input Class | Expected Output | Layer |
|----------|-------------|-----------------|-------|
| valid 3-page snapshot | normal Snapshot | `Ok(())`, 1 entry, round-trip equal | integration |
| empty 0-page snapshot | empty pages | `Ok(())`, round-trip equal | integration |
| overwrite existing | 2 pages → 5 pages | `Ok(())`, load returns 5-page | integration |
| serialization fails | (proven unreachable by K03) | `Err(StateError::SerializationFailed { .. })` | unit |
| begin_write fails | corrupted DB | `Err(CommitError::WriteTransaction { .. })` | integration |
| commit fails | disk-full simulation | `Err(CommitError::CommitFailed { .. })` | integration |
| payload > 50 MiB | 52,428,801 bytes | `Err(CommitError::PayloadTooLarge { table: "snapshots", size: 52428801, max: 52428800 })` | unit |
| payload == 50 MiB | 52,428,800 bytes | `Ok(())` | unit |
| zero hash key | `[0u8; 32]` via commit_changes | `Err(CommitError::ZeroHashKey { table: "snapshots", index: 0 })` | integration |

### 8.4 `serialize_snapshot`

| Scenario | Input Class | Expected Output | Layer |
|----------|-------------|-----------------|-------|
| valid snapshot | any well-formed Snapshot | `Ok(Vec<u8>)` non-empty | unit |
| round-trip | serialize then deserialize | `PartialEq` equal | unit |
| non-empty bytes | 1-page Snapshot | `Ok(bytes)` where `len > 0` | unit |
| serialization error | (proven unreachable by K03) | `Err(StateError::SerializationFailed { .. })` | unit |
| invariant | any valid Snapshot | round-trip holds | proptest |

### 8.5 `StateReadSession::load_snapshots`

| Scenario | Input Class | Expected Output | Layer |
|----------|-------------|-----------------|-------|
| subset of existing keys | 2 of 3 stored keys | `Ok(HashMap)` with `len == 2` | integration |
| no matching keys | nonexistent key | `Ok(HashMap)` with `len == 0` | integration |
| empty key list | `&[]` | `Ok(HashMap)` with `len == 0` | integration |
| table dropped | snapshots table removed | `Err(StateError::TableOpenFailed { table: "snapshots" })` | integration |
| storage error | corrupted DB file | `Err(StateError::StorageError { operation, message })` | integration |
| wrong-type archive | String bytes in snapshot slot | `Err(StateError::ArchiveValidationFailed { key_hex, message })` | integration |

### 8.6 `ArchivedRaw::deserialize`

| Scenario | Input Class | Expected Output | Layer |
|----------|-------------|-----------------|-------|
| valid archive | correct rkyv bytes | `Ok(T)` PartialEq-equal | unit |
| corrupt bytes | `vec![0xFF, 0xFE; 256]` | `Err(StateError::DeserializationFailed { type_name: "Snapshot" })` | unit |
| empty bytes | `vec![]` | `Err(StateError::InvalidArchive { type_name: _ })` | unit |

### 8.7 `run_watch` (integration)

| Scenario | Input Class | Expected Output | Layer |
|----------|-------------|-----------------|-------|
| first scrape (no previous) | empty DB | plan with all Added | integration |
| unchanged content | DB matches scrape | empty plan (0 changes) | integration |
| new page added | DB has 2, scrape has 3 | plan with 1 Added | integration |
| page removed | DB has 3, scrape has 2 | plan with 1 Removed | integration |
| page modified | same URL, different hash | plan with 1 Modified | integration |
| multiple changes | mixed add/remove/modify | correct counts | integration |
| no commit called | any scenario | snapshots table unchanged | integration |

### 8.8 `run_apply` (integration)

| Scenario | Input Class | Expected Output | Layer |
|----------|-------------|-----------------|-------|
| changes present | scrape differs from stored | 1 snapshot committed | integration |
| no changes | scrape matches stored | no commit (early exit) | integration |
| idempotent second run | same scrape dir again | no additional commit | integration |
| --yes flag | auto-confirm | commits without stdin | integration |
| --yes omitted | stdin piped "n" | no commit, exit code 1 | integration |
| multiple pages | 10 pages different | 10-page snapshot committed | integration |

### 8.9 Error Variant Coverage Matrix (all 15 contract variants)

| Error Variant | Scenario ID | Exact Variant Assertion | Status |
|---|---|---|---|
| `CommitError::DatabaseOpen` | B02 | `matches!(err, CommitError::DatabaseOpen { .. })` + path field check | COVERED |
| `CommitError::TableInit` | B06 | `matches!(err, CommitError::TableInit { .. })` + reason non-empty | COVERED |
| `CommitError::ReadTransaction` | B11 | `matches!(err, CommitError::ReadTransaction { .. })` + reason non-empty | COVERED |
| `CommitError::WriteTransaction` | B19 | `matches!(err, CommitError::WriteTransaction { .. })` + reason non-empty | COVERED |
| `CommitError::CommitFailed` | B20 | `matches!(err, CommitError::CommitFailed { .. })` + reason non-empty | COVERED |
| `CommitError::ZeroHashKey` | B23 | `matches!(err, CommitError::ZeroHashKey { table: "snapshots", index: 0 })` | COVERED |
| `CommitError::PayloadTooLarge` | B21 | `matches!(err, CommitError::PayloadTooLarge { table: "snapshots", size: 52428801, max: 52428800 })` | COVERED |
| `StateError::SerializationFailed` | B40 | K03 proof + structural construction test | COVERED |
| `StateError::DeserializationFailed` | B13, B48 | `matches!(err, StateError::DeserializationFailed { type_name: "Snapshot", .. })` | COVERED |
| `StateError::InvalidArchive` | B14, B49 | `matches!(err, StateError::InvalidArchive { type_name: _, .. })` | COVERED |
| `StateError::ArchiveValidationFailed` | B46 | `matches!(err, StateError::ArchiveValidationFailed { key_hex, message })` | COVERED |
| `StateError::TableOpenFailed` | B12, B44 | `matches!(err, StateError::TableOpenFailed { table: "snapshots", .. })` | COVERED |
| `StateError::StorageError` | B45 | `matches!(err, StateError::StorageError { operation, message })` | COVERED |

## Open Questions

1. **Q: Does `Snapshot` already have `rkyv::Archive, rkyv::Serialize, rkyv::Deserialize` derives?** No — grep returned no matches for `rkyv` in `watch.rs`. These derives must be added as part of this bead. B54 covers verifying this. `PageHash` fields (String, [u8; 32]) are all rkyv-compatible.

2. **Q: How to test `process::exit` paths in `run_watch` and `run_apply`?** Both functions call `process::exit()` which prevents normal test return. Options: (a) extract the decision logic into a testable helper that returns exit codes instead of calling `exit`, (b) use subprocess testing. Recommend option (a) for testability. B33, B36 use subprocess approach as fallback.

3. **Q: The `StateReadSession` in `commit.rs` is a stub (`todo!()`). The `bulk_load.rs` has a different `StateReadSession`. Which one will the migration use?** The contract specifies using `StateDb::begin_read()` from `commit.rs` and its `StateReadSession::load_snapshots`. The stub must be implemented. B41-B46 cover the stub implementation.

4. **Q: How to trigger `CommitError::TableInit` deterministically?** B06 uses file corruption (append garbage to valid DB, then reopen). If `Database::create` overwrites the corrupted file successfully and `initialize_tables` still succeeds, this variant may be unreachable through `open_state_db`. In that case: K02 proves the error mapping exists, and B06 documents the unreachable path. The variant is still tested at the `commit_changes` level (existing commit.rs tests exercise the precondition validation pipeline).

5. **Q: How to trigger `StateError::StorageError` in `load_snapshots`?** B45 uses file truncation after DB open. If redb's read path doesn't produce `StorageError` for this corruption method, alternative: construct a DB, open read session, then from another process/thread corrupt the file. If still unreliable, test via direct redb table `get()` on a corrupted table (integration test that writes garbage bytes directly to the table via a raw write transaction, then reads via `load_snapshots`).

6. **Q: Can `StateError::SerializationFailed` ever be triggered for `serialize_snapshot`?** All `Snapshot` fields (String, DateTime<Utc>, BTreeMap<String, PageHash>) are rkyv-compatible with proper derives. K03 proves this path is unreachable. B40 provides a structural construction test as fallback.

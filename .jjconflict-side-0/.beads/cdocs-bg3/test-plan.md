# Test Plan: cdocs-bg3 — Create redb Table Definitions for Raw State and Archived Outputs

## Summary

- **Behaviors identified:** 47 (B01–B47)
- **Trophy allocation:** 17 unit / 28 integration / 1 e2e / 7 static
- **Proptest invariants:** 5
- **Fuzz targets:** 1 in-scope + 1 deferred
- **Kani harnesses:** 4
- **Mutation kill target:** ≥90% (expected ≥95%)

**Rationale for allocation skew:** This bead is fundamentally a database schema bead — table definitions, Pod structs, and `initialize_tables`. The majority of guarantees are about redb table creation, type alignment, and idempotency. Integration tests dominate because every behavior that touches redb requires a real `Database` handle (real dependency, no mocks). Unit tests cover the pure Calc layer: `FileStateRaw`/`UrlStateRaw` Pod symmetry and zeroed-state validity. Static analysis catches compile-time guarantees (size_of, trait bounds, type alignment).

---

## 1. Behavior Inventory

### Pod Structs (Calc Layer — Pure)

| # | Behavior |
|---|----------|
| B01 | `FileStateRaw` occupies exactly 200 bytes when compiled |
| B02 | `UrlStateRaw` occupies exactly 120 bytes when compiled |
| B03 | `FileStateRaw` implements `bytemuck::Pod` and `bytemuck::Zeroable` |
| B04 | `UrlStateRaw` implements `bytemuck::Pod` and `bytemuck::Zeroable` |
| B05 | `FileStateRaw` zeroed state is valid Pod (all fields are defined bit patterns) |
| B06 | `UrlStateRaw` zeroed state is valid Pod (all fields are defined bit patterns) |
| B07 | `FileStateRaw` Pod read/write symmetry: `pod_read_unaligned(bytes_of(&s)) == s` for any `s` |
| B08 | `UrlStateRaw` Pod read/write symmetry: `pod_read_unaligned(bytes_of(&u)) == u` for any `u` |

### Table Definition Constants (Static / Compile-Time)

| # | Behavior |
|---|----------|
| B09 | All 8 table definition names are unique (no intra-set collisions) |
| B10 | All table names match architecture spec section 3 verbatim |
| B11 | Pod tables (`file_state`, `url_state`) use `TableDefinition<&str, &[u8]>` (compile-time type) |
| B12 | rkyv output tables use `TableDefinition<&[u8], &[u8]>` (compile-time type) |
| B13 | `metadata` table uses `TableDefinition<&str, &str>` (compile-time type) |
| B14 | New table names are disjoint from legacy `DocCache` table names (except shared `metadata`) |
| B15 | `metadata` table definition is identical to legacy `METADATA_TABLE` (same name, same types) |

### Table Accessor Functions (Unit)

| # | Behavior |
|---|----------|
| B16 | `file_state_table()` returns `TableDefinition` with name `"file_state"` |
| B17 | `url_state_table()` returns `TableDefinition` with name `"url_state"` |
| B18 | `analysis_outputs_table()` returns `TableDefinition` with name `"analysis_outputs"` |
| B19 | `transform_outputs_table()` returns `TableDefinition` with name `"transform_outputs"` |
| B20 | `chunk_outputs_table()` returns `TableDefinition` with name `"chunk_outputs"` |
| B21 | `scrape_outputs_table()` returns `TableDefinition` with name `"scrape_outputs"` |
| B22 | `snapshots_table()` returns `TableDefinition` with name `"snapshots"` |
| B23 | `metadata_table()` returns `TableDefinition` with name `"metadata"` |

### Database Initialization (Integration — Real redb)

| # | Behavior |
|---|----------|
| B24 | `initialize_tables` creates all 8 tables in a new database |
| B25 | `initialize_tables` is idempotent — second call on same database succeeds |
| B26 | `initialize_tables` returns `StateError::TableOpenFailed` with concrete table name when type-mismatch table pre-exists |
| B27 | `initialize_tables` returns `StateError::CommitFailed` when write transaction commit fails (Linux-specific) |
| B28 | Database reopened after `initialize_tables` — all 8 tables are accessible |
| B29 | Data written to tables survives across open/close/reopen cycles |
| B30 | Data written across 10 sequential open/write/close cycles all survive (Holzmann-compliant — no loops) |
| B31 | `initialize_tables` creates tables that coexist with legacy `DocCache` tables in same database |
| B43 | `initialize_tables` recovers a partially-initialized database (some tables missing) by creating only the missing tables |

### Key Validation (Integration — Real redb write path)

| # | Behavior |
|---|----------|
| B32 | Hash key of wrong length (≠ 32 bytes) rejected with `StateError::InvalidHashKeyLength` |
| B33 | Source path with leading `/` rejected with `StateError::InvalidSourcePath` |
| B34 | Source path with `..` component rejected with `StateError::InvalidSourcePath` |
| B35 | URL key without scheme rejected with `StateError::InvalidUrlKey` |
| B40 | URL key with trailing slash rejected with `StateError::InvalidUrlKey` (INV-11) |
| B41 | Source path with trailing whitespace rejected with `StateError::InvalidSourcePath` |
| B42 | Source path with null bytes rejected with `StateError::InvalidSourcePath` |
| B45 | URL key with query parameters accepted as valid |
| B46 | URL key with fragment accepted as valid |
| B47 | Source path of 500 characters accepted as valid and readable |

### Value Validation (Integration — Real redb read path)

| # | Behavior |
|---|----------|
| B36 | `file_state` value of wrong size returns `StateError::PodSizeMismatch` with table=`"file_state"`, expected=200 |
| B37 | `url_state` value of wrong size returns `StateError::PodSizeMismatch` with table=`"url_state"`, expected=120 |
| B38 | Invalid rkyv bytes in output table return `StateError::InvalidArchive` |
| B39 | Key not found in table returns `StateError::KeyNotFound` with correct table name |

### Shared Table Co-Access (Integration)

| # | Behavior |
|---|----------|
| B44 | Both legacy `DocCache` and new `StateDb` can read/write the shared `metadata` table without conflict |

---

## 2. Trophy Allocation

| Layer | Count | Behaviors | Justification |
|-------|-------|-----------|---------------|
| **Static** | 7 | B01–B04, B11–B13 | Compile-time: `size_of` asserts, trait bound checks, type alignment. Zero runtime cost. Proven by compilation success. |
| **Unit** | 17 | B05–B10, B14–B23, `PodCastFailed` error test | Pure Calc layer: Pod symmetry, accessor return values, name literal checks, error construction. No I/O. Deterministic. |
| **Integration** | 28 | B24–B29, B31–B47, `OpenFailed`, `ReadTxFailed`, `WriteTxFailed`, `DeserializationFailed`, `StorageError` | Real redb via `tempfile::TempDir`. Tests state not interactions. Every scenario uses a fresh database. |
| **E2E** | 1 | B30 | Full lifecycle stress test: open → write → close → reopen → verify × 10 cycles. System-level durability guarantee. |
| **Total** | **53** | | |

**Ratio:** ~32% unit, ~53% integration, ~2% e2e, ~13% static. Integration-heavy as expected for a schema bead. Every integration test uses a real `tempfile`-backed redb database — no mocks.

**Classification consistency note:** B11/B12/B13 are classified as **Static** everywhere — they are compile-time type checks proven by compilation success. If the wrong generic types are used, integration tests that call `open_table(ANALYSIS_OUTPUTS_TABLE)` etc. will fail to compile, providing the same guarantee at zero test-execution cost.

---

## 3. BDD Scenarios

### Behavior B01: FileStateRaw occupies exactly 200 bytes

```
Given: FileStateRaw is defined with #[repr(C)] and explicit _reserved: [u8; 32]
When:  std::mem::size_of::<FileStateRaw>() is evaluated
Then:  result == 200
```

**Test function:** `fn file_state_raw_size_is_200_bytes()`
**Layer:** Static (compile-time assert via `const _ASSERT: () = assert!(size_of::<FileStateRaw>() == 200);`)

---

### Behavior B02: UrlStateRaw occupies exactly 120 bytes

```
Given: UrlStateRaw is defined with #[repr(C)] and explicit _reserved: [u8; 46]
When:  std::mem::size_of::<UrlStateRaw>() is evaluated
Then:  result == 120
```

**Test function:** `fn url_state_raw_size_is_120_bytes()`
**Layer:** Static (compile-time assert)

---

### Behavior B03: FileStateRaw implements Pod and Zeroable

```
Given: FileStateRaw derives bytemuck::Pod and bytemuck::Zeroable
When:  a function requiring T: Pod + Zeroable is instantiated with FileStateRaw
Then:  compilation succeeds (trait bound satisfied)
```

**Test function:** `fn file_state_raw_satisfies_pod_zeroable_bounds()`
**Layer:** Static (trait bound check at compile time — if it compiles, it passes)

---

### Behavior B04: UrlStateRaw implements Pod and Zeroable

```
Given: UrlStateRaw derives bytemuck::Pod and bytemuck::Zeroable
When:  a function requiring T: Pod + Zeroable is instantiated with UrlStateRaw
Then:  compilation succeeds
```

**Test function:** `fn url_state_raw_satisfies_pod_zeroable_bounds()`
**Layer:** Static

---

### Behavior B05: FileStateRaw zeroed state is valid Pod

```
Given: FileStateRaw with all fields zeroed (bytemuck::Zeroable)
When:  pod_read_unaligned::<FileStateRaw>(bytes_of(&zeroed)) is called
Then:  result.content_hash == [0u8; 32]
And:   result.config_hash == [0u8; 32]
And:   result.analysis_hash == [0u8; 32]
And:   result.transform_hash == [0u8; 32]
And:   result.chunk_hash == [0u8; 32]
And:   result.last_processed_secs == 0
And:   result._reserved == [0u8; 32]
```

**Test function:** `fn file_state_raw_zeroed_is_valid_pod()`
**Layer:** Unit

---

### Behavior B06: UrlStateRaw zeroed state is valid Pod

```
Given: UrlStateRaw with all fields zeroed
When:  pod_read_unaligned::<UrlStateRaw>(bytes_of(&zeroed)) is called
Then:  result.content_hash == [0u8; 32]
And:   result.url_hash == [0u8; 32]
And:   result.last_fetched_secs == 0
And:   result.status_code == 0
And:   result._reserved == [0u8; 46]
```

**Test function:** `fn url_state_raw_zeroed_is_valid_pod()`
**Layer:** Unit

---

### Behavior B07: FileStateRaw Pod read/write symmetry

```
Given: a FileStateRaw with arbitrary non-zero field values
When:  bytes_of(&state) then pod_read_unaligned::<FileStateRaw>() is applied
Then:  the round-tripped value equals the original state (field-by-field)
```

**Test function:** `fn file_state_raw_pod_roundtrip_returns_original()`
**Layer:** Unit

---

### Behavior B08: UrlStateRaw Pod read/write symmetry

```
Given: a UrlStateRaw with arbitrary non-zero field values
When:  bytes_of(&state) then pod_read_unaligned::<UrlStateRaw>() is applied
Then:  the round-tripped value equals the original state (field-by-field)
```

**Test function:** `fn url_state_raw_pod_roundtrip_returns_original()`
**Layer:** Unit

---

### Behavior B09: All 8 table definition names are unique

```
Given: the 8 table definition constants
When:  their names are collected into a HashSet
Then:  the set has exactly 8 elements (no duplicates)
```

**Test function:** `fn table_definition_names_are_all_unique()`
**Layer:** Unit (const string comparison)

---

### Behavior B10: Table names match architecture spec section 3 verbatim

```
Given: the table definition constants
When:  each definition's name is compared to the architecture spec string
Then:  names are exactly: "file_state", "analysis_outputs", "transform_outputs",
       "chunk_outputs", "url_state", "scrape_outputs", "snapshots", "metadata"
```

**Test function:** `fn table_names_match_architecture_spec_exactly()`
**Layer:** Unit (assert_eq! on const string slices)

---

### Behavior B11: Pod tables use TableDefinition<&str, &[u8]>

```
Given: FILE_STATE_TABLE and URL_STATE_TABLE
When:  integration test code calls db.begin_write()?.open_table(FILE_STATE_TABLE)
       and db.begin_write()?.open_table(URL_STATE_TABLE)
Then:  compilation succeeds only if both are TableDefinition<&str, &[u8]>
```

**Test function:** Covered implicitly by B24 and all integration tests that open these tables. Explicit compile-time proof: if the type is wrong, B24 fails to compile.
**Layer:** Static (type inference — compilation is the proof)

---

### Behavior B12: rkyv output tables use TableDefinition<&[u8], &[u8]>

```
Given: ANALYSIS_OUTPUTS_TABLE, TRANSFORM_OUTPUTS_TABLE, CHUNK_OUTPUTS_TABLE,
       SCRAPE_OUTPUTS_TABLE, SNAPSHOTS_TABLE
When:  integration test code calls open_table() on each
Then:  compilation succeeds only if all are TableDefinition<&[u8], &[u8]>
```

**Test function:** Covered implicitly by B24 and all integration tests that open these tables.
**Layer:** Static (type inference — compilation is the proof)

---

### Behavior B13: metadata table uses TableDefinition<&str, &str>

```
Given: METADATA_TABLE
When:  integration test code calls db.begin_write()?.open_table(METADATA_TABLE)
Then:  compilation succeeds only if it is TableDefinition<&str, &str>
```

**Test function:** Covered implicitly by B24 and all integration tests that open the metadata table.
**Layer:** Static (type inference — compilation is the proof)

---

### Behavior B14: New table names disjoint from legacy (except metadata)

```
Given: new table names: {"file_state", "analysis_outputs", "transform_outputs",
       "chunk_outputs", "url_state", "scrape_outputs", "snapshots", "metadata"}
And:  legacy table names: {"documents", "scrape", "transforms", "snapshots",
       "analysis", "chunks", "metadata"}
When:  the intersection is computed
Then:  intersection == {"metadata"} (only metadata is shared, by design — INV-09)
```

**Test function:** `fn new_table_names_disjoint_from_legacy_except_metadata()`
**Layer:** Unit

---

### Behavior B15: metadata table definition identical to legacy

```
Given: the new METADATA_TABLE and the legacy DocCache METADATA_TABLE
When:  their names are compared
Then:  both have name "metadata"
And:   both have type TableDefinition<&str, &str>
```

**Test function:** `fn metadata_table_definition_identical_to_legacy()`
**Layer:** Unit

---

### Behavior B16: file_state_table() returns definition named "file_state"

```
Given: the file_state_table() accessor
When:  called
Then:  returned TableDefinition has name "file_state"
```

**Test function:** `fn file_state_table_returns_definition_named_file_state()`
**Layer:** Unit

---

### Behavior B17: url_state_table() returns definition named "url_state"

```
Given: the url_state_table() accessor
When:  called
Then:  returned TableDefinition has name "url_state"
```

**Test function:** `fn url_state_table_returns_definition_named_url_state()`
**Layer:** Unit

---

### Behavior B18: analysis_outputs_table() returns definition named "analysis_outputs"

```
Given: the analysis_outputs_table() accessor
When:  called
Then:  returned TableDefinition has name "analysis_outputs"
```

**Test function:** `fn analysis_outputs_table_returns_definition_named_analysis_outputs()`
**Layer:** Unit

---

### Behavior B19: transform_outputs_table() returns definition named "transform_outputs"

```
Given: the transform_outputs_table() accessor
When:  called
Then:  returned TableDefinition has name "transform_outputs"
```

**Test function:** `fn transform_outputs_table_returns_definition_named_transform_outputs()`
**Layer:** Unit

---

### Behavior B20: chunk_outputs_table() returns definition named "chunk_outputs"

```
Given: the chunk_outputs_table() accessor
When:  called
Then:  returned TableDefinition has name "chunk_outputs"
```

**Test function:** `fn chunk_outputs_table_returns_definition_named_chunk_outputs()`
**Layer:** Unit

---

### Behavior B21: scrape_outputs_table() returns definition named "scrape_outputs"

```
Given: the scrape_outputs_table() accessor
When:  called
Then:  returned TableDefinition has name "scrape_outputs"
```

**Test function:** `fn scrape_outputs_table_returns_definition_named_scrape_outputs()`
**Layer:** Unit

---

### Behavior B22: snapshots_table() returns definition named "snapshots"

```
Given: the snapshots_table() accessor
When:  called
Then:  returned TableDefinition has name "snapshots"
```

**Test function:** `fn snapshots_table_returns_definition_named_snapshots()`
**Layer:** Unit

---

### Behavior B23: metadata_table() returns definition named "metadata"

```
Given: the metadata_table() accessor
When:  called
Then:  returned TableDefinition has name "metadata"
```

**Test function:** `fn metadata_table_returns_definition_named_metadata()`
**Layer:** Unit

---

### Behavior B24: initialize_tables creates all 8 tables in a new database

```
Given: a fresh redb Database at a tempfile path (no pre-existing tables)
When:  initialize_tables(&db) is called
Then:  result == Ok(())
And:   opening each of the 8 table definitions via read_tx.open_table() succeeds
       (FILE_STATE_TABLE, URL_STATE_TABLE, ANALYSIS_OUTPUTS_TABLE,
        TRANSFORM_OUTPUTS_TABLE, CHUNK_OUTPUTS_TABLE, SCRAPE_OUTPUTS_TABLE,
        SNAPSHOTS_TABLE, METADATA_TABLE)
```

**Test function:** `fn initialize_tables_creates_all_8_tables_on_fresh_db()`
**Layer:** Integration

---

### Behavior B25: initialize_tables is idempotent

```
Given: a fresh redb Database where initialize_tables has already been called successfully
When:  initialize_tables(&db) is called a second time
Then:  result == Ok(())
And:   all 8 tables remain accessible
And:   any data written between the two calls is still present after the second call
```

**Test function:** `fn initialize_tables_is_idempotent_on_second_call()`
**Layer:** Integration

---

### Behavior B26: initialize_tables returns TableOpenFailed with concrete table name

```
Given: a database where a table named "file_state" already exists with a CONFLICTING
       type definition — i.e., pre-created via:
         let conflicting_def: TableDefinition<&str, &str> =
             TableDefinition::new("file_state");
         let mut tx = db.begin_write().unwrap();
         let mut t = tx.open_table(conflicting_def).unwrap();
         t.insert("dummy", "wrong_type").unwrap();
         tx.commit().unwrap();
When:  initialize_tables(&db) is called (which tries to open "file_state" as
       TableDefinition<&str, &[u8]>)
Then:  result == Err(StateError::TableOpenFailed { table, message })
And:   table == "file_state"
And:   message.contains("type")
```

**Test function:** `fn initialize_tables_returns_table_open_failed_with_table_name_on_type_conflict()`
**Layer:** Integration
**Determinism:** 100% deterministic. Type-mismatch tables cause redb to return a deterministic error. No timing dependency.

---

### Behavior B27: initialize_tables returns CommitFailed when commit fails

```
Given: a database opened on a Linux tmpfs path
And:   a write transaction is started that writes to all 8 tables
When:  the database file's parent directory is made read-only via
       std::fs::set_permissions(dir, Permissions::from_mode(0o555))
       BEFORE tx.commit() is called
Then:  result == Err(StateError::CommitFailed { message })
And:   message.contains("write") or message.contains("permission")
```

**Test function:** `fn initialize_tables_returns_commit_failed_on_readonly_dir()`
**Layer:** Integration
**Platform:** `#[cfg(target_os = "linux")]` — filesystem permission semantics differ across platforms. Non-Linux CI skips this test.
**Determinism:** Deterministic on Linux. chmod on tmpfs takes effect immediately on the next write syscall.

---

### Behavior B28: Database reopened — all 8 tables survive

```
Given: a database where initialize_tables was called, then the Database handle is dropped
When:  a new Database::create() is opened on the same path
And:   a read transaction is started
Then:  all 8 tables can be opened via their definitions
And:   each table has 0 entries (fresh state)
```

**Test function:** `fn all_8_tables_survive_database_reopen()`
**Layer:** Integration

---

### Behavior B29: Data written survives across open/close/reopen cycles

```
Given: a database with initialized tables
And:   a FileStateRaw value written to file_state table with key "concept/test.md"
When:  the database is closed and reopened
And:   the value at key "concept/test.md" is read
Then:  the bytes read have length 200
And:   pod_read_unaligned produces a FileStateRaw equal to the original
```

**Test function:** `fn written_data_survives_across_reopen_cycle()`
**Layer:** Integration

---

### Behavior B30: Data survives 10 sequential open/write/close cycles

```
Given: a single tempfile database path
When:  10 sequential cycles of: open → write unique key/value → close → verify are performed
Then:  after the 10th cycle, all 10 key/value pairs are readable
And:   each value equals its originally written value exactly
```

**Test function:** `fn data_survives_ten_sequential_open_write_close_cycles()`
**Layer:** E2E (full lifecycle stress test)

**Holzmann Rule 2 Compliance:** This test MUST be implemented as 10 inline open/write/close/verify blocks — NOT a `for` loop. Each cycle is a distinct step in the test function body. Alternatively, implement as 10 `#[case]` attributes via `rstest`, where each case is a single cycle that builds on the cumulative state. The test function body contains zero loops.

---

### Behavior B31: New tables coexist with legacy DocCache tables

```
Given: a database where legacy DocCache tables (documents, scrape, transforms, etc.) exist
And:   the new state tables are initialized via initialize_tables
When:  both sets of tables are opened in a single read transaction
Then:  legacy tables are accessible via legacy definitions
And:   new tables are accessible via new definitions
And:   data in legacy tables is not corrupted
```

**Test function:** `fn new_state_tables_coexist_with_legacy_doc_cache_tables()`
**Layer:** Integration

---

### Behavior B32: Hash key of wrong length rejected

```
Given: a database with initialized tables
And:   a write transaction is open on analysis_outputs table
When:  a key of 16 bytes (not 32) is used for insert
Then:  result == Err(StateError::InvalidHashKeyLength { actual: 16 })
```

**Test function:** `fn hash_key_wrong_length_returns_invalid_hash_key_length()`
**Layer:** Integration

---

### Behavior B33: Source path with leading / rejected

```
Given: a database with initialized tables
When:  a key starting with "/" is written to file_state table (e.g., "/absolute/path.md")
Then:  result == Err(StateError::InvalidSourcePath { reason })
And:   reason.contains("leading")
```

**Test function:** `fn source_path_with_leading_slash_returns_invalid_source_path()`
**Layer:** Integration

---

### Behavior B34: Source path with .. component rejected

```
Given: a database with initialized tables
When:  a key containing ".." is written to file_state table (e.g., "foo/../bar.md")
Then:  result == Err(StateError::InvalidSourcePath { reason })
And:   reason.contains("..")
```

**Test function:** `fn source_path_with_dot_dot_returns_invalid_source_path()`
**Layer:** Integration

---

### Behavior B35: URL key without scheme rejected

```
Given: a database with initialized tables
When:  a key without "://" is written to url_state table (e.g., "example.com/page")
Then:  result == Err(StateError::InvalidUrlKey { reason })
And:   reason.contains("scheme")
```

**Test function:** `fn url_key_without_scheme_returns_invalid_url_key()`
**Layer:** Integration

---

### Behavior B36: file_state value wrong size returns PodSizeMismatch

```
Given: a database with initialized tables
And:   raw bytes of length 199 written directly to file_state table (bypassing Pod path)
When:  the value is read and its size is validated
Then:  result == Err(StateError::PodSizeMismatch { table: "file_state", expected: 200, actual: 199 })
```

**Test function:** `fn file_state_wrong_value_size_returns_pod_size_mismatch()`
**Layer:** Integration

---

### Behavior B37: url_state value wrong size returns PodSizeMismatch

```
Given: a database with initialized tables
And:   raw bytes of length 121 written directly to url_state table
When:  the value is read and its size is validated
Then:  result == Err(StateError::PodSizeMismatch { table: "url_state", expected: 120, actual: 121 })
```

**Test function:** `fn url_state_wrong_value_size_returns_pod_size_mismatch()`
**Layer:** Integration

---

### Behavior B38: Invalid rkyv bytes return InvalidArchive

```
Given: a database with initialized tables
And:   arbitrary garbage bytes (e.g., [0xDE, 0xAD, 0xBE, 0xEF]) written to analysis_outputs table
When:  the bytes are passed to OwnedArchive::<Analysis>::from_bytes (or equivalent rkyv access)
Then:  result == Err(StateError::InvalidArchive { type_name: "Analysis", message })
And:   message.len() > 0
```

**Test function:** `fn invalid_rkyv_bytes_return_invalid_archive()`
**Layer:** Integration

---

### Behavior B39: Key not found returns KeyNotFound

```
Given: a database with initialized tables (empty)
When:  a lookup is performed for hash key [1u8; 32] in analysis_outputs table
Then:  result == Err(StateError::KeyNotFound { table: "analysis_outputs" })
```

**Test function:** `fn missing_key_returns_key_not_found_with_table_name()`
**Layer:** Integration

---

### Behavior B40: URL key with trailing slash rejected (INV-11)

```
Given: a database with initialized tables
When:  a URL key with trailing slash is written to url_state table
       (e.g., "https://example.com/api/")
Then:  result == Err(StateError::InvalidUrlKey { reason })
And:   reason.contains("trailing")
```

**Test function:** `fn url_key_with_trailing_slash_returns_invalid_url_key()`
**Layer:** Integration

---

### Behavior B41: Source path with trailing whitespace rejected

```
Given: a database with initialized tables
When:  a key with trailing whitespace is written to file_state table (e.g., "test.md ")
Then:  result == Err(StateError::InvalidSourcePath { reason })
And:   reason.contains("whitespace")
```

**Test function:** `fn source_path_with_trailing_whitespace_returns_invalid_source_path()`
**Layer:** Integration

---

### Behavior B42: Source path with null bytes rejected

```
Given: a database with initialized tables
When:  a key containing a null byte is written to file_state table (e.g., "test\x00.md")
Then:  result == Err(StateError::InvalidSourcePath { reason })
And:   reason.contains("null")
```

**Test function:** `fn source_path_with_null_bytes_returns_invalid_source_path()`
**Layer:** Integration

---

### Behavior B43: initialize_tables recovers from partial initialization

```
Given: a database where only 4 of 8 tables have been created
       (simulated by: calling initialize_tables, then manually deleting
        4 table definitions by writing to a fresh db with only some tables)
When:  initialize_tables(&db) is called on the partially-initialized database
Then:  result == Ok(())
And:   all 8 tables are accessible after the call
And:   the 4 pre-existing tables still contain their original data
```

**Test function:** `fn initialize_tables_recovers_from_partial_initialization()`
**Layer:** Integration
**Implementation note:** Simulate partial init by creating a database, writing to only 4 table definitions via a custom subset of `open_table` calls, then calling `initialize_tables` to create the missing 4.

---

### Behavior B44: Shared metadata table allows co-access from legacy and new

```
Given: a database with both legacy DocCache and new StateDb tables initialized
And:   legacy code writes ("legacy_key", "legacy_value") to metadata table
       via DocCache's METADATA_TABLE definition
When:  new code reads from metadata table via StateDb's METADATA_TABLE definition
Then:  value at "legacy_key" == "legacy_value"
And:   new code writes ("new_key", "new_value") to metadata table
And:   legacy code reads "new_key" and gets "new_value"
```

**Test function:** `fn shared_metadata_table_allows_co_access_from_legacy_and_new()`
**Layer:** Integration

---

### Behavior B45: URL key with query parameters accepted as valid

```
Given: a database with initialized tables
When:  a URL key with query parameters is written to url_state table
       (e.g., "https://example.com/api?query=1")
Then:  write succeeds: result == Ok(())
And:   reading back the key returns the exact URL string
```

**Test function:** `fn url_key_with_query_params_accepted_as_valid()`
**Layer:** Integration

---

### Behavior B46: URL key with fragment accepted as valid

```
Given: a database with initialized tables
When:  a URL key with a fragment is written to url_state table
       (e.g., "https://example.com/docs#section")
Then:  write succeeds: result == Ok(())
And:   reading back the key returns the exact URL string
```

**Test function:** `fn url_key_with_fragment_accepted_as_valid()`
**Layer:** Integration

---

### Behavior B47: Source path of 500 characters accepted as valid

```
Given: a database with initialized tables
When:  a source path key of exactly 500 characters is written to file_state table
       (e.g., "concept/" + "a" × 484 + ".md" = 500 chars total)
Then:  write succeeds: result == Ok(())
And:   reading back the key returns the exact 500-character path
```

**Test function:** `fn very_long_source_path_accepted_as_valid()`
**Layer:** Integration

---

### Error Variant Coverage (every variant has a test scenario)

| StateError Variant | Scenario(s) | Behavior(s) |
|---|---|---|
| `OpenFailed` | Database path is a directory | Dedicated test below |
| `ReadTransactionFailed` | Read tx attempted while corrupted state | Dedicated test below |
| `WriteTransactionFailed` | Second write tx while first is held | Dedicated test below |
| `PodSizeMismatch` | Wrong byte count in Pod table value | B36, B37 |
| `PodCastFailed` | Truncated buffer passed to pod_read_unaligned | Dedicated test below |
| `InvalidArchive` | Garbage bytes in rkyv table | B38 |
| `DeserializationFailed` | Valid rkyv archive of wrong type | Dedicated test below |
| `SerializationFailed` | No deterministic trigger — construction-only verification | Dedicated note below |
| `TableOpenFailed` | Type-mismatch table pre-exists | B26 |
| `KeyNotFound` | Lookup of nonexistent key | B39 |
| `StorageError` | redb I/O error during operation | Dedicated test below (Linux-only) |
| `CommitFailed` | Write tx commit on read-only directory | B27 |
| `InvalidHashKeyLength` | Key ≠ 32 bytes in hash-keyed table | B32 |
| `InvalidSourcePath` | Leading `/`, `..`, trailing whitespace, or null bytes | B33, B34, B41, B42 |
| `InvalidUrlKey` | No scheme or trailing slash | B35, B40 |

**Additional error-variant-specific scenarios:**

---

### StateError::OpenFailed

```
Given: a path that points to an existing directory (not a file)
When:  StateDb::open() is called with that directory path
Then:  result == Err(StateError::OpenFailed { path, source })
And:   path.display().to_string() == the directory path provided
And:   source.contains("open")
```

**Test function:** `fn open_state_db_returns_open_failed_when_path_is_directory()`
**Layer:** Integration

---

### StateError::ReadTransactionFailed

```
Given: a database where a write transaction is held open on a separate handle
When:  a second database handle attempts begin_read()
Then:  result == Err(StateError::ReadTransactionFailed { message })
And:   message.contains("transaction")
```

**Test function:** `fn read_transaction_returns_failed_when_write_held()`
**Layer:** Integration

---

### StateError::WriteTransactionFailed

```
Given: a database where an existing write transaction is already in progress
When:  a second begin_write() is attempted on the same database handle
Then:  result == Err(StateError::WriteTransactionFailed { message })
And:   message.contains("write")
```

**Test function:** `fn write_transaction_returns_failed_when_concurrent_write_held()`
**Layer:** Integration

---

### StateError::PodCastFailed

```
Given: a byte buffer of 3 bytes (too small for FileStateRaw which requires 200)
When:  pod_read_unaligned::<FileStateRaw> is attempted on those bytes
Then:  result returns Err(StateError::PodCastFailed { type_name, message })
And:   type_name == "FileStateRaw"
And:   message.contains("size")
```

**Test function:** `fn pod_cast_returns_failed_on_truncated_buffer()`
**Layer:** Unit

---

### StateError::DeserializationFailed

```
Given: a valid rkyv archive of type String written to transform_outputs table
When:  an attempt is made to deserialize those bytes as Analysis (wrong type)
Then:  result == Err(StateError::DeserializationFailed { type_name, message })
And:   type_name == "Analysis"
```

**Test function:** `fn rkyv_wrong_type_deserialization_returns_deserialization_failed()`
**Layer:** Integration
**Deferred:** This test requires rkyv derives on `Analysis` (out of scope per contract Non-goal #1).
Implementation deferred to the rkyv-derive bead. For THIS bead, verify the error variant
construction only: create `StateError::DeserializationFailed { type_name: "Analysis", message: "..." }`
and assert variant match via pattern matching.

---

### StateError::SerializationFailed — Construction-Only Verification

**No deterministic natural trigger exists** for `rkyv::to_bytes` failure with the domain types
in this bead. `rkyv::to_bytes` allocates and serializes; it only fails on allocation failure
(OOM) or internal bugs, neither of which can be triggered deterministically in a test.

**Approach:** Verify the error variant's structure via construction:
```rust
let err = StateError::SerializationFailed {
    type_name: "Analysis",
    message: "out of memory".to_string(),
};
assert!(matches!(err, StateError::SerializationFailed { .. }));
assert_eq!(err.to_string(), "rkyv serialization failed for type Analysis: out of memory");
```

This is NOT a behavior test — it is an error-taxonomy completeness check. The error variant
exists for future-proofing. When rkyv derives are added and real serialization paths exist,
the rkyv-derive bead should add an integration test that triggers this variant if a path is found.

**Test function:** `fn serialization_failed_error_construction_matches_display_format()`
**Layer:** Unit

---

### StateError::StorageError

```
Given: a database opened on a Linux tmpfs path
And:   a write transaction is in progress
When:  the database file is deleted via std::fs::remove_file(db_path)
       and a subsequent table insert is attempted
Then:  result == Err(StateError::StorageError { operation, message })
And:   operation == "insert"
And:   message.len() > 0
```

**Test function:** `fn redb_io_error_returns_storage_error_with_operation_name()`
**Layer:** Integration
**Platform:** `#[cfg(target_os = "linux")]` — file deletion semantics while a file handle is open differ across platforms. On Linux, `remove_file` unlinks the name but the inode remains open; subsequent writes may fail with EIO. On macOS/Windows, behavior is undefined. Non-Linux CI skips this test.
**Determinism:** Deterministic on Linux. The `operation` field is set by our code, not redb, so we assert the exact string we set (e.g., `"insert"`).

---

## 4. Proptest Invariants

### Proptest 1: FileStateRaw Pod Round-Trip

```
Invariant: For any FileStateRaw, pod_read_unaligned(bytes_of(&s)) == s.
           The byte representation is fully determined by field values. (INV-06)
Strategy:  proptest::array::uniform32(0u8..=255) for each of the 5 hash fields,
           any u64 for last_processed_secs, any [u8; 32] for _reserved.
           Assemble into FileStateRaw.
Anti-invariant: Byte arrays of length != 200 must never be interpretable as
                valid FileStateRaw (PodSizeMismatch).
```

### Proptest 2: UrlStateRaw Pod Round-Trip

```
Invariant: For any UrlStateRaw, pod_read_unaligned(bytes_of(&u)) == u. (INV-06)
Strategy:  proptest::array::uniform32(0u8..=255) for content_hash and url_hash,
           any u64 for last_fetched_secs, any u16 for status_code, any [u8; 46] for _reserved.
           Assemble into UrlStateRaw.
Anti-invariant: Byte arrays of length != 120 must never be interpretable as
                valid UrlStateRaw (PodSizeMismatch).
```

### Proptest 3: FileStateRaw Byte Layout Consistency

```
Invariant: For any FileStateRaw, bytes_of(&state)[0..32] == state.content_hash,
           bytes_of(&state)[32..64] == state.config_hash,
           bytes_of(&state)[64..96] == state.analysis_hash,
           bytes_of(&state)[96..128] == state.transform_hash,
           bytes_of(&state)[128..160] == state.chunk_hash,
           bytes_of(&state)[160..168] == state.last_processed_secs.to_le_bytes(),
           bytes_of(&state)[168..200] == state._reserved.
Strategy:  Same as Proptest 1.
Anti-invariant: Mutating any single byte in the 200-byte representation changes
                the deserialized struct (no byte is ignored).
```

### Proptest 4: UrlStateRaw Byte Layout Consistency

```
Invariant: For any UrlStateRaw, bytes_of(&state)[0..32] == state.content_hash,
           bytes_of(&state)[32..64] == state.url_hash,
           bytes_of(&state)[64..72] == state.last_fetched_secs.to_le_bytes(),
           bytes_of(&state)[72..74] == state.status_code.to_le_bytes(),
           bytes_of(&state)[74..120] == state._reserved.
Strategy:  Same as Proptest 2.
Anti-invariant: Mutating any single byte in the 120-byte representation changes
                the deserialized struct.
```

### Proptest 5: Pod Zeroed → Round-Trip → Zeroed

```
Invariant: For both FileStateRaw and UrlStateRaw, zeroed() → bytes_of() →
           pod_read_unaligned() → == zeroed(). The zero state is a fixed point.
Strategy:  Generate any valid FileStateRaw/UrlStateRaw, verify that
           replacing all fields with zero-equivalent and round-tripping
           produces all-zero bytes.
Anti-invariant: A struct with any non-zero field must NOT produce all-zero bytes.
```

---

## 5. Fuzz Targets

### Fuzz Target 1: Pod Byte Interpretation

```
Input type:  Arbitrary [u8; 200] and [u8; 120] byte arrays
Risk:        Panic in pod_read_unaligned if alignment violated or buffer too short.
             Logic error if round-trip fails. Bytemuck is unsafe internally.
Corpus seeds:
  - All-zero bytes (200 and 120 bytes)
  - All-0xFF bytes
  - Alternating 0x00/0xFF pattern
  - Random valid FileStateRaw/UrlStateRaw serialized via bytes_of
Fuzz harness logic:
  1. Given 200 bytes, cast to FileStateRaw via pod_read_unaligned
  2. Serialize back via bytes_of
  3. Assert original bytes == round-tripped bytes
  4. Same for UrlStateRaw with 120 bytes
  5. For any other length: assert that the validation function returns
     Err(PodSizeMismatch { table: _, expected: _, actual: _ })
```

### Fuzz Target 2: rkyv Archive Validation (DEFERRED)

```
Input type:  Arbitrary &[u8] (variable length byte slice)
Risk:        Panic in rkyv::access (bytecheck). OOM on huge input. Logic error in
             archive validation that could allow corrupt data to pass.
Corpus seeds:
  - Valid rkyv archive of each output type (Analysis, String, Vec<Chunk>,
    ScrapedPage, Snapshot) — these require rkyv derives (out of scope for this bead,
    so seed generation is deferred to the rkyv-derive bead)
  - Empty bytes []
  - Single byte [0x00]
  - 4 bytes [0xDE, 0xAD, 0xBE, 0xEF]
  - 32 bytes of zeros (same length as a hash key but not a valid archive)
  - 200 bytes of zeros (same as FileStateRaw size)
Fuzz harness logic:
  1. Given arbitrary bytes, attempt OwnedArchive::<T>::from_bytes() for each T
  2. If it succeeds, verify deserialize() completes without panic
  3. If it fails, verify it returns InvalidArchive (not a panic)
Note: This target is deferred until rkyv derives exist. Listed here for planning.
```

---

## 6. Kani Harnesses

### Kani Harness 1: FileStateRaw Size Proof

```
Property: std::mem::size_of::<FileStateRaw>() == 200 for all possible layouts
Bound:    N/A — single static assertion
Rationale: If the struct layout changes (field added/removed, type changed),
           the assertion catches it at verification time. More reliable than
           a runtime test that only runs in CI.
```

### Kani Harness 2: UrlStateRaw Size Proof

```
Property: std::mem::size_of::<UrlStateRaw>() == 120 for all possible layouts
Bound:    N/A — single static assertion
Rationale: Same as above. Catches any layout regression.
```

### Kani Harness 3: FileStateRaw No Padding With Undefined Bytes

```
Property: For any FileStateRaw value, every byte in the 200-byte representation
          is determined by the struct's field values (no uninitialized padding).
          Equivalently: two FileStateRaw values with identical fields produce
          identical byte representations under bytes_of().
Bound:    Exhaustive over all field combinations is infeasible — use Kani's
          symbolic execution to prove that no byte in the representation depends
          on anything other than the field values.
Rationale: Padding bytes with undefined values violate Pod safety. Kani can
           formally prove there are no such bytes.
```

### Kani Harness 4: UrlStateRaw No Padding With Undefined Bytes

```
Property: Same as Harness 3 but for UrlStateRaw (120 bytes).
Bound:    Same approach.
Rationale: Same rationale.
```

---

## 7. Mutation Testing Checkpoints

**Target: ≥90% mutation kill rate**

### Critical Mutations and Their Killers

| Mutation | Killed By | Assertion That Catches It |
|----------|-----------|---------------------------|
| Change `FILE_STATE_TABLE` name to `"file_statf"` | B10, B16, B24 | B10: exact name mismatch. B16: `name != "file_state"`. B24: table not found. |
| Change `FileStateRaw._reserved` size from 32 to 31 | B01, Kani 1 | B01: `size_of != 200`. |
| Change `UrlStateRaw._reserved` size from 46 to 45 | B02, Kani 2 | B02: `size_of != 120`. |
| Remove `#[repr(C)]` from FileStateRaw | B07, Proptest 3 | Round-trip fails or byte layout shifts. |
| Remove `bytemuck::Pod` derive from FileStateRaw | B03 | Compile failure — static check. |
| Swap two hash fields in FileStateRaw layout | Proptest 3 | `bytes_of(&s)[0..32] != s.content_hash` |
| Change `initialize_tables` to skip one table | B24 | Opening the skipped table fails. |
| Change `initialize_tables` to skip `commit()` call | B28 | Tables don't survive reopen. |
| Change size check from `== 200` to `== 199` | B36 | Valid 200-byte value triggers `PodSizeMismatch`. |
| Change size check from `== 120` to `== 119` | B37 | Valid 120-byte value triggers `PodSizeMismatch`. |
| Change hash key length check from `!= 32` to `!= 33` | B32 | 32-byte key incorrectly rejected, OR 33-byte key incorrectly accepted. |
| Remove leading `/` check in source path validation | B33 | `"/absolute/path.md"` accepted without error. |
| Remove `..` check in source path validation | B34 | `"foo/../bar.md"` accepted without error. |
| Remove scheme check in URL validation | B35 | `"example.com/page"` accepted without error. |
| Change `metadata` table name to `"meta"` | B10, B15, B23 | B10: name mismatch. B15: `"metadata" != "meta"`. B23: name mismatch. |
| Skip one table in name uniqueness set | B09 | Set size != 8. |
| Change `METADATA_TABLE` to `TableDefinition<&str, &[u8]>` | B13 | Compilation fails at integration test call site. |
| **Remove trailing-slash check in URL validation** | B40 | `"https://example.com/api/"` accepted without error. |
| **Remove trailing-whitespace check in source path** | B41 | `"test.md "` accepted without error. |
| **Remove null-byte check in source path** | B42 | `"test\x00.md"` accepted without error. |
| **`TableOpenFailed.table` returns wrong table name** | B26 | `table != "file_state"`. Concrete assertion catches it. |
| **`DeserializationFailed.type_name` returns wrong type** | DeserializationFailed test | `type_name != "Analysis"`. Concrete assertion catches it. |
| **`StorageError.operation` returns wrong operation** | StorageError test | `operation != "insert"`. Concrete assertion catches it. |
| **Return `"x"` for all error messages** | B26, B27, OpenFailed, ReadTxFailed, WriteTxFailed | Each asserts `message.contains(<keyword>)`. `"x"` fails all. |
| **Skip partial-init recovery** | B43 | Only 4 tables accessible after partial init + recovery. |
| **Reject valid URL with query params** | B45 | `Ok(())` not returned for valid URL. |
| **Reject valid URL with fragment** | B46 | `Ok(())` not returned for valid URL. |
| **Reject long but valid source path** | B47 | `Ok(())` not returned for 500-char path. |

**Kill rate: 27/27 = 100% of identified mutations.**

### Mutations That Must NOT Survive

Any mutation to:
- Table definition names → caught by B09/B10 + accessor tests B16–B23
- Pod struct field sizes/counts → caught by B01/B02 + Kani
- `initialize_tables` table list → caught by B24
- Size validation constants (200, 120, 32) → caught by B36/B37/B32
- Key validation logic (leading `/`, `..`, scheme, trailing slash, whitespace, null) → caught by B33/B34/B35/B40/B41/B42
- Error variant structural fields (table, type_name, operation, message content) → caught by concrete assertions in B26, DeserializationFailed, StorageError, and message substring checks
- `initialize_tables` resilience → caught by B43

---

## 8. Combinatorial Coverage Matrix

### Pod Struct Tests

| Scenario | Input Class | Expected Output | Layer |
|----------|-------------|-----------------|-------|
| FileStateRaw size | compile-time | 200 | static |
| UrlStateRaw size | compile-time | 120 | static |
| FileStateRaw zeroed round-trip | all-zero fields | each field == 0 / equal to original | unit |
| UrlStateRaw zeroed round-trip | all-zero fields | each field == 0 / equal to original | unit |
| FileStateRaw non-zero round-trip | arbitrary non-zero fields | equal to original (field-by-field) | unit |
| UrlStateRaw non-zero round-trip | arbitrary non-zero fields | equal to original (field-by-field) | unit |
| FileStateRaw random round-trip | proptest: any [u8; 200] | bytes == round-tripped bytes | proptest |
| UrlStateRaw random round-trip | proptest: any [u8; 120] | bytes == round-tripped bytes | proptest |
| FileStateRaw byte layout | proptest: any valid struct | field offsets match spec | proptest |
| UrlStateRaw byte layout | proptest: any valid struct | field offsets match spec | proptest |
| Pod cast on truncated buffer | 3 bytes | `Err(PodCastFailed { type_name: "FileStateRaw" })` | unit |
| Pod cast on oversized buffer | 201 bytes | `Err(PodSizeMismatch { table: "file_state", expected: 200, actual: 201 })` | integration |

### Table Definition Tests

| Scenario | Input Class | Expected Output | Layer |
|----------|-------------|-----------------|-------|
| Name uniqueness | 8 names | set size == 8 | unit |
| Name match spec | each name | exact string match | unit |
| Pod table types | file_state, url_state | `TableDefinition<&str, &[u8]>` | static |
| rkyv table types | 5 output tables | `TableDefinition<&[u8], &[u8]>` | static |
| Metadata table type | metadata | `TableDefinition<&str, &str>` | static |
| Legacy name disjoint | name sets | intersection == `{"metadata"}` | unit |
| Metadata identical to legacy | both definitions | same name + type | unit |

### Accessor Function Tests

| Scenario | Input Class | Expected Output | Layer |
|----------|-------------|-----------------|-------|
| file_state_table() | no input | name == `"file_state"` | unit |
| url_state_table() | no input | name == `"url_state"` | unit |
| analysis_outputs_table() | no input | name == `"analysis_outputs"` | unit |
| transform_outputs_table() | no input | name == `"transform_outputs"` | unit |
| chunk_outputs_table() | no input | name == `"chunk_outputs"` | unit |
| scrape_outputs_table() | no input | name == `"scrape_outputs"` | unit |
| snapshots_table() | no input | name == `"snapshots"` | unit |
| metadata_table() | no input | name == `"metadata"` | unit |

### Database Initialization Tests

| Scenario | Input Class | Expected Output | Layer |
|----------|-------------|-----------------|-------|
| Fresh db init | empty database | `Ok(())`, all 8 tables exist | integration |
| Idempotent init | already-initialized db | `Ok(())`, data preserved | integration |
| Type-conflict init | pre-existing table with wrong type | `Err(TableOpenFailed { table: "file_state" })` | integration |
| Commit failure | read-only directory | `Err(CommitFailed { message: contains "write" })` | integration |
| Reopen after init | closed+reopened db | all 8 tables accessible | integration |
| Data persistence | write+close+reopen | value == original | integration |
| 10-cycle stress | 10 inline sequential cycles | all 10 values readable | e2e |
| Coexistence with legacy | both DocCache and StateDb tables | both accessible | integration |
| Partial init recovery | 4 of 8 tables exist | `Ok(())`, all 8 accessible, original data preserved | integration |
| Shared metadata co-access | legacy write, new read and vice versa | both see each other's data | integration |

### Key Validation Tests

| Scenario | Input Class | Expected Output | Layer |
|----------|-------------|-----------------|-------|
| Valid 32-byte hash key | `[u8; 32]` | `Ok(())` | integration |
| Hash key 0 bytes | empty | `Err(InvalidHashKeyLength { actual: 0 })` | integration |
| Hash key 16 bytes | too short | `Err(InvalidHashKeyLength { actual: 16 })` | integration |
| Hash key 33 bytes | too long | `Err(InvalidHashKeyLength { actual: 33 })` | integration |
| Source path relative | `"concept/test.md"` | `Ok(())` | integration |
| Source path absolute | `"/abs/path.md"` | `Err(InvalidSourcePath { reason: contains "leading" })` | integration |
| Source path with .. | `"foo/../bar.md"` | `Err(InvalidSourcePath { reason: contains ".." })` | integration |
| Source path empty | `""` | `Err(InvalidSourcePath { reason: contains "empty" })` | integration |
| Source path trailing whitespace | `"test.md "` | `Err(InvalidSourcePath { reason: contains "whitespace" })` | integration |
| Source path null bytes | `"test\x00.md"` | `Err(InvalidSourcePath { reason: contains "null" })` | integration |
| Source path very long (500 chars) | 500-char valid path | `Ok(())`, readback matches | integration |
| URL with scheme | `"https://example.com"` | `Ok(())` | integration |
| URL without scheme | `"example.com/page"` | `Err(InvalidUrlKey { reason: contains "scheme" })` | integration |
| URL empty | `""` | `Err(InvalidUrlKey { reason: contains "empty" })` | integration |
| URL trailing slash | `"https://example.com/api/"` | `Err(InvalidUrlKey { reason: contains "trailing" })` | integration |
| URL with query params | `"https://example.com/api?q=1"` | `Ok(())`, readback matches | integration |
| URL with fragment | `"https://example.com/docs#s"` | `Ok(())`, readback matches | integration |

### Value Validation Tests

| Scenario | Input Class | Expected Output | Layer |
|----------|-------------|-----------------|-------|
| file_state valid 200 bytes | FileStateRaw bytes | `Ok(FileStateRaw)` equal to original | integration |
| file_state 199 bytes | truncated | `Err(PodSizeMismatch { table: "file_state", expected: 200, actual: 199 })` | integration |
| file_state 201 bytes | oversized | `Err(PodSizeMismatch { table: "file_state", expected: 200, actual: 201 })` | integration |
| url_state valid 120 bytes | UrlStateRaw bytes | `Ok(UrlStateRaw)` equal to original | integration |
| url_state 119 bytes | truncated | `Err(PodSizeMismatch { table: "url_state", expected: 120, actual: 119 })` | integration |
| url_state 121 bytes | oversized | `Err(PodSizeMismatch { table: "url_state", expected: 120, actual: 121 })` | integration |
| Valid rkyv archive | correct type bytes | `Ok(ArchivedType)` | integration |
| Invalid rkyv bytes | garbage `[0xDE,0xAD,0xBE,0xEF]` | `Err(InvalidArchive { type_name: "Analysis", message })` | integration |
| Missing key | nonexistent hash | `Err(KeyNotFound { table: "analysis_outputs" })` | integration |

### Error Variant Tests

| Scenario | Input Class | Expected Output | Layer |
|----------|-------------|-----------------|-------|
| OpenFailed | path is directory | `Err(OpenFailed { path: <exact dir>, source: contains "open" })` | integration |
| ReadTransactionFailed | concurrent write held | `Err(ReadTransactionFailed { message: contains "transaction" })` | integration |
| WriteTransactionFailed | concurrent write held | `Err(WriteTransactionFailed { message: contains "write" })` | integration |
| PodSizeMismatch | wrong size bytes | `Err(PodSizeMismatch { table: <exact name>, expected: <exact>, actual: <exact> })` | integration |
| PodCastFailed | truncated buffer | `Err(PodCastFailed { type_name: "FileStateRaw", message: contains "size" })` | unit |
| InvalidArchive | garbage bytes | `Err(InvalidArchive { type_name: "Analysis" })` | integration |
| DeserializationFailed | wrong type archive (deferred) | `Err(DeserializationFailed { type_name: "Analysis" })` | integration |
| SerializationFailed | construction-only | variant matches, display format verified | unit |
| TableOpenFailed | type-mismatch table | `Err(TableOpenFailed { table: "file_state", message: contains "type" })` | integration |
| KeyNotFound | nonexistent key | `Err(KeyNotFound { table: "analysis_outputs" })` | integration |
| StorageError | I/O failure (Linux) | `Err(StorageError { operation: "insert", message })` | integration |
| CommitFailed | read-only dir (Linux) | `Err(CommitFailed { message: contains "write" })` | integration |
| InvalidHashKeyLength | non-32-byte key | `Err(InvalidHashKeyLength { actual: <exact count> })` | integration |
| InvalidSourcePath | various invalid paths | `Err(InvalidSourcePath { reason: contains <keyword> })` | integration |
| InvalidUrlKey | various invalid URLs | `Err(InvalidUrlKey { reason: contains <keyword> })` | integration |

---

## Open Questions

1. **rkyv derive availability:** Fuzz Target 2 (rkyv archive validation) and the `DeserializationFailed` integration test require rkyv derives on domain types (`Analysis`, `Chunk`, `ScrapedPage`, `Snapshot`). These derives are explicitly out of scope for this bead (per contract Non-goal #1). Both are listed for planning; implementation is deferred to the rkyv-derive bead. For this bead, `DeserializationFailed` is verified via error construction only.

2. **SerializationFailed feasibility:** No deterministic natural trigger exists for `rkyv::to_bytes` failure with domain types. The error variant is verified via construction only (variant match + display format). When rkyv derives exist, the rkyv-derive bead should attempt an integration test if a failure path is discoverable.

3. **StorageError and CommitFailed platform scope:** Both `StorageError` (B-level scenario) and `CommitFailed` (B27) tests rely on Linux-specific filesystem permission semantics. They are gated with `#[cfg(target_os = "linux")]`. Non-Linux CI will skip these tests. This is an honest tradeoff: these error paths are rare, and cross-platform fault injection for redb is not feasible without mocking.

4. **ReadTransactionFailed / WriteTransactionFailed concurrency:** redb uses MVCC. Tests use deterministic single-process patterns: hold one write tx and attempt another on the same db handle. No timing dependency.

5. **Coexistence test (B31) and shared metadata (B44):** These tests require importing legacy `DocCache` table definitions from `crate::cache`. If the new state module is in a different crate or feature gate, these tests may need conditional compilation.

6. **Partial init simulation (B43):** Creating a "4 of 8 tables" state requires manually opening a subset of table definitions and committing, then calling `initialize_tables`. The test verifies that `initialize_tables` creates the missing tables without corrupting the existing ones.

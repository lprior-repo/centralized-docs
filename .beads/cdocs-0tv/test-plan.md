# Test Plan: cdocs-0tv — Snapshot APIs on `StateReadSession` and `StateDb`

## Summary

- **Behaviors identified**: 24
- **Trophy allocation**: 2 unit / 22 integration / 0 E2E / 0 static (covered by existing clippy/forbid(unsafe_code))
- **Proptest invariants**: 3
- **Fuzz targets**: 2
- **Kani harnesses**: 1
- **Mutation kill target**: ≥90%

---

## 1. Behavior Inventory

| # | Behavior |
|---|----------|
| B01 | `serialize_snapshot` returns valid rkyv bytes when given any well-formed `Snapshot` |
| B02 | `serialize_snapshot` returns `Err(SerializationFailed)` when rkyv serialization fails (in pathological edge case) |
| B03 | `load_snapshots` returns `HashMap` containing `OwnedArchive<Snapshot>` for every requested hash present in the `snapshots` table |
| B04 | `load_snapshots` omits missing hashes from the returned `HashMap` (no error for absent keys) |
| B05 | `load_snapshots` returns empty `HashMap` when called with empty `hashes` slice (no table access) |
| B06 | `load_snapshots` returns `OwnedArchive` values whose bytes are owned independently of the redb read transaction |
| B07 | `load_snapshots` returns `Err(TableOpenFailed { table: "snapshots" })` when the snapshots table cannot be opened for read |
| B08 | `load_snapshots` returns `Err(ArchiveValidationFailed)` when stored bytes fail rkyv bytecheck validation |
| B09 | `load_snapshots` returns `Err(DeserializationFailed)` when stored bytes fail rkyv deserialization |
| B10 | `load_snapshots` returns `Err(StorageError)` when a redb read operation fails |
| B11 | `commit_changes` writes all `new_snapshots` entries to the `snapshots` table in a single write transaction |
| B12 | `commit_changes` removes all `deleted_snapshots` entries from the `snapshots` table |
| B13 | `commit_changes` applies delete after insert — if a key appears in both `new_snapshots` and `deleted_snapshots`, the key is absent after commit (delete wins) |
| B14 | `commit_changes` applies last-entry-wins semantics for duplicate keys in `new_snapshots` |
| B15 | `commit_changes` rolls back all snapshot changes on failure — ACID atomicity |
| B16 | `commit_changes` returns `Err(WriteTransactionFailed)` when `begin_write` fails |
| B17 | `commit_changes` returns `Err(CommitFailed)` when `write_tx.commit()` fails |
| B18 | `load_snapshots` handles a mix of found and not-found hashes in a single call |
| B19 | `commit_changes` returns `Err(TableOpenFailed { table: "snapshots" })` when the snapshots table cannot be opened for write |
| B20 | `commit_changes` returns `Err(StorageError)` when a redb insert or delete fails during commit |
| B21 | `commit_changes` returns `Err(WriteTransactionFailed)` when `StateReadSession` is still active (one-read one-write invariant violation) |
| B22 | `commit_changes` returns `Ok(())` with no table mutations when `StateChanges` has empty `new_snapshots` and empty `deleted_snapshots` |
| B23 | `load_snapshots` handles 10,000+ hashes without error or OOM |
| B24 | `commit_changes` handles 10,000+ `new_snapshots` entries without error or OOM |

---

## 2. Trophy Allocation

| Behavior | Layer | Justification |
|----------|-------|---------------|
| B01 serialize_snapshot happy | Unit | Pure function, no I/O, deterministic |
| B02 serialize_snapshot error | Unit | Pure function, error path |
| B03 load_snapshots found | Integration | Real redb, real rkyv, real table I/O |
| B04 load_snapshots missing keys | Integration | Real redb table with no matching keys |
| B05 load_snapshots empty input | Integration | Must verify zero table access via real redb |
| B06 OwnedArchive byte independence | Integration | Must hold bytes after dropping redb txn |
| B07 TableOpenFailed (read path) | Integration | Corrupt table definition against real redb |
| B08 ArchiveValidationFailed | Integration | Write corrupt bytes, read back via real redb |
| B09 DeserializationFailed | Integration | Write valid-rkyv-but-wrong-type bytes, read back |
| B10 StorageError (read path) | Integration | Force redb error during read |
| B11 commit writes new_snapshots | Integration | Real redb write + verify persistence |
| B12 commit deletes | Integration | Real redb write + verify removal |
| B13 delete precedence over insert | Integration | Same key in both new and deleted |
| B14 last-entry wins on duplicate | Integration | Duplicate keys in new_snapshots |
| B15 ACID rollback on failure | Integration | Force mid-transaction failure, verify no partial writes |
| B16 WriteTransactionFailed | Integration | Force begin_write failure |
| B17 CommitFailed | Integration | Force commit failure |
| B18 mixed found/not-found | Integration | Real redb with partial data |
| B19 TableOpenFailed (write path) | Integration | Delete table, attempt commit, real redb |
| B20 StorageError (write path) | Integration | Force redb insert/delete failure during commit |
| B21 read session still active | Integration | Real redb MVCC contention, verify error + state |
| B22 empty changes | Integration | Real redb, verify no mutations |
| B23 load_snapshots 10K+ hashes | Integration | Real redb at scale boundary |
| B24 commit_changes 10K+ entries | Integration | Real redb at scale boundary |

**Ratios**: 2 unit (8%), 22 integration (92%), 0 static (0%), 0 E2E (0%).
**Justification for deviation**: This bead is a data layer — it sits between pure calc and I/O. The pure function (`serialize_snapshot`) is tested at unit level. All `load_snapshots` and `commit_changes` behaviors require a real redb database to validate, making integration tests the appropriate majority. No E2E because there is no CLI surface in this bead — the action layer migration (`cmd/watch.rs`) is a separate bead.

---

## 3. BDD Scenarios

### B01: `serialize_snapshot` returns valid rkyv bytes

```
fn serialize_snapshot_returns_valid_rkyv_bytes_when_given_snapshot()
```

**Given**: A `Snapshot` with `target_url = "https://example.com"`, `timestamp = Utc.ymd(2025, 1, 1).and_hms(0, 0, 0)`, and a non-empty `pages` map containing one `PageHash` entry.

**When**: `serialize_snapshot(&snapshot)` is called.

**Then**: The result is `Ok(bytes)` where `rkyv::access::<rkyv::ArchivedSnapshot>(&bytes)` succeeds without error, and the archived value's `target_url` field equals `"https://example.com"`, and `OwnedArchive::from_bytes(bytes).deserialize()` returns a `Snapshot` equal to the original input.

---

### B02: `serialize_snapshot` returns `Err(SerializationFailed)`

```
fn serialize_snapshot_returns_serialization_failed_when_rkyv_fails()
```

**Given**: Conditions that cause `rkyv::to_bytes` to fail. In practice, this requires a `Snapshot` whose serialized form exceeds `isize::MAX` bytes or triggers an internal rkyv sanity check. Test by mocking / constructing a pathologically deep recursive structure or by testing with a custom wrapper that injects failure.

**When**: `serialize_snapshot(&snapshot)` is called.

**Then**: The result is `Err(StateError::SerializationFailed { message })` where `message` is a non-empty string.

**Note**: If rkyv `to_bytes` is infallible for all valid `Snapshot` inputs (which it likely is), this test should verify that the error variant is mapped correctly by constructing a scenario where the rkyv serializer reports an error. If rkyv truly cannot fail on `Snapshot`, document this and mark as "not practically reachable — covered by type system."

---

### B03: `load_snapshots` returns entries for found hashes

```
fn load_snapshots_returns_owned_archives_when_hashes_exist_in_table()
```

**Given**: A `StateDb` with the `snapshots` table populated: serialize a `Snapshot` via `serialize_snapshot`, write it to the `snapshots` table under key `K1` using a direct redb write transaction. Then open a `StateReadSession`.

**When**: `session.load_snapshots(&[K1])` is called.

**Then**: The result is `Ok(map)` where `map.len() == 1`, `map.contains_key(&K1)` is `true`, and `map[&K1].deserialize()` returns a `Snapshot` equal to the original.

---

### B04: `load_snapshots` omits missing hashes

```
fn load_snapshots_omits_missing_hashes_when_not_in_table()
```

**Given**: A `StateDb` with an empty `snapshots` table. Open a `StateReadSession`.

**When**: `session.load_snapshots(&[K_MISSING])` is called where `K_MISSING` is any `[u8; 32]` not present in the table.

**Then**: The result is `Ok(map)` where `map.is_empty()` is `true`. No error is returned.

---

### B05: `load_snapshots` returns empty map for empty input

```
fn load_snapshots_returns_empty_hashmap_when_hashes_slice_is_empty()
```

**Given**: A `StateDb` with a populated `snapshots` table. Open a `StateReadSession`.

**When**: `session.load_snapshots(&[])` is called.

**Then**: The result is `Ok(map)` where `map.is_empty()` is `true`. The `snapshots` table is not accessed (verifiable by checking that redb read transaction stats show zero table opens, or by testing that a corrupted table does not cause an error).

---

### B06: `OwnedArchive` bytes are independent of redb transaction

```
fn load_snapshots_returns_bytes_independent_of_redb_transaction_lifetime()
```

**Given**: A `StateDb` with one snapshot `K1` in the `snapshots` table.

**When**: `session.load_snapshots(&[K1])` is called, the result is stored in `map`, and then the `StateReadSession` is dropped (ending the redb read transaction).

**Then**: `map[&K1].archived()` still returns a valid `&ArchivedSnapshot` with correct field values. `map[&K1].deserialize()` returns `Ok(Snapshot)` equal to the original.

---

### B07: `load_snapshots` returns `TableOpenFailed` (read path)

```
fn load_snapshots_returns_table_open_failed_when_snapshots_table_missing()
```

**Given**: A `StateDb` opened from a path where the `snapshots` table has been explicitly deleted (via `write_tx.delete_table(SNAPSHOTS_TABLE)` after `StateDb::open`). Open a `StateReadSession`.

**When**: `session.load_snapshots(&[K1])` is called.

**Then**: The result is `Err(StateError::TableOpenFailed { table: "snapshots", message })` where `table` equals `"snapshots"` and `message` is a non-empty string describing the failure.

---

### B08: `load_snapshots` returns `ArchiveValidationFailed`

```
fn load_snapshots_returns_archive_validation_failed_when_bytes_corrupt()
```

**Given**: A `StateDb` where the `snapshots` table contains entry `K1` with value `b"DEADBEEF_CORRUPT_BYTES"` (not a valid rkyv archive of `Snapshot`). Open a `StateReadSession`.

**When**: `session.load_snapshots(&[K1])` is called.

**Then**: The result is `Err(StateError::ArchiveValidationFailed { key_hex, message })` where `key_hex` equals the hex-encoded `K1` and `message` is a non-empty string describing the validation failure.

---

### B09: `load_snapshots` returns `DeserializationFailed`

```
fn load_snapshots_returns_deserialization_failed_when_bytes_wrong_type()
```

**Given**: A `StateDb` where the `snapshots` table contains entry `K1` with bytes that are a valid rkyv archive but of a different type (e.g., rkyv-serialized `String` `"hello"` instead of `Snapshot`). Open a `StateReadSession`.

**When**: `session.load_snapshots(&[K1])` is called.

**Then**: The result is `Err(StateError::DeserializationFailed { key_hex, message })` where `key_hex` matches `K1` hex-encoded and `message` is a non-empty string.

---

### B10: `load_snapshots` returns `StorageError` (read path)

```
fn load_snapshots_returns_storage_error_when_redb_read_fails()
```

**Given**: A `StateDb` that is corrupted or locked in a way that causes redb read operations to fail. This can be simulated by closing the database handle while the read session still holds a reference, or by using a corrupted redb file.

**When**: `session.load_snapshots(&[K1])` is called.

**Then**: The result is `Err(StateError::StorageError { operation: "load_snapshots", message })` where `operation` equals `"load_snapshots"` and `message` is a non-empty string.

**Note**: This scenario may be difficult to trigger deterministically. If redb guarantees read success for open databases, document that this path is unreachable in practice but covered by the error mapping code.

---

### B11: `commit_changes` writes new snapshots

```
fn commit_changes_writes_new_snapshots_to_table_when_changes_committed()
```

**Given**: A `StateDb` with an empty `snapshots` table. A `StateChanges` with `new_snapshots = vec![(K1, rkyv_bytes)]` where `rkyv_bytes` is the output of `serialize_snapshot(&snapshot)`. The `StateReadSession` has been dropped.

**When**: `state_db.commit_changes(&changes)` is called.

**Then**: The result is `Ok(())`. Opening a new `StateReadSession` and calling `load_snapshots(&[K1])` returns `Ok(map)` where `map[&K1].deserialize()` equals the original `snapshot`.

---

### B12: `commit_changes` deletes snapshots

```
fn commit_changes_removes_deleted_snapshots_from_table()
```

**Given**: A `StateDb` with snapshot `K1` in the `snapshots` table. A `StateChanges` with `new_snapshots = vec![]` and `deleted_snapshots = vec![K1]`. The `StateReadSession` has been dropped.

**When**: `state_db.commit_changes(&changes)` is called.

**Then**: The result is `Ok(())`. Opening a new `StateReadSession` and calling `load_snapshots(&[K1])` returns `Ok(map)` where `map.is_empty()` is `true` (key was deleted).

---

### B13: Delete takes precedence over insert for same key

```
fn commit_changes_delete_wins_when_same_key_in_new_and_deleted()
```

**Given**: A `StateDb` with no prior snapshot for `K1`. A `StateChanges` with `new_snapshots = vec![(K1, rkyv_bytes)]` AND `deleted_snapshots = vec![K1]`. The `StateReadSession` has been dropped.

**When**: `state_db.commit_changes(&changes)` is called.

**Then**: The result is `Ok(())`. Opening a new `StateReadSession` and calling `load_snapshots(&[K1])` returns `Ok(map)` where `map.is_empty()` is `true` — the delete was applied after the insert, leaving no entry.

---

### B14: Last entry wins for duplicate keys in `new_snapshots`

```
fn commit_changes_last_entry_wins_when_duplicate_keys_in_new_snapshots()
```

**Given**: A `StateChanges` with `new_snapshots = vec![(K1, bytes_v1), (K1, bytes_v2)]` where `bytes_v1` and `bytes_v2` serialize different `Snapshot` values. The `StateReadSession` has been dropped.

**When**: `state_db.commit_changes(&changes)` is called.

**Then**: The result is `Ok(())`. Opening a new `StateReadSession` and calling `load_snapshots(&[K1])` returns `Ok(map)` where `map[&K1].deserialize()` equals the `Snapshot` serialized as `bytes_v2` (the last entry wins).

---

### B15: ACID rollback on failure

```
fn commit_changes_rolls_back_all_snapshot_changes_when_commit_fails()
```

**Given**: A `StateDb` with snapshot `K_EXISTING` already in the `snapshots` table. A `StateChanges` with `new_snapshots = vec![(K_NEW, bytes)]` and `deleted_snapshots = vec![K_EXISTING]`. The commit is forced to fail by corrupting the database handle or using a mechanism that deterministically triggers a commit-phase failure (e.g., closing the file handle before commit, or using a read-only filesystem wrapper).

**When**: `state_db.commit_changes(&changes)` is called.

**Then**: The result is `Err(StateError::CommitFailed { message })` where `message` is a non-empty string. Opening a new `StateDb` on the same path and loading `K_EXISTING` returns the original snapshot (unchanged). `K_NEW` is not present in the table. No partial state was written.

**Note**: This test verifies redb's ACID guarantee. If redb guarantees atomicity at the storage level, the test confirms the error path correctly propagates `CommitFailed` without partial writes. The trigger mechanism must deterministically produce `CommitFailed` — not a generic "error".

---

### B16: `commit_changes` returns `WriteTransactionFailed`

```
fn commit_changes_returns_write_transaction_failed_when_begin_write_fails()
```

**Given**: A `StateDb` that cannot start a write transaction (e.g., the underlying file is on a read-only filesystem, or the database handle is in a state where writes are blocked).

**When**: `state_db.commit_changes(&changes)` is called with any `StateChanges` containing `new_snapshots` or `deleted_snapshots`.

**Then**: The result is `Err(StateError::WriteTransactionFailed { message })` where `message` is a non-empty string.

---

### B17: `commit_changes` returns `CommitFailed`

```
fn commit_changes_returns_commit_failed_when_redb_commit_fails()
```

**Given**: A scenario where `write_tx.commit()` fails (e.g., disk full, I/O error during flush). This is difficult to simulate with redb directly — may require a custom test fixture that limits disk space or uses a FUSE filesystem.

**When**: `state_db.commit_changes(&changes)` is called and the commit phase fails.

**Then**: The result is `Err(StateError::CommitFailed { message })` where `message` is a non-empty string.

---

### B18: Mixed found and not-found hashes

```
fn load_snapshots_returns_partial_map_when_some_hashes_found_and_some_missing()
```

**Given**: A `StateDb` with snapshots for `K1` and `K2` in the table, but NOT for `K3`.

**When**: `session.load_snapshots(&[K1, K2, K3])` is called.

**Then**: The result is `Ok(map)` where `map.len() == 2`, `map.contains_key(&K1)` is `true`, `map.contains_key(&K2)` is `true`, `map.contains_key(&K3)` is `false`.

---

### B19: `commit_changes` returns `TableOpenFailed` (write path)

```
fn commit_changes_returns_table_open_failed_when_snapshots_table_missing_for_write()
```

**Given**: A `StateDb` opened normally, then the `snapshots` table is explicitly deleted via `write_tx.delete_table(SNAPSHOTS_TABLE)` after `StateDb::open`. The `StateReadSession` has been dropped. A `StateChanges` with `new_snapshots = vec![(K1, rkyv_bytes)]`.

**When**: `state_db.commit_changes(&changes)` is called.

**Then**: The result is `Err(StateError::TableOpenFailed { table: "snapshots", message })` where `table` equals `"snapshots"` and `message` is a non-empty string. The write transaction is not started or is cleanly aborted — no partial writes occur.

---

### B20: `commit_changes` returns `StorageError` (write path)

```
fn commit_changes_returns_storage_error_when_redb_insert_fails_during_commit()
```

**Given**: A `StateDb` in a state that causes redb insert or delete to fail during the commit write transaction. This can be triggered by corrupting the database file after opening the write transaction has begun, or by using a redb wrapper that injects a storage error on the next table write operation. The `StateReadSession` has been dropped. A `StateChanges` with `new_snapshots = vec![(K1, rkyv_bytes)]`.

**When**: `state_db.commit_changes(&changes)` is called.

**Then**: The result is `Err(StateError::StorageError { operation, message })` where `operation` matches the commit-path operation string (e.g., `"commit_snapshot_insert"` or the operation string used in the implementation) and `message` is a non-empty string.

**Note**: If redb's insert/delete operations are infallible for an open write transaction on a healthy database, this test should document that the error mapping code is correct but the path is unreachable in practice. The test-writer must verify whether a deterministic trigger exists.

---

### B21: `commit_changes` fails when `StateReadSession` is still active

```
fn commit_changes_returns_write_transaction_failed_when_read_session_still_active()
```

**Given**: A `StateDb` with a `StateReadSession` that is still in scope (NOT dropped). This violates the one-read, one-write invariant. A `StateChanges` with `new_snapshots = vec![(K1, rkyv_bytes)]`.

**When**: `state_db.commit_changes(&changes)` is called while the `StateReadSession` is still alive.

**Then**: The result is `Err(StateError::WriteTransactionFailed { message })` where `message` is a non-empty string describing the transaction contention. Opening a new `StateDb` on the same path and loading `K1` confirms it is absent — no partial writes occurred. After dropping the `StateReadSession`, calling `commit_changes` with the same changes succeeds (`Ok(())`), proving the error is caused specifically by the active read session.

**Rationale**: Contract Invariant 2 (contract.md line 51) states: "Snapshot persistence obeys the one-read, one-write invariant per run. The StateReadSession must be dropped before commit_changes can succeed." redb enforces this at runtime via MVCC. This negative test proves the enforcement works and the correct error variant is returned.

---

### B22: `commit_changes` succeeds with no mutations when changes are empty

```
fn commit_changes_succeeds_with_no_mutations_when_new_and_deleted_snapshots_empty()
```

**Given**: A `StateDb` with snapshot `K_EXISTING` already in the `snapshots` table. A `StateChanges` with `new_snapshots = vec![]` and `deleted_snapshots = vec![]`. The `StateReadSession` has been dropped.

**When**: `state_db.commit_changes(&changes)` is called.

**Then**: The result is `Ok(())`. Opening a new `StateReadSession` and calling `load_snapshots(&[K_EXISTING])` returns `Ok(map)` where `map.len() == 1` and `map[&K_EXISTING].deserialize()` equals the original snapshot — no mutations occurred.

---

### B23: `load_snapshots` handles 10,000+ hashes without error

```
fn load_snapshots_returns_all_entries_when_given_10000_hashes()
```

**Given**: A `StateDb` with 10,000 snapshots in the `snapshots` table, each with a unique key `K_i` for `i` in `0..10000`. Open a `StateReadSession`.

**When**: `session.load_snapshots(&all_10000_keys)` is called.

**Then**: The result is `Ok(map)` where `map.len() == 10000`. No OOM, no timeout, no panic. A random sample of 10 entries from `map` deserialize to their original `Snapshot` values.

**Rationale**: Boundary test for scale. Ensures no batching issues, excessive memory allocation, or transaction duration limits are hit at realistic scale.

---

### B24: `commit_changes` handles 10,000+ entries without error

```
fn commit_changes_writes_10000_snapshots_when_given_10000_new_entries()
```

**Given**: A `StateDb` with an empty `snapshots` table. A `StateChanges` with `new_snapshots` containing 10,000 entries `(K_i, rkyv_bytes_i)` for unique keys. The `StateReadSession` has been dropped.

**When**: `state_db.commit_changes(&changes)` is called.

**Then**: The result is `Ok(())`. Opening a new `StateReadSession` and calling `load_snapshots(&all_10000_keys)` returns `Ok(map)` where `map.len() == 10000`. No OOM, no timeout, no panic. A random sample of 10 entries deserialize to their original `Snapshot` values.

**Rationale**: Boundary test for scale. Ensures no write transaction timeout, no excessive memory allocation, no batch-size limits.

---

## 4. Proptest Invariants

### Proptest 1: `serialize_snapshot` round-trip

**Invariant**: For any valid `Snapshot` value, `serialize_snapshot(&s)` produces bytes that, when accessed via `rkyv::access`, yield an `ArchivedSnapshot` whose fields match the original. Further, `OwnedArchive::from_bytes(bytes).deserialize()` produces a `Snapshot` equal to the original.

**Strategy**: Generate arbitrary `Snapshot` values with:
- `target_url`: any non-empty UTF-8 string up to 2048 chars (`"[a-zA-Z0-9:/._-]{1,2048}"`)
- `timestamp`: any `DateTime<Utc>` within year 2000–2100
- `pages`: `BTreeMap<String, PageHash>` with 0–50 entries, where each `PageHash` has arbitrary `url`, `content_hash: [u8; 32]` (any 32 bytes), and `title` (non-empty string)

**Anti-invariant**: Empty `target_url` should still succeed (no constraint on non-empty in the type).

```
fn proptest_serialize_snapshot_roundtrip(snapshot in snapshot_strategy())
```

---

### Proptest 2: `serialize_snapshot` output is deterministic

**Invariant**: Calling `serialize_snapshot(&s)` twice on the same `Snapshot` produces identical byte vectors.

**Strategy**: Same `Snapshot` strategy as Proptest 1.

```
fn proptest_serialize_snapshot_deterministic(snapshot in snapshot_strategy())
```

---

### Proptest 3: `load_snapshots` — round-trip through commit + load

**Invariant**: For any set of `(key, Snapshot)` pairs, serializing each via `serialize_snapshot`, committing via `commit_changes`, then loading via `load_snapshots` produces a `HashMap` containing exactly the committed keys with deserializable values equal to the originals.

**Strategy**: Generate `Vec<([u8; 32], Snapshot)>` with 0–20 entries, no duplicate keys.

**Anti-invariant**: Empty vec → empty result. Duplicate keys → last-wins semantics.

```
fn proptest_load_snapshots_roundtrip(entries in snapshot_entries_strategy())
```

---

## 5. Fuzz Targets

### Fuzz Target 1: `load_snapshots` — corrupt bytes in redb

**Input type**: `Vec<u8>` — arbitrary bytes stored directly in the `snapshots` table under a fixed key, then loaded via `load_snapshots`.

**Risk**: Panic in rkyv `access` or `bytecheck` when validating corrupt bytes. Must never panic — must always return `Err(ArchiveValidationFailed)` or `Err(DeserializationFailed)`.

**Corpus seeds**:
- Empty byte vector (`[]`)
- Single null byte (`[0x00]`)
- Valid rkyv archive of `Snapshot` (happy path baseline)
- Valid rkyv archive of `String` (wrong type)
- Random 32 bytes
- Valid rkyv archive with first 4 bytes zeroed (corrupt header)
- Very large byte vector (1 MB of random data)

```
fn fuzz_load_snapshots_corrupt_bytes(data: &[u8])
```

---

### Fuzz Target 2: `serialize_snapshot` — arbitrary Snapshot-like input

**Input type**: Raw bytes interpreted as rkyv-serialized `Snapshot` or arbitrary struct fields.

**Risk**: If `serialize_snapshot` ever accepts untrusted input (e.g., bytes claimed to be a Snapshot), fuzz to ensure no panic on pathological inputs. Since `serialize_snapshot` takes a `&Snapshot` (Rust type), the primary risk is in the rkyv serializer itself, which should be infallible for valid types. This fuzz target validates that assumption.

**Corpus seeds**:
- Minimal `Snapshot` (empty pages)
- Maximal `Snapshot` (1000 pages, long URLs, long titles)
- Snapshot with non-ASCII `target_url`
- Snapshot with zero timestamp

```
fn fuzz_serialize_snapshot_arbitrary_snapshot(data: &[u8])
// Strategy: interpret data as field values for Snapshot, construct Snapshot, call serialize_snapshot
```

---

## 6. Kani Harnesses

### Kani Harness 1: `serialize_snapshot` — no panic, no overflow

**Property**: `serialize_snapshot` never panics and never causes undefined behavior for any valid `Snapshot` input. Specifically:
- No arithmetic overflow in length calculations
- No out-of-bounds access in byte construction
- Result bytes are always a valid rkyv archive of `Snapshot`

**Bound**: `pages.len() <= 10`, `target_url.len() <= 256`, each page URL/title `<= 256` chars.

**Rationale**: rkyv serialization involves pointer arithmetic and length calculations. While rkyv is well-tested, a Kani proof provides formal assurance that our specific `Snapshot` type never triggers edge cases in the serializer.

```
#[kani::proof]
fn kani_serialize_snapshot_no_panic()
```

---

## 7. Mutation Testing Checkpoints

**Threshold**: ≥90% mutation kill rate.

| Mutation Target | Killed By |
|-----------------|-----------|
| `serialize_snapshot`: remove `rkyv::to_bytes` call, return empty vec | B01 — checks round-trip via `rkyv::access` + field equality |
| `serialize_snapshot`: swap error variant from `SerializationFailed` to `StorageError` | B02 — checks exact error variant `SerializationFailed` |
| `load_snapshots`: remove early-return for empty `hashes` | B05 — empty input should not touch table |
| `load_snapshots`: return error for missing key instead of omitting | B04 — missing keys must not error |
| `load_snapshots`: change `bytes.to_vec()` to `bytes` (keep reference) | B06 — must own bytes after txn dropped |
| `load_snapshots`: swap `ArchiveValidationFailed` → `DeserializationFailed` | B08 — checks exact error variant |
| `load_snapshots`: swap `DeserializationFailed` → `ArchiveValidationFailed` | B09 — checks exact error variant |
| `load_snapshots`: skip one hash in iteration | B03 — must return all found entries |
| `load_snapshots`: skip `key_hex` formatting in error | B08, B09 — checks `key_hex` matches |
| `commit_changes`: swap insert/delete order (insert after delete) | B13 — delete must win on collision |
| `commit_changes`: skip deduplication (first-wins instead of last-wins) | B14 — last entry must win |
| `commit_changes`: skip delete entirely | B12 — deleted keys must be absent |
| `commit_changes`: skip insert entirely | B11 — new keys must be present |
| `commit_changes`: return `Ok(())` early before commit | B15 — verifies state unchanged + B11 verifies actual persistence |
| `commit_changes`: swap `CommitFailed` → `WriteTransactionFailed` | B17 — checks exact error variant |
| `commit_changes`: remove `TableOpenFailed` mapping for write path (fall through to `StorageError`) | B19 — checks exact error variant `TableOpenFailed` from write path |
| `commit_changes`: remove `StorageError` mapping for write path (fall through to `CommitFailed`) | B20 — checks exact error variant `StorageError` from write path |
| `commit_changes`: skip read-session-active check (allow concurrent write) | B21 — must fail when read session active |
| `commit_changes`: mutate table when changes are empty | B22 — empty changes must not mutate state |

---

## 8. Combinatorial Coverage Matrix

### `serialize_snapshot` (unit)

| Scenario | Input Class | Expected Output | Layer |
|----------|-------------|-----------------|-------|
| Happy path: typical snapshot | Valid Snapshot with 1–10 pages | `Ok(Vec<u8>)` where `rkyv::access` succeeds and archived `target_url` matches input | unit |
| Happy path: empty pages map | Valid Snapshot with `pages = BTreeMap::new()` | `Ok(Vec<u8>)` where `rkyv::access` succeeds and `deserialize()` equals original | unit |
| Happy path: large snapshot | Valid Snapshot with 100+ pages | `Ok(Vec<u8>)` where `rkyv::access` succeeds and `deserialize()` equals original | unit |
| Round-trip | Any valid Snapshot | `deserialize(serialize(s)) == Ok(s)` | unit + proptest |
| Determinism | Same Snapshot called twice | Identical byte vectors | proptest |
| Serialization failure | Pathological input (if reachable) | `Err(StateError::SerializationFailed { message })` where `message` is non-empty | unit |

### `load_snapshots` (integration)

| Scenario | Input Class | Expected Output | Layer |
|----------|-------------|-----------------|-------|
| Found: single hash | `[K1]` where K1 exists | `Ok(HashMap { K1 → OwnedArchive })` with correct deserialized value | integration |
| Found: multiple hashes | `[K1, K2, K3]` all exist | `Ok(HashMap { K1 → _, K2 → _, K3 → _ })` len == 3 | integration |
| Not found: single hash | `[K_MISSING]` | `Ok(HashMap::new())` | integration |
| Not found: multiple hashes | `[K1, K_MISSING]` | `Ok(HashMap { K1 → _ })` len == 1 | integration |
| Empty input | `&[]` | `Ok(HashMap::new())` | integration |
| Boundary: all-zeros key | `[ [0u8; 32] ]` stored | `Ok(HashMap { [0u8; 32] → OwnedArchive })` | integration |
| Boundary: all-0xFF key | `[ [0xFF; 32] ]` stored | `Ok(HashMap { [0xFF; 32] → OwnedArchive })` | integration |
| Boundary: 10,000+ hashes | 10,000 unique keys all stored | `Ok(HashMap)` len == 10000 | integration |
| Corrupt bytes in table | `[K1]` with corrupt value | `Err(StateError::ArchiveValidationFailed { key_hex, message })` | integration |
| Wrong-type bytes in table | `[K1]` with wrong type | `Err(StateError::DeserializationFailed { key_hex, message })` | integration |
| Table missing (read path) | `[K1]` with no snapshots table | `Err(StateError::TableOpenFailed { table: "snapshots", message })` | integration |
| Storage error (read path) | `[K1]` with corrupted redb | `Err(StateError::StorageError { operation: "load_snapshots", message })` | integration |
| Round-trip: commit then load | Any set of (key, snapshot) pairs | All pairs found with correct values | proptest |

### `commit_changes` (snapshot paths) (integration)

| Scenario | Input Class | Expected Output | Layer |
|----------|-------------|-----------------|-------|
| Write single new snapshot | `new_snapshots = [(K1, bytes)]` | `Ok(())`, K1 persists and deserializes correctly | integration |
| Write multiple new snapshots | `new_snapshots = [(K1, b1), (K2, b2)]` | `Ok(())`, K1 and K2 persist | integration |
| Delete existing snapshot | `deleted_snapshots = [K1]` | `Ok(())`, K1 absent | integration |
| Delete + insert same key | Both `new_snapshots = [(K1, b)]` and `deleted_snapshots = [K1]` | `Ok(())`, K1 absent (delete wins) | integration |
| Duplicate keys in new | `new_snapshots = [(K1, b1), (K1, b2)]` | `Ok(())`, K1 has b2 value (last wins) | integration |
| Empty changes | `new_snapshots = []`, `deleted_snapshots = []` | `Ok(())`, existing state unchanged | integration |
| ACID: rollback on failure | Changes + forced commit failure | `Err(StateError::CommitFailed { message })`, no partial writes | integration |
| WriteTransactionFailed | DB in read-only state | `Err(StateError::WriteTransactionFailed { message })` | integration |
| CommitFailed | Disk full during commit | `Err(StateError::CommitFailed { message })` | integration |
| TableOpenFailed (write path) | Snapshots table deleted before commit | `Err(StateError::TableOpenFailed { table: "snapshots", message })` | integration |
| StorageError (write path) | redb insert/delete fails during commit | `Err(StateError::StorageError { operation, message })` | integration |
| Read session still active | Active StateReadSession not dropped | `Err(StateError::WriteTransactionFailed { message })`, state unchanged | integration |
| Boundary: 10,000+ new entries | 10,000 unique (K_i, bytes_i) pairs | `Ok(())`, all 10,000 persist | integration |

---

## Open Questions

1. **B02 (SerializationFailed reachability)**: `rkyv::to_bytes::<rkyv::rancor::Error>(&value)` may be infallible for all valid `Snapshot` values. If so, `SerializationFailed` is unreachable through normal use but remains in the error enum for forward compatibility. The test-writer should verify this and document accordingly rather than fabricating an impossible scenario.

2. **B10 (StorageError trigger — read path)**: redb provides strong guarantees for read operations on an open database. If there is no deterministic way to trigger a redb `StorageError` during a read on a healthy database, this test should document that the error mapping is correct but the path is unreachable in practice.

3. **B17 (CommitFailed trigger)**: Similar to B10, forcing a redb commit failure deterministically may require external infrastructure (FUSE filesystem, disk quota). The test-writer should evaluate feasibility and potentially use a wrapper/fake for the redb commit call if direct forcing is impractical.

4. **B20 (StorageError trigger — write path)**: redb's insert/delete operations within an open write transaction may be infallible on a healthy database. The test-writer must investigate whether a deterministic trigger exists (e.g., corruption during write, out-of-space). If no trigger exists, document that the error mapping code is correct but the path is unreachable.

5. **B21 (exact error variant)**: When a `StateReadSession` is active, redb's `begin_write()` may return a `WriteTransactionFailed` error or a different contention error depending on redb's MVCC implementation. The test-writer must confirm the exact variant returned and update the Then clause accordingly. The plan assumes `WriteTransactionFailed` based on redb's single-writer constraint.

6. **rkyv version alignment**: The architecture spec references `rkyv = "0.8"`. The actual `Cargo.toml` does not yet include `rkyv`. The test-writer must confirm rkyv is added before writing tests. All `rkyv::access`, `rkyv::to_bytes`, and `rkyv::Deserialize` API calls are version-sensitive.

7. **OwnedArchive implementation**: The contract assumes `OwnedArchive<T>` already exists. The test-writer must verify its availability and API (`archived()`, `deserialize()`, `from_bytes()` or equivalent constructor) before writing tests.

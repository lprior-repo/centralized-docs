# Test Plan: `cdocs-b3v` — Raw State Bulk Loaders on `StateReadSession`

## Summary

| Metric                     | Count |
|----------------------------|-------|
| Behaviors identified       | 30    |
| Trophy allocation          | 2 static / 2 unit / 26 integration |
| Proptest invariants        | 4     |
| Fuzz targets               | 2     |
| Kani harnesses             | 2     |
| Error variants covered     | 3/3 for both `load_file_states` AND `load_url_states` |
| Mutation kill target       | ≥90%  |

**Rationale for trophy skew:** This bead is almost entirely integration-level. The two public methods (`load_file_states`, `load_url_states`) operate on a real `redb::ReadTransaction` scanning real tables. The only pure (Calc-layer) logic is the `bytemuck::Pod` zero-copy cast itself and the struct size invariants — those are unit tests. Everything else requires a real redb database, which makes it integration by definition.

**Resource cleanup:** ALL integration tests use `redb::Builder::new()` in-memory mode. No tempfile-based databases. No cleanup required. If a future test adds tempfile usage, the cleanup strategy must be explicitly documented here.

---

## 1. Behavior Inventory

### Construction

| #  | Behavior                                                                                           |
|----|----------------------------------------------------------------------------------------------------|
| B1 | `StateReadSession::new` borrows a `ReadTransaction` and returns a session that holds the `&'tx` lifetime |
| B2 | `StateReadSession` is not `Clone` (compile-time enforced)                                         |
| B3 | `StateReadSession` is not `Send` (compile-time enforced, transaction-bound)                       |

### `load_file_states`

| #  | Behavior                                                                                           |
|----|----------------------------------------------------------------------------------------------------|
| B4 | `load_file_states` returns `Ok(HashMap)` with all rows when every row value is exactly 40 bytes    |
| B5 | `load_file_states` returns `Ok(HashMap::new())` when `SNAPSHOTS_TABLE` is empty                    |
| B6 | `load_file_states` returns `Err(MalformedRow)` with exact fields when value is 39 bytes            |
| B7 | `load_file_states` returns `Err(MalformedRow)` with exact fields when value is 41 bytes            |
| B8 | `load_file_states` returns `Err(MalformedRow)` with exact fields when value is 0 bytes             |
| B9 | `load_file_states` returns `Err(MalformedRow)` on the **first** corrupt row and aborts — no partial map |
| B10| `load_file_states` returns `Err(Utf8KeyError)` with exact `bytes_lossy` when key has non-UTF-8 bytes |
| B11| `load_file_states` returns `Err(BackendError)` with exact fields when table cannot be opened       |
| B12| `load_file_states` decoded `FileStateRaw` values are bitwise-identical to the bytes written        |
| B13| `load_file_states` uses the borrowed transaction — snapshot isolation verified                      |
| B14| `load_file_states` does not read rows from `SCRAPE_TABLE` (cross-table isolation)                  |
| B15| `load_file_states` HashMap keys are exact UTF-8 round-trips including multi-byte characters        |

### `load_url_states`

| #  | Behavior                                                                                           |
|----|----------------------------------------------------------------------------------------------------|
| B16| `load_url_states` returns `Ok(HashMap)` with all rows when every row value is exactly 40 bytes     |
| B17| `load_url_states` returns `Ok(HashMap::new())` when `SCRAPE_TABLE` is empty                        |
| B18| `load_url_states` returns `Err(MalformedRow)` with exact fields when value is 39 bytes             |
| B19| `load_url_states` returns `Err(MalformedRow)` with exact fields when value is 41 bytes             |
| B20| `load_url_states` returns `Err(MalformedRow)` with exact fields when value is 0 bytes              |
| B21| `load_url_states` returns `Err(MalformedRow)` with exact fields when value is 80 bytes             |
| B22| `load_url_states` returns `Err(MalformedRow)` on first corrupt row — aborts with exact fields, no partial map |
| B23| `load_url_states` returns `Err(Utf8KeyError)` with exact `bytes_lossy` when key has non-UTF-8 bytes |
| B24| `load_url_states` returns `Err(BackendError)` with exact fields when table cannot be opened        |
| B25| `load_url_states` decoded `UrlStateRaw` values are bitwise-identical to the bytes written          |
| B26| `load_url_states` uses the borrowed transaction — snapshot isolation verified                       |
| B27| `load_url_states` does not read rows from `SNAPSHOTS_TABLE` (cross-table isolation)                |
| B28| `load_url_states` HashMap keys are exact UTF-8 round-trips including multi-byte characters         |

### Struct invariants

| #  | Behavior                                                                                           |
|----|----------------------------------------------------------------------------------------------------|
| B29| `size_of::<FileStateRaw>()` == 40 bytes (compile-time const assert)                                |
| B30| `size_of::<UrlStateRaw>()` == 40 bytes (compile-time const assert)                                 |

---

## 2. Trophy Allocation

| Behavior | Layer       | Justification                                                                                    |
|----------|-------------|--------------------------------------------------------------------------------------------------|
| B1       | integration | Requires a real `redb::ReadTransaction` to construct the session                                 |
| B2       | static      | Compile-time `Clone` negative — `static_assertions::assert_not_impl_any!` or manual trait check  |
| B3       | static      | Compile-time `Send` negative — same technique                                                    |
| B4       | integration | Full redb round-trip: write rows via write tx, read via `StateReadSession`                       |
| B5       | integration | Empty table is a redb state — integration layer                                                  |
| B6       | integration | Requires writing a malformed row to redb then reading it back                                    |
| B7       | integration | Requires writing a malformed row to redb then reading it back                                    |
| B8       | integration | Requires writing a malformed row to redb then reading it back                                    |
| B9       | integration | Requires a mixed table (good + bad rows) to verify abort-on-first                                |
| B10      | integration | Non-UTF-8 key must be written to redb (raw bytes), then read back                                |
| B11      | integration | Backend failure requires a real redb context without table initialization                        |
| B12      | integration | Bitwise identity requires write-then-read round-trip through redb                                |
| B13      | integration | Transaction scope verification — integration boundary                                             |
| B14      | integration | Cross-table isolation requires both tables populated in real redb                                |
| B15      | integration | Key round-trip requires real redb write + read                                                   |
| B16      | integration | Mirror of B4 for URL states — full redb round-trip                                               |
| B17      | integration | Mirror of B5 for URL states                                                                      |
| B18      | integration | Mirror of B6 for URL states — 39-byte boundary                                                   |
| B19      | integration | Mirror of B7 for URL states — 41-byte boundary                                                   |
| B20      | integration | Mirror of B8 for URL states — 0-byte boundary                                                    |
| B21      | integration | 80-byte value test for URL states                                                                |
| B22      | integration | Mirror of B9 for URL states — abort-on-first with exact field assertions                         |
| B23      | integration | Mirror of B10 for URL states                                                                     |
| B24      | integration | Mirror of B11 for URL states — BackendError for SCRAPE_TABLE                                     |
| B25      | integration | Mirror of B12 for URL states — bitwise identity                                                  |
| B26      | integration | Mirror of B13 for URL states — snapshot isolation                                                |
| B27      | integration | Mirror of B14 for URL states — cross-table isolation                                             |
| B28      | integration | Mirror of B15 for URL states — key identity                                                      |
| B29      | unit        | Pure compile-time `assert!` on `std::mem::size_of`                                               |
| B30      | unit        | Pure compile-time `assert!` on `std::mem::size_of`                                               |

**Layer counts:** 2 static + 2 unit + 26 integration = 30 behaviors
**Ratio:** ~7% static, ~7% unit, ~87% integration — justified by the bead's nature: it's an I/O-bound redb scanner with zero pure Calc functions beyond struct size asserts. Both methods are fully symmetric in coverage — no file/URL asymmetry survives.

---

## 3. BDD Scenarios

### Behavior B1: `StateReadSession::new` borrows transaction

```rust
fn session_new_holds_transaction_reference_when_constructed()
```

**Given:** A redb in-memory database with tables initialized via `DocCache::open(CacheConfig::in_memory())`, and one valid row written to `SNAPSHOTS_TABLE`
**When:** `StateReadSession::new(&read_tx)` is called with a live `redb::ReadTransaction`
**Then:** The returned `StateReadSession` has lifetime `'tx` tied to the transaction, and calling `load_file_states()` returns `Ok(map)` where `map.len() == 1`

> **Note:** The assertion is NOT "succeeds without error." It asserts a concrete `map.len() == 1` with the exact expected entry, proving the session actually works.

---

### Behavior B2: `StateReadSession` is not `Clone`

```rust
fn session_is_not_clone_by_design()
```

**Given:** The `StateReadSession` type definition
**When:** Compile-time trait check is attempted
**Then:** `Clone` is not implemented — compilation fails if code tries to clone a session

> **Implementation note:** Use `static_assertions::assert_not_impl_any!(StateReadSession<'static>: Clone);` or verify manually in a `compile_fail` test doc.

---

### Behavior B3: `StateReadSession` is not `Send`

```rust
fn session_is_not_send_by_design()
```

**Given:** The `StateReadSession` type definition
**When:** Compile-time trait check is attempted
**Then:** `Send` is not implemented — the session cannot be moved across thread boundaries

> **Implementation note:** `redb::ReadTransaction` contains `!Send` internals. If `StateReadSession` wraps `&'tx ReadTransaction`, it inherits `!Send` automatically. Verify with `static_assertions::assert_not_impl_any!(StateReadSession<'static>: Send);`

---

### Behavior B4: `load_file_states` returns complete map for all well-formed rows

```rust
fn load_file_states_returns_all_rows_when_table_has_valid_entries()
```

**Given:** A redb in-memory database with `SNAPSHOTS_TABLE` containing 3 rows:
  - Key: `"src/main.rs"` → Value: `FileStateRaw { content_hash: [0xAA; 32], last_seen_epoch_s: 1700000000 }`
  - Key: `"src/lib.rs"` → Value: `FileStateRaw { content_hash: [0xBB; 32], last_seen_epoch_s: 1700000001 }`
  - Key: `"README.md"` → Value: `FileStateRaw { content_hash: [0xCC; 32], last_seen_epoch_s: 1700000002 }`
**When:** `session.load_file_states()` is called
**Then:** `Ok(map)` where `map.len() == 3`, and:
  - `map["src/main.rs"] == FileStateRaw { content_hash: [0xAA; 32], last_seen_epoch_s: 1700000000 }`
  - `map["src/lib.rs"] == FileStateRaw { content_hash: [0xBB; 32], last_seen_epoch_s: 1700000001 }`
  - `map["README.md"] == FileStateRaw { content_hash: [0xCC; 32], last_seen_epoch_s: 1700000002 }`

---

### Behavior B5: `load_file_states` returns empty map for empty table

```rust
fn load_file_states_returns_empty_hashmap_when_table_is_empty()
```

**Given:** A redb in-memory database with `SNAPSHOTS_TABLE` initialized but containing zero rows
**When:** `session.load_file_states()` is called
**Then:** `Ok(map)` where `map.len() == 0` AND `map.is_empty() == true` AND `map == HashMap::new()`

**Assertion:** Triple-check — NOT just `is_ok()`. Assert `len() == 0`, `is_empty()`, and equality with `HashMap::new()`.

---

### Behavior B6: `load_file_states` returns `MalformedRow` for 39-byte value

```rust
fn load_file_states_returns_malformed_row_error_when_value_is_39_bytes()
```

**Given:** A redb in-memory database with `SNAPSHOTS_TABLE` containing one row where the value is 39 bytes (one byte short of 40)
  - Key: `"bad_row.dat"` → Value: `[0u8; 39]`
**When:** `session.load_file_states()` is called
**Then:** `Err(StateLoadError::MalformedRow { key, actual, expected })` where:
  - `key == "bad_row.dat"`
  - `actual == 39`
  - `expected == 40`

---

### Behavior B7: `load_file_states` returns `MalformedRow` for 41-byte value

```rust
fn load_file_states_returns_malformed_row_error_when_value_is_41_bytes()
```

**Given:** A redb in-memory database with `SNAPSHOTS_TABLE` containing one row where the value is 41 bytes
  - Key: `"oversized.bin"` → Value: `[0u8; 41]`
**When:** `session.load_file_states()` is called
**Then:** `Err(StateLoadError::MalformedRow { key, actual, expected })` where:
  - `key == "oversized.bin"`
  - `actual == 41`
  - `expected == 40`

---

### Behavior B8: `load_file_states` returns `MalformedRow` for 0-byte value

```rust
fn load_file_states_returns_malformed_row_error_when_value_is_0_bytes()
```

**Given:** A redb in-memory database with `SNAPSHOTS_TABLE` containing one row where the value is 0 bytes (empty)
  - Key: `"empty.dat"` → Value: `&[]`
**When:** `session.load_file_states()` is called
**Then:** `Err(StateLoadError::MalformedRow { key, actual, expected })` where:
  - `key == "empty.dat"`
  - `actual == 0`
  - `expected == 40`

---

### Behavior B9: `load_file_states` aborts on first malformed row — no partial map

```rust
fn load_file_states_aborts_on_first_malformed_row_without_partial_map()
```

**Given:** A redb in-memory database with `SNAPSHOTS_TABLE` containing 4 rows:
  - Key: `"good1.rs"` → Value: valid `FileStateRaw` (40 bytes, `[0xAA; 32]` + `1700000000u64.to_le_bytes()`)
  - Key: `"good2.rs"` → Value: valid `FileStateRaw` (40 bytes, `[0xBB; 32]` + `1700000001u64.to_le_bytes()`)
  - Key: `"broken.rs"` → Value: 20 bytes (malformed)
  - Key: `"good3.rs"` → Value: valid `FileStateRaw` (40 bytes, `[0xDD; 32]` + `1700000003u64.to_le_bytes()`)
**When:** `session.load_file_states()` is called
**Then:** `Err(StateLoadError::MalformedRow { key, actual, expected })` where:
  - `key == "broken.rs"`
  - `actual == 20`
  - `expected == 40`

**Critical assertion:** The result is `Err`, NOT `Ok(map)` with 3 entries. The entire load is atomic.

---

### Behavior B10: `load_file_states` returns `Utf8KeyError` for non-UTF-8 key

```rust
fn load_file_states_returns_utf8_key_error_when_key_has_invalid_utf8_bytes()
```

**Given:** A redb in-memory database with `SNAPSHOTS_TABLE` containing one row:
  - Key: `[0xFF, 0xFE, 0x00]` (invalid UTF-8) → Value: valid `FileStateRaw` (40 bytes)
**When:** `session.load_file_states()` is called
**Then:** `Err(StateLoadError::Utf8KeyError { bytes_lossy })` where:
  - `bytes_lossy == String::from_utf8_lossy(&[0xFF, 0xFE, 0x00])` — which equals `"��\0"` (two U+FFFD replacement characters followed by null character `\0`)

**Assertion:** Exact string equality against `String::from_utf8_lossy(&[0xFF, 0xFE, 0x00])`. NOT "contains", NOT "is non-empty". The exact deterministic output of `from_utf8_lossy`.

---

### Behavior B11: `load_file_states` returns `BackendError` when table cannot be opened

```rust
fn load_file_states_returns_backend_error_when_table_cannot_be_opened()
```

**Given:** A `StateReadSession` constructed with a `ReadTransaction` from a `redb::Database::create()` (in-memory) where `SNAPSHOTS_TABLE` was never created (no `initialize_tables` call)
**When:** `session.load_file_states()` is called
**Then:** `Err(StateLoadError::BackendError { operation, message })` where:
  - `operation == "open_table"` (exact `&'static str` equality)
  - `message` equals the exact `Display` representation of the redb error for opening a non-existent table

**Implementation note for test-writer:** Run `db.begin_read().unwrap().open_table(SNAPSHOTS_TABLE)` against a database where the table was never created. Capture the resulting `redb::StorageError`'s `to_string()` output. Hardcode that exact string as the expected `message` value. This is deterministic per redb version — capture once, assert exactly. Do NOT use "contains" or "is non-empty".

---

### Behavior B12: Decoded `FileStateRaw` values are bitwise-identical

```rust
fn load_file_states_decoded_values_are_bitwise_identical_to_written_bytes()
```

**Given:** A redb in-memory database with `SNAPSHOTS_TABLE` containing one row:
  - Key: `"exact_test.rs"` → Value: raw bytes of `FileStateRaw { content_hash: [0xDE, 0xAD, 0xBE, 0xEF, 0x00, ...rest_zero], last_seen_epoch_s: 0x123456789ABCDEF0 }`
**When:** `session.load_file_states()` is called
**Then:** `Ok(map)` where `map["exact_test.rs"].content_hash == [0xDE, 0xAD, 0xBE, 0xEF, 0x00, 0, ...]` AND `map["exact_test.rs"].last_seen_epoch_s == 0x123456789ABCDEF0`

**Assertion:** Byte-for-byte comparison of `content_hash` array AND exact `u64` equality of `last_seen_epoch_s`. NOT just `is_ok()`.

---

### Behavior B13: `load_file_states` uses the borrowed transaction — snapshot isolation

```rust
fn load_file_states_uses_borrowed_transaction_without_opening_new_one()
```

**Given:** A redb in-memory database with `SNAPSHOTS_TABLE` containing one valid row, and a `StateReadSession` constructed with a specific `ReadTransaction`
**When:** `session.load_file_states()` is called
**Then:** The returned data reflects the snapshot of the borrowed transaction (not a newer write)

**Test strategy:**
1. Open a write transaction, write row A (`"first.rs"` → `FileStateRaw { content_hash: [0x11; 32], last_seen_epoch_s: 100 }`), commit.
2. Open a read transaction (`read_tx_1`).
3. Open a second write transaction, write row B (`"second.rs"` → `FileStateRaw { content_hash: [0x22; 32], last_seen_epoch_s: 200 }`), commit.
4. Construct `StateReadSession::new(&read_tx_1)`.
5. Call `load_file_states()`.
6. Assert `Ok(map)` where `map.len() == 1` AND `map.contains_key("first.rs") == true` AND `map.contains_key("second.rs") == false`.

This proves the method uses the existing transaction rather than opening a fresh one.

---

### Behavior B14: `load_file_states` does not read `SCRAPE_TABLE` (cross-table isolation)

```rust
fn load_file_states_ignores_scrape_table_rows()
```

**Given:** A redb in-memory database with:
  - `SNAPSHOTS_TABLE`: 2 valid file-state rows
  - `SCRAPE_TABLE`: 3 valid URL-state rows
**When:** `session.load_file_states()` is called
**Then:** `Ok(map)` where `map.len() == 2` (only file states, no URL states leaked)

---

### Behavior B15: `load_file_states` HashMap keys are exact UTF-8 round-trips

```rust
fn load_file_states_preserves_key_strings_exactly()
```

**Given:** A redb in-memory database with `SNAPSHOTS_TABLE` containing rows with keys including:
  - `"src/üñíçödé/päth.rs"` (multi-byte UTF-8)
  - `"simple.txt"` (ASCII)
  - `"path/with spaces/and-dashes.md"`
**When:** `session.load_file_states()` is called
**Then:** `Ok(map)` where:
  - `map.contains_key("src/üñíçödé/päth.rs") == true`
  - `map.contains_key("simple.txt") == true`
  - `map.contains_key("path/with spaces/and-dashes.md") == true`
  - `map.len() == 3`

---

### Behavior B16: `load_url_states` returns complete map for all well-formed rows

```rust
fn load_url_states_returns_all_rows_when_table_has_valid_entries()
```

**Given:** A redb in-memory database with `SCRAPE_TABLE` containing 3 rows:
  - Key: `"https://docs.rs/sha2"` → Value: `UrlStateRaw { content_hash: [0x11; 32], last_seen_epoch_s: 1700000010 }`
  - Key: `"https://example.com/guide"` → Value: `UrlStateRaw { content_hash: [0x22; 32], last_seen_epoch_s: 1700000020 }`
  - Key: `"https://rust-lang.org/learn"` → Value: `UrlStateRaw { content_hash: [0x33; 32], last_seen_epoch_s: 1700000030 }`
**When:** `session.load_url_states()` is called
**Then:** `Ok(map)` where `map.len() == 3` and:
  - `map["https://docs.rs/sha2"] == UrlStateRaw { content_hash: [0x11; 32], last_seen_epoch_s: 1700000010 }`
  - `map["https://example.com/guide"] == UrlStateRaw { content_hash: [0x22; 32], last_seen_epoch_s: 1700000020 }`
  - `map["https://rust-lang.org/learn"] == UrlStateRaw { content_hash: [0x33; 32], last_seen_epoch_s: 1700000030 }`

---

### Behavior B17: `load_url_states` returns empty map for empty table

```rust
fn load_url_states_returns_empty_hashmap_when_table_is_empty()
```

**Given:** A redb in-memory database with `SCRAPE_TABLE` initialized but containing zero rows
**When:** `session.load_url_states()` is called
**Then:** `Ok(map)` where `map.len() == 0` AND `map.is_empty() == true` AND `map == HashMap::new()`

---

### Behavior B18: `load_url_states` returns `MalformedRow` for 39-byte value

```rust
fn load_url_states_returns_malformed_row_error_when_value_is_39_bytes()
```

**Given:** A redb in-memory database with `SCRAPE_TABLE` containing one row where the value is 39 bytes (one byte short of 40)
  - Key: `"https://broken-short.example.com"` → Value: `[0u8; 39]`
**When:** `session.load_url_states()` is called
**Then:** `Err(StateLoadError::MalformedRow { key, actual, expected })` where:
  - `key == "https://broken-short.example.com"`
  - `actual == 39`
  - `expected == 40`

---

### Behavior B19: `load_url_states` returns `MalformedRow` for 41-byte value

```rust
fn load_url_states_returns_malformed_row_error_when_value_is_41_bytes()
```

**Given:** A redb in-memory database with `SCRAPE_TABLE` containing one row where the value is 41 bytes
  - Key: `"https://oversized.example.com"` → Value: `[0u8; 41]`
**When:** `session.load_url_states()` is called
**Then:** `Err(StateLoadError::MalformedRow { key, actual, expected })` where:
  - `key == "https://oversized.example.com"`
  - `actual == 41`
  - `expected == 40`

---

### Behavior B20: `load_url_states` returns `MalformedRow` for 0-byte value

```rust
fn load_url_states_returns_malformed_row_error_when_value_is_0_bytes()
```

**Given:** A redb in-memory database with `SCRAPE_TABLE` containing one row where the value is 0 bytes (empty)
  - Key: `"https://empty.example.com"` → Value: `&[]`
**When:** `session.load_url_states()` is called
**Then:** `Err(StateLoadError::MalformedRow { key, actual, expected })` where:
  - `key == "https://empty.example.com"`
  - `actual == 0`
  - `expected == 40`

---

### Behavior B21: `load_url_states` returns `MalformedRow` for 80-byte value

```rust
fn load_url_states_returns_malformed_row_error_when_value_is_80_bytes()
```

**Given:** A redb in-memory database with `SCRAPE_TABLE` containing one row where the value is 80 bytes (double the expected size)
  - Key: `"https://double.example.com"` → Value: `[0u8; 80]`
**When:** `session.load_url_states()` is called
**Then:** `Err(StateLoadError::MalformedRow { key, actual, expected })` where:
  - `key == "https://double.example.com"`
  - `actual == 80`
  - `expected == 40`

---

### Behavior B22: `load_url_states` aborts on first malformed row — no partial map

```rust
fn load_url_states_aborts_on_first_malformed_row_without_partial_map()
```

**Given:** A redb in-memory database with `SCRAPE_TABLE` containing 4 rows:
  - Key: `"https://good1.example.com"` → Value: valid `UrlStateRaw` (40 bytes, `[0xAA; 32]` + `1700000010u64.to_le_bytes()`)
  - Key: `"https://good2.example.com"` → Value: valid `UrlStateRaw` (40 bytes, `[0xBB; 32]` + `1700000020u64.to_le_bytes()`)
  - Key: `"https://broken.example.com"` → Value: 20 bytes (malformed)
  - Key: `"https://good3.example.com"` → Value: valid `UrlStateRaw` (40 bytes, `[0xDD; 32]` + `1700000030u64.to_le_bytes()`)
**When:** `session.load_url_states()` is called
**Then:** `Err(StateLoadError::MalformedRow { key, actual, expected })` where:
  - `key == "https://broken.example.com"`
  - `actual == 20`
  - `expected == 40`

**Critical assertion:** The result is `Err`, NOT `Ok(map)` with 3 entries. All three fields (`key`, `actual`, `expected`) are asserted exactly — no `{ .. }` wildcard pattern.

---

### Behavior B23: `load_url_states` returns `Utf8KeyError` for non-UTF-8 key

```rust
fn load_url_states_returns_utf8_key_error_when_key_has_invalid_utf8_bytes()
```

**Given:** A redb in-memory database with `SCRAPE_TABLE` containing one row:
  - Key: `[0x80, 0x81, 0x82]` (invalid UTF-8 continuation bytes without leading bytes) → Value: valid `UrlStateRaw` (40 bytes)
**When:** `session.load_url_states()` is called
**Then:** `Err(StateLoadError::Utf8KeyError { bytes_lossy })` where:
  - `bytes_lossy == String::from_utf8_lossy(&[0x80, 0x81, 0x82])` — which equals `"��"` (three U+FFFD replacement characters)

**Assertion:** Exact string equality against `String::from_utf8_lossy(&[0x80, 0x81, 0x82])`. NOT "contains", NOT "is non-empty". The exact deterministic output of `from_utf8_lossy`.

---

### Behavior B24: `load_url_states` returns `BackendError` when table cannot be opened

```rust
fn load_url_states_returns_backend_error_when_table_cannot_be_opened()
```

**Given:** A `StateReadSession` constructed with a `ReadTransaction` from a `redb::Database::create()` (in-memory) where `SCRAPE_TABLE` was never created (no `initialize_tables` call)
**When:** `session.load_url_states()` is called
**Then:** `Err(StateLoadError::BackendError { operation, message })` where:
  - `operation == "open_table"` (exact `&'static str` equality)
  - `message` equals the exact `Display` representation of the redb error for opening a non-existent table

**Implementation note for test-writer:** Same technique as B11. Run `db.begin_read().unwrap().open_table(SCRAPE_TABLE)` against a database where the table was never created. Capture the resulting `redb::StorageError`'s `to_string()` output. Hardcode that exact string as the expected `message` value. Do NOT use "contains" or "is non-empty".

---

### Behavior B25: `load_url_states` decoded values are bitwise-identical

```rust
fn load_url_states_decoded_values_are_bitwise_identical_to_written_bytes()
```

**Given:** A redb in-memory database with `SCRAPE_TABLE` containing one row with known bytes:
  - Key: `"https://test.example.com"` → Value: `UrlStateRaw { content_hash: [0xCA, 0xFE, 0xBA, 0xBE, 0x00, ...rest_zero], last_seen_epoch_s: 0xFEDCBA9876543210 }`
**When:** `session.load_url_states()` is called
**Then:** `Ok(map)` where:
  - `map["https://test.example.com"].content_hash == [0xCA, 0xFE, 0xBA, 0xBE, 0x00, 0, ...]`
  - `map["https://test.example.com"].last_seen_epoch_s == 0xFEDCBA9876543210`

**Assertion:** Byte-for-byte comparison of `content_hash` array AND exact `u64` equality of `last_seen_epoch_s`.

---

### Behavior B26: `load_url_states` uses the borrowed transaction — snapshot isolation

```rust
fn load_url_states_uses_borrowed_transaction_without_opening_new_one()
```

**Given:** A redb in-memory database with `SCRAPE_TABLE` containing one valid row
**When:** `session.load_url_states()` is called within a stale read transaction
**Then:** The returned data reflects the snapshot of the borrowed transaction (not a newer write)

**Test strategy:**
1. Open a write transaction, write URL row A (`"https://first.com"` → `UrlStateRaw { content_hash: [0x11; 32], last_seen_epoch_s: 100 }`), commit.
2. Open a read transaction (`read_tx_1`).
3. Open a second write transaction, write URL row B (`"https://second.com"` → `UrlStateRaw { content_hash: [0x22; 32], last_seen_epoch_s: 200 }`), commit.
4. Construct `StateReadSession::new(&read_tx_1)`.
5. Call `load_url_states()`.
6. Assert `Ok(map)` where `map.len() == 1` AND `map.contains_key("https://first.com") == true` AND `map.contains_key("https://second.com") == false`.

This proves the method uses the existing transaction rather than opening a fresh one.

---

### Behavior B27: `load_url_states` does not read `SNAPSHOTS_TABLE` (cross-table isolation)

```rust
fn load_url_states_ignores_snapshots_table_rows()
```

**Given:** A redb in-memory database with:
  - `SCRAPE_TABLE`: 2 valid URL-state rows
  - `SNAPSHOTS_TABLE`: 3 valid file-state rows
**When:** `session.load_url_states()` is called
**Then:** `Ok(map)` where `map.len() == 2` (only URL states, no file states leaked)

---

### Behavior B28: `load_url_states` HashMap keys are exact UTF-8 round-trips

```rust
fn load_url_states_preserves_key_strings_exactly()
```

**Given:** A redb in-memory database with `SCRAPE_TABLE` containing rows with keys including:
  - `"https://example.com/üñíçödé"` (multi-byte UTF-8 in URL path)
  - `"https://simple.com/page"` (ASCII)
  - `"https://example.com/path with spaces"`
**When:** `session.load_url_states()` is called
**Then:** `Ok(map)` where:
  - `map.contains_key("https://example.com/üñíçödé") == true`
  - `map.contains_key("https://simple.com/page") == true`
  - `map.contains_key("https://example.com/path with spaces") == true`
  - `map.len() == 3`

---

### Behavior B29: `FileStateRaw` size is 40 bytes

```rust
fn file_state_raw_is_exactly_40_bytes()
```

**Given:** The `FileStateRaw` struct definition
**When:** `std::mem::size_of::<FileStateRaw>()` is evaluated
**Then:** The result equals 40

**Implementation note:** `const_assert_eq!(std::mem::size_of::<FileStateRaw>(), 40);` — compile-time enforced.

---

### Behavior B30: `UrlStateRaw` size is 40 bytes

```rust
fn url_state_raw_is_exactly_40_bytes()
```

**Given:** The `UrlStateRaw` struct definition
**When:** `std::mem::size_of::<UrlStateRaw>()` is evaluated
**Then:** The result equals 40

**Implementation note:** `const_assert_eq!(std::mem::size_of::<UrlStateRaw>(), 40);` — compile-time enforced.

---

## 4. Proptest Invariants

### Proptest: FileStateRaw Pod round-trip

**Invariant:** For any valid `FileStateRaw` struct, converting it to bytes via `bytemuck::bytes_of()` and back via `bytemuck::from_bytes()` produces bitwise-identical results.

```
Strategy:
  - content_hash: any [u8; 32] (proptest::array::32(..any::<u8>()))
  - last_seen_epoch_s: any u64 (proptest::num::u64::ANY)
  - Boundary seeds: u64::MIN (0), u64::MAX (18446744073709551615)

Anti-invariant:
  - Byte slices of length != 40 should fail to cast (cannot test via bytemuck
    directly since Pod cast is unsafe for wrong sizes; the load methods enforce this)
```

**Test function:** `fn file_state_raw_pod_roundtrip_is_identity_for_all_values()`

---

### Proptest: UrlStateRaw Pod round-trip

**Invariant:** For any valid `UrlStateRaw` struct, `bytes_of → from_bytes` round-trip is the identity function.

```
Strategy:
  - content_hash: any [u8; 32]
  - last_seen_epoch_s: any u64
  - Boundary seeds: u64::MIN (0), u64::MAX (18446744073709551615)

Anti-invariant:
  - Byte slices of length != 40 cannot be safely cast
```

**Test function:** `fn url_state_raw_pod_roundtrip_is_identity_for_all_values()`

---

### Proptest: `load_file_states` HashMap cardinality matches row count

**Invariant:** For N valid rows written to `SNAPSHOTS_TABLE`, `load_file_states()` returns a map with exactly N entries. This holds for any N in [0, 100].

```
Strategy:
  - N: 0..=100usize
  - Keys: unique random UTF-8 strings (proptest string strategy, max 64 chars)
  - Values: random FileStateRaw instances
  - Boundary seeds: N = 0 (empty), N = 1 (minimum non-empty), N = 100 (upper bound)

Anti-invariant:
  - If any row has wrong byte length, result must be Err(MalformedRow), not Ok with N-1 entries
```

**Test function:** `fn load_file_states_map_size_equals_row_count_for_any_n()`

---

### Proptest: `load_url_states` HashMap cardinality matches row count

**Invariant:** For N valid rows written to `SCRAPE_TABLE`, `load_url_states()` returns a map with exactly N entries. This holds for any N in [0, 100].

```
Strategy:
  - N: 0..=100usize
  - Keys: unique random URL strings (proptest string strategy, max 128 chars)
  - Values: random UrlStateRaw instances
  - Boundary seeds: N = 0 (empty), N = 1 (minimum non-empty), N = 100 (upper bound)

Anti-invariant:
  - If any row has wrong byte length, result must be Err(MalformedRow), not Ok with N-1 entries
```

**Test function:** `fn load_url_states_map_size_equals_row_count_for_any_n()`

---

## 5. Fuzz Targets

### Fuzz Target: `load_file_states` raw byte value parsing

```
Input type:  &[u8] — arbitrary byte slice simulating a row value in SNAPSHOTS_TABLE
Risk class:  Logic error — wrong length produces MalformedRow, correct length must decode
             correctly without panic. If bytemuck cast is misapplied, could trigger UB.
Corpus seeds:
  - [0u8; 0]   — empty value
  - [0u8; 39]  — one byte short
  - [0u8; 40]  — exactly right
  - [0u8; 41]  — one byte over
  - [0xFF; 40] — all-ones correct size
  - [0xAA; 40] — mixed pattern correct size
```

**Fuzz function signature:**
```rust
fn fuzz_file_state_raw_decode(input: &[u8]) {
    if input.len() == 40 {
        // Must not panic — bytemuck::from_bytes must succeed
        let state: &FileStateRaw = bytemuck::from_bytes(input);
        assert_eq!(state.content_hash.len(), 32);
    }
    // Any other length: our load methods would return MalformedRow
}
```

---

### Fuzz Target: `load_url_states` raw byte value parsing

```
Input type:  &[u8] — arbitrary byte slice simulating a row value in SCRAPE_TABLE
Risk class:  Same as above — Pod cast on wrong-sized input
Corpus seeds:  Same 6 seeds as file state target
```

**Fuzz function:** Mirror of file state fuzz target using `UrlStateRaw`.

---

## 6. Kani Harnesses

### Kani Harness: `FileStateRaw` has no padding bytes

```
Property:  size_of::<FileStateRaw>() == size_of::<[u8; 32]>() + size_of::<u64>()
           AND align_of::<FileStateRaw>() == max(align_of::<[u8; 32]>(), align_of::<u64>())
Bound:     Single struct instance
Rationale: If padding bytes exist between fields, bytemuck Pod cast will read
           uninitialized memory. Kani can prove the layout is contiguous.
```

**Kani function:**
```rust
#[kani::proof]
fn file_state_raw_no_padding() {
    assert_eq!(
        std::mem::size_of::<FileStateRaw>(),
        std::mem::size_of::<[u8; 32]>() + std::mem::size_of::<u64>()
    );
}
```

---

### Kani Harness: `UrlStateRaw` has no padding bytes

```
Property:  size_of::<UrlStateRaw>() == size_of::<[u8; 32]>() + size_of::<u64>()
Bound:     Single struct instance
Rationale: Same as FileStateRaw — padding would cause Pod cast UB
```

---

## 7. Mutation Testing Checkpoints

**Threshold: ≥90% mutation kill rate**

### Critical mutations and which tests catch them:

| #  | Mutation                                              | Caught by test                                                   | Method |
|----|-------------------------------------------------------|------------------------------------------------------------------|--------|
| M1 | Change expected size check from `40` to `39`          | `load_file_states_returns_malformed_row_error_when_value_is_39_bytes` — would wrongly accept 39-byte rows | file |
| M2 | Change expected size check from `40` to `41`          | `load_file_states_returns_malformed_row_error_when_value_is_41_bytes` — would wrongly accept 41-byte rows | file |
| M3 | Change expected size check from `40` to `39`          | `load_url_states_returns_malformed_row_error_when_value_is_39_bytes` — would wrongly accept 39-byte rows | URL |
| M4 | Change expected size check from `40` to `41`          | `load_url_states_returns_malformed_row_error_when_value_is_41_bytes` — would wrongly accept 41-byte rows | URL |
| M5 | Remove early-return on MalformedRow (file)            | `load_file_states_aborts_on_first_malformed_row_without_partial_map` — would return partial map | file |
| M6 | Remove early-return on MalformedRow (URL)             | `load_url_states_aborts_on_first_malformed_row_without_partial_map` — would return partial map | URL |
| M7 | Replace UTF-8 validation with `unwrap()` (file)       | `load_file_states_returns_utf8_key_error_when_key_has_invalid_utf8_bytes` — would panic | file |
| M8 | Replace UTF-8 validation with `unwrap()` (URL)        | `load_url_states_returns_utf8_key_error_when_key_has_invalid_utf8_bytes` — would panic | URL |
| M9 | Swap `SNAPSHOTS_TABLE` / `SCRAPE_TABLE` targets       | `load_file_states_ignores_scrape_table_rows` + `load_url_states_ignores_snapshots_table_rows` — would return wrong data | both |
| M10| Return empty map instead of scanning (file)           | `load_file_states_returns_all_rows_when_table_has_valid_entries` — map would be empty | file |
| M11| Return empty map instead of scanning (URL)            | `load_url_states_returns_all_rows_when_table_has_valid_entries` — map would be empty | URL |
| M12| Remove `BackendError` mapping, propagate raw redb err (file) | `load_file_states_returns_backend_error_when_table_cannot_be_opened` — error variant mismatch | file |
| M13| Remove `BackendError` mapping, propagate raw redb err (URL) | `load_url_states_returns_backend_error_when_table_cannot_be_opened` — error variant mismatch | URL |
| M14| Flip `<` to `<=` in byte length comparison            | `load_file_states_returns_malformed_row_error_when_value_is_39_bytes` + 41 + URL equivalents — boundary shift | both |
| M15| Remove key from MalformedRow error                    | Compile failure (struct field removed) or test asserting `key == "..."` fails | both |
| M16| Replace `bytemuck::from_bytes` with `from_bytes_mut`  | Compile-time catch (immutable borrow vs mutable) — static layer | both |
| M17| Replace `bytes_lossy` with arbitrary non-empty string | `load_file_states_returns_utf8_key_error...` + `load_url_states_returns_utf8_key_error...` — exact string comparison fails | both |
| M18| Swap `actual`/`expected` fields in MalformedRow       | `load_file_states_returns_malformed_row_error_when_value_is_39_bytes` (actual==39 != expected==40) + URL equivalents — field values swapped | both |
| M19| Fill MalformedRow `actual` with 0                     | `load_url_states_aborts_on_first_malformed_row_without_partial_map` asserts `actual == 20` explicitly | URL |
| M20| Fill MalformedRow `expected` with 999                 | `load_url_states_aborts_on_first_malformed_row_without_partial_map` asserts `expected == 40` explicitly | URL |

### Mutation kill strategy:
- Every branch in `load_file_states` AND `load_url_states` has a dedicated test
- Both methods have symmetric coverage — no file/URL asymmetry that mutations can hide in
- Every error variant has a test that matches on the specific variant AND checks exact field values
- Empty table tests catch "skip scan" mutations
- Atomicity tests (B9, B22) catch "continue-on-error" mutations
- `bytes_lossy` assertions use exact `from_utf8_lossy` output — no vague "contains" patterns
- BackendError tests exist for BOTH methods — no missing error path
- URL abort test (B22) asserts all 3 fields explicitly — no `{ .. }` wildcard

---

## 8. Combinatorial Coverage Matrix

### `load_file_states`

| Scenario                                  | Input Class                    | Expected Output                                         | Layer       |
|-------------------------------------------|--------------------------------|---------------------------------------------------------|-------------|
| Happy path: 3 valid rows                  | 3 × valid 40-byte values       | `Ok(HashMap {3 entries, exact values})`                 | integration |
| Happy path: 1 valid row                   | 1 × valid 40-byte value        | `Ok(HashMap {1 entry, exact value})`                    | integration |
| Empty table                               | 0 rows                         | `Ok(HashMap::new())`                                    | integration |
| Malformed: value too short (39 bytes)     | 1 × 39-byte value              | `Err(MalformedRow { key: "bad_row.dat", actual: 39, expected: 40 })` | integration |
| Malformed: value too long (41 bytes)      | 1 × 41-byte value              | `Err(MalformedRow { key: "oversized.bin", actual: 41, expected: 40 })` | integration |
| Malformed: empty value (0 bytes)          | 1 × 0-byte value               | `Err(MalformedRow { key: "empty.dat", actual: 0, expected: 40 })` | integration |
| Mixed: good rows + 1 bad row              | 2 valid + 1 malformed          | `Err(MalformedRow { key: "broken.rs", actual: 20, expected: 40 })` | integration |
| Non-UTF-8 key                             | key = `[0xFF, 0xFE, 0x00]`     | `Err(Utf8KeyError { bytes_lossy: from_utf8_lossy(&[0xFF,0xFE,0x00]) })` | integration |
| Table does not exist                      | No SNAPSHOTS_TABLE in db       | `Err(BackendError { operation: "open_table", message: <exact_redb_err> })` | integration |
| Bitwise identity of decoded values        | Known bytes → read back        | `Ok(map)` with exact byte-for-byte match                | integration |
| Snapshot isolation (reads stale tx)       | Write after tx opened          | `Ok(map)` with `len == 1`, only pre-tx row present      | integration |
| Cross-table isolation                     | SNAPSHOTS + SCRAPE rows        | `Ok(map)` with `len == 2`, only SNAPSHOTS entries       | integration |
| Multi-byte UTF-8 keys                     | Keys with üñíçödé chars        | `Ok(map)` with `len == 3`, exact key strings            | integration |
| Proptest: cardinality for N rows          | N ∈ [0, 100]                   | `Ok(map)` with `map.len() == N`                         | proptest    |
| Proptest: Pod round-trip identity         | Any FileStateRaw               | `bytes_of(from_bytes(x)) == bytes_of(x)`                | proptest    |
| Struct size: FileStateRaw == 40           | Compile-time                   | `size_of == 40`                                         | unit        |
| Struct size: UrlStateRaw == 40            | Compile-time                   | `size_of == 40`                                         | unit        |
| Fuzz: arbitrary &[u8] input               | Any byte slice                 | len==40: no panic; else: MalformedRow path              | fuzz        |

### `load_url_states`

| Scenario                                  | Input Class                    | Expected Output                                         | Layer       |
|-------------------------------------------|--------------------------------|---------------------------------------------------------|-------------|
| Happy path: 3 valid rows                  | 3 × valid 40-byte values       | `Ok(HashMap {3 entries, exact values})`                 | integration |
| Happy path: 1 valid row                   | 1 × valid 40-byte value        | `Ok(HashMap {1 entry, exact value})`                    | integration |
| Empty table                               | 0 rows                         | `Ok(HashMap::new())`                                    | integration |
| Malformed: value too short (39 bytes)     | 1 × 39-byte value              | `Err(MalformedRow { key: "https://broken-short.example.com", actual: 39, expected: 40 })` | integration |
| Malformed: value too long (41 bytes)      | 1 × 41-byte value              | `Err(MalformedRow { key: "https://oversized.example.com", actual: 41, expected: 40 })` | integration |
| Malformed: empty value (0 bytes)          | 1 × 0-byte value               | `Err(MalformedRow { key: "https://empty.example.com", actual: 0, expected: 40 })` | integration |
| Malformed: double-size value (80 bytes)   | 1 × 80-byte value              | `Err(MalformedRow { key: "https://double.example.com", actual: 80, expected: 40 })` | integration |
| Mixed: good rows + 1 bad row              | 2 valid + 1 malformed          | `Err(MalformedRow { key: "https://broken.example.com", actual: 20, expected: 40 })` | integration |
| Non-UTF-8 key                             | key = `[0x80, 0x81, 0x82]`     | `Err(Utf8KeyError { bytes_lossy: from_utf8_lossy(&[0x80,0x81,0x82]) })` | integration |
| Table does not exist                      | No SCRAPE_TABLE in db          | `Err(BackendError { operation: "open_table", message: <exact_redb_err> })` | integration |
| Bitwise identity of decoded values        | Known bytes → read back        | Exact byte match                                        | integration |
| Snapshot isolation (reads stale tx)       | Write after tx opened          | `Ok(map)` with `len == 1`, only pre-tx row present      | integration |
| Cross-table isolation                     | SCRAPE + SNAPSHOTS rows        | `Ok(map)` with `len == 2`, only SCRAPE entries          | integration |
| Multi-byte UTF-8 keys                     | Keys with üñíçödé chars        | `Ok(map)` with `len == 3`, exact key strings            | integration |
| Proptest: cardinality for N rows          | N ∈ [0, 100]                   | `Ok(map)` with `map.len() == N`                         | proptest    |
| Proptest: Pod round-trip identity         | Any UrlStateRaw                | `bytes_of(from_bytes(x)) == bytes_of(x)`                | proptest    |
| Fuzz: arbitrary &[u8] input               | Any byte slice                 | len==40: no panic; else: MalformedRow path              | fuzz        |

---

## Error Variant Coverage Matrix

Every variant in `StateLoadError` has at least one test with exact field assertions, for **both** `load_file_states` and `load_url_states`:

| Error Variant                | `load_file_states` tests                                                                                      | `load_url_states` tests                                                                                       | Asserted Fields                    |
|------------------------------|---------------------------------------------------------------------------------------------------------------|---------------------------------------------------------------------------------------------------------------|------------------------------------|
| `MalformedRow`               | `load_file_states_returns_malformed_row_error_when_value_is_39_bytes` (exact key, actual:39, expected:40)    | `load_url_states_returns_malformed_row_error_when_value_is_39_bytes` (exact key, actual:39, expected:40)    | `key`, `actual`, `expected`        |
|                              | `load_file_states_returns_malformed_row_error_when_value_is_41_bytes` (exact key, actual:41, expected:40)    | `load_url_states_returns_malformed_row_error_when_value_is_41_bytes` (exact key, actual:41, expected:40)    |                                    |
|                              | `load_file_states_returns_malformed_row_error_when_value_is_0_bytes` (exact key, actual:0, expected:40)      | `load_url_states_returns_malformed_row_error_when_value_is_0_bytes` (exact key, actual:0, expected:40)      |                                    |
|                              | `load_file_states_aborts_on_first_malformed_row_without_partial_map` (exact key, actual:20, expected:40)     | `load_url_states_aborts_on_first_malformed_row_without_partial_map` (exact key, actual:20, expected:40)     |                                    |
| `Utf8KeyError`               | `load_file_states_returns_utf8_key_error_when_key_has_invalid_utf8_bytes` (exact `from_utf8_lossy` output)   | `load_url_states_returns_utf8_key_error_when_key_has_invalid_utf8_bytes` (exact `from_utf8_lossy` output)   | `bytes_lossy`                      |
| `BackendError`               | `load_file_states_returns_backend_error_when_table_cannot_be_opened` (operation: "open_table", exact message)| `load_url_states_returns_backend_error_when_table_cannot_be_opened` (operation: "open_table", exact message)| `operation`, `message`             |

**No error variant is tested with only `is_err()`.** Every error test destructures the variant and asserts specific field values. Both methods have symmetric error-variant coverage — no `BackendError` gap.

---

## Defect Fix Register

The following defects from the `test-plan-review.md` rejection are addressed in this revision:

| Defect ID | Severity  | Description                                                        | Fix Applied                                                                                     |
|-----------|-----------|--------------------------------------------------------------------|-------------------------------------------------------------------------------------------------|
| LETHAL-1  | LETHAL    | `load_url_states` has no `BackendError` scenario                   | Added B24 + row in error-variant matrix + mutation checkpoint M13                               |
| MAJOR-1   | MAJOR     | `Utf8KeyError` `bytes_lossy` asserts vague "contains"/"non-empty" | B10 and B23 now assert exact `String::from_utf8_lossy(&input_bytes)` equality                  |
| MAJOR-2   | MAJOR     | `BackendError` `message` asserts only "non-empty"                 | B11 and B24 now specify exact redb error message capture + equality                             |
| MAJOR-3   | MAJOR     | URL abort test uses `{ .. }` wildcard                              | B22 now asserts `key == "https://broken.example.com"`, `actual == 20`, `expected == 40`       |
| MAJOR-4   | MAJOR     | `load_url_states` missing 0/39/41-byte boundary tests              | Added B18 (39 bytes), B19 (41 bytes), B20 (0 bytes)                                             |
| MAJOR-5   | MAJOR     | Mutation survivor: `!= 40` → `> 40` in URL path                    | Killed by B18 (39-byte URL test) + M3 checkpoint                                               |
| MAJOR-6   | MAJOR     | Mutation survivor: BackendError branch deletion in URL path        | Killed by B24 (URL BackendError test) + M13 checkpoint                                          |
| MINOR-1   | MINOR     | B1 "succeeds without error" is `is_ok()` in disguise               | B1 now asserts concrete `map.len() == 1` with exact entry                                      |
| MINOR-2   | MINOR     | Summary says 14 behaviors; inventory lists 16                      | Summary now shows 30 behaviors matching inventory B1-B30                                        |
| MINOR-3   | MINOR     | Summary trophy counts don't match detailed allocation              | Summary now shows 2 static / 2 unit / 26 integration matching trophy table                     |
| MINOR-4   | MINOR     | Resource cleanup not explicit                                      | Added explicit note: all tests use in-memory redb, no tempfile, no cleanup needed               |
| MINOR-5   | MINOR     | `u64::MIN`/`u64::MAX` not named as boundary seeds                 | Proptest strategies now explicitly list `u64::MIN (0)` and `u64::MAX` as boundary seeds        |

---

## Open Questions

1. **Non-UTF-8 key insertion into redb:** The contract specifies that `Utf8KeyError` should be returned for non-UTF-8 keys. However, redb keys are `&[u8]`, so inserting raw non-UTF-8 bytes is straightforward. The test-writer must use the raw `write_tx.open_table(SCRAPE_TABLE)` + `table.insert(&non_utf8_bytes[..], &valid_value[..])` approach, bypassing any string-typed API. Verify this is possible with the redb 2.x API.

2. **Table-not-found test strategy:** Behaviors B11 and B24 require testing against a database where the target table was never created. The test-writer should create a `redb::Database::create()` (in-memory) and open a read transaction WITHOUT calling `initialize_tables()`. This should cause `open_table()` to fail. Verify that redb 2.x returns a discoverable error (not panic) in this case.

3. **`bytemuck` dependency:** The contract assumes `bytemuck` will be added to `Cargo.toml`. The test-writer should verify this dependency exists before writing tests. If it's missing, add `bytemuck = "1"` to dependencies first.

4. **`StateLoadError` location:** The contract defines `StateLoadError` but doesn't specify which module file it lives in. Likely candidates: `cache/mod.rs` or a new `cache/state.rs`. Test-writer should check where it's actually defined.

5. **Session not `Send` verification:** If `StateReadSession` wraps `&'tx ReadTransaction`, it inherits `!Send` from redb. But if `ReadTransaction` becomes `Send` in a future redb version, this test would break. Consider whether to add a PhantomData marker to explicitly enforce `!Send`.

6. **`BackendError` message exact value:** The exact error message string depends on the redb version. Test-writer must capture the actual `to_string()` output of the redb error for "table does not exist" and hardcode it. If redb is upgraded, this assertion may need updating. Consider using a helper constant or build-time capture.

# Implementation Summary: cdocs-bvh

## Bead Metadata
- **bead_id:** cdocs-bvh
- **bead_title:** data: add archive-safe persisted output records and rkyv derives
- **status:** COMPLETE

## What Was Implemented

### 1. Persisted Record Module (`centralized-docs/src/persisted.rs`)

A new module defining append-only persisted record types with rkyv 0.8 derives for zero-copy deserialization.

**Record Types (23 types):**

- **Analysis family:** `PersistedHeading`, `PersistedLinkKind`, `PersistedLink`, `PersistedAnalysis`, `PersistedFailedFile`, `PersistedAnalyzeResult`
- **Transform family:** `PersistedTransformError`, `PersistedTransformResult`
- **Chunk family:** `PersistedChunkType`, `PersistedChunkLevel`, `PersistedChunk`, `PersistedChunksResult`
- **Scrape family:** `PersistedHeader`, `PersistedPageFilterStatus`, `PersistedScrapedPage`, `PersistedScrapeResult`
- **Watch family:** `PersistedPageHash`, `PersistedChangeKind`, `PersistedPageChange`, `PersistedChangeSummary`, `PersistedSnapshot`, `PersistedChangePlan`
- **Assign family:** `PersistedIdMapping`

**Error Type:** `PersistError` with 8 variants:
- `EmptyField` — required string field empty/whitespace
- `OutOfRange` — numeric field outside bounds
- `SchemaVersionMismatch` — unsupported schema version
- `SerializationFailed` — rkyv buffer errors
- `DeserializationFailed` — corrupted bytes
- `UnknownVariant` — unmapped enum discriminant
- `NonFiniteFloat` — NaN/Inf where finite required
- `InvalidHashLength` — content_hash not 32 bytes

**Infallible Conversions (Runtime → Persisted):** 29 `*_to_persisted()` functions
- `Arc<str>` → `String`
- `HashMap<K,V>` → sorted `Vec<(K,V)>` via `itertools::sorted_by`
- `DateTime<Utc>` → `i64` unix epoch seconds (`.timestamp()`)
- `BTreeMap<String, PageHash>` → `Vec<(String, PersistedPageHash)>` (preserving sort)
- All top-level batch records carry `schema_version: 1`

**Fallible Conversions (Persisted → Runtime):** 29 `persisted_*_to_runtime()` functions
- Validate `schema_version == 1` first (fail-fast on version mismatch)
- Non-empty string validation on identifiers: `source_path`, `title`, `category`, `chunk_id`, `doc_id`, `content`, `target`, `id`, etc.
- Range validation: heading levels 1..=6, token_count > 0
- Float validation: `density_score.is_finite()`
- `i64` → `DateTime<Utc>` via `Utc.timestamp_opt(secs, 0).single().ok_or_else(...)`

### 2. Dependency Addition (`centralized-docs/Cargo.toml`)

```toml
rkyv = { version = "0.8", features = ["std", "bytecheck"] }
```

### 3. Module Registration (`centralized-docs/src/lib.rs`)

Added `pub mod persisted;` to the public module list.

### 4. Test File (`centralized-docs/tests/persisted_tests.rs`)

91 integration tests covering behaviors B01–B68:
- **B01–B30**: Infallible conversion correctness (30 tests)
- **B31–B61**: Fallible conversion happy/error paths (31 tests)
- **B62**: rkyv round-trip for all 13 record types (13 tests)
- **B63**: Deterministic serialization (2 tests)
- **B64–B67**: Invalid archived bytes (truncated, bit-flipped, zeroed, random noise) (4 tests)
- **B68**: Deterministic frontmatter ordering (1 test)
- **Full pipeline**: Runtime → persisted → rkyv → persisted → runtime (10 tests)

### Test Fixes Applied

Fixed incorrect epoch second values in 2 test assertions:
- `2025-01-15T10:30:00Z` = `1736937000` (tests originally had `1736931000`, which is `2025-01-15T08:50:00Z`)

## Constraint Adherence

| Constraint | Evidence |
|---|---|
| **Zero unwrap/expect/panic** | All non-test code uses `Result`, `match`, `map`, `and_then`, `?`. Test code allows unwrap via module-level `#![allow]`. |
| **Zero mut** | No `let mut` in `persisted.rs`. Sorting done via `itertools::sorted_by` (pure, no mutation). |
| **Data → Calc → Actions** | Pure data types + pure conversion functions. No I/O in persisted module. |
| **Make illegal states unrepresentable** | `PersistError` enum covers all failure modes exhaustively. Schema version checked first. |
| **Expression-based** | All conversions use expression chains with `?` operator and combinators. |
| **Clippy flawless** | `cargo clippy -p centralized-docs --lib -- -D warnings -D clippy::unwrap_used -D clippy::panic -D clippy::expect_used -W clippy::pedantic` passes with zero warnings. |

## Files Changed

| File | Action |
|---|---|
| `centralized-docs/src/persisted.rs` | Created (~750 lines) |
| `centralized-docs/src/lib.rs` | Added `pub mod persisted;` |
| `centralized-docs/Cargo.toml` | Added `rkyv = "0.8"` dependency |
| `centralized-docs/tests/persisted_tests.rs` | Fixed 2 incorrect epoch second values |

## Test Results

```
cargo fmt -p centralized-docs --check: OK
cargo clippy -p centralized-docs --lib: OK (0 warnings)
cargo test -p centralized-docs --test persisted_tests: 91 passed, 0 failed
```

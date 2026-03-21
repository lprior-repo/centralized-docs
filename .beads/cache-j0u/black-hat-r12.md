# Black-Hat Audit Round 12

**Scope**: `centralized-docs/src/cache/` (15 files) + `centralized-docs/src/errors/` (6 files)
**Date**: 2026-03-21
**Method**: Line-by-line adversarial review, zero prior context
**Skill**: `black-hat-reviewer` not found — conducted independently

---

## STATUS: REJECTED

**13/17 constraints PASS | 4 constraints FAIL | 2 advisory defects**

---

## Files Audited (21 files, every line read)

### cache/
| File | Lines | Role |
|------|-------|------|
| `mod.rs` | 28 | Re-exports, module doc |
| `config.rs` | 95 | Types: CacheBackend, CacheConfig, CacheType, CacheStats, limits |
| `hash.rs` | 118 | SHA-256 hashing, redb table helpers, size validation |
| `store/mod.rs` | 261 | DocCache: typed get/put, get_or_compute, stats, clear_all |
| `store/dedup.rs` | 194 | Lock-free in-flight dedup via DashMap + OnceLock |
| `tests/mod.rs` | 6 | Test module declarations |
| `tests/basic.rs` | 145 | Roundtrip, miss, struct, stats, clear, in-memory, disabled |
| `tests/adversarial.rs` | 234 | ATTACKs 1-5, 10 |
| `tests/adversarial_edge.rs` | 135 | ATTACKs 12-15 |
| `tests/adviserial_stress.rs` | 114 | ATTACKs 8-9 |
| `tests/dedup.rs` | 192 | Dedup correctness, ATTACK 11/11b |
| `tests/limits.rs` | 131 | Key/value size limits, ATTACKs 6-7 |

### errors/
| File | Lines | Role |
|------|-------|------|
| `mod.rs` | 140 | DocTransformerError top-level enum, clippy deny directives |
| `cache.rs` | 20 | CacheError: KeyTooLarge, ValueTooLarge, BackendError |
| `config.rs` | 24 | ConfigError: NotFound, InvalidFormat, MissingKey, InvalidValue, CategoryRule |
| `embedding.rs` | 33 | EmbeddingError: GenerationFailed, ApiError, RateLimited, etc. |
| `transformer.rs` | 89 | DocumentError, IndexError, IoError |
| `validation.rs` | 49 | ValidationError: EmptyQuery, QueryTooLong, BrokenLink, etc. |

---

## 17-Constraint Verification

### C1: ZERO_PANICS — PASS
No `unwrap()`, `expect()`, or `panic!()` in any production code path. All fallible operations use `?` or explicit error handling. `catch_unwind` in `store/mod.rs:207` is defensive — it wraps **user-provided** compute closures, not production logic. Test code uses `expect` (permitted by doctrine).

### C2: ZERO_UNSAFE — PASS
Zero `unsafe` blocks across all 21 files. `errors/mod.rs:5` enforces `#![forbid(unsafe_code)]` at module level.

### C3: ZERO_MUT — **FAIL**

6 `mut` bindings in production code:

| File:Line | Binding | Reason |
|-----------|---------|--------|
| `hash.rs:100` | `let mut hasher = Sha256::new()` | sha2 crate requires `&mut Digest` for `update()` |
| `hash.rs:103` | `let mut array = [0u8; 32]` | `copy_from_slice` requires `&mut [u8]` |
| `store/mod.rs:97` | `let mut table = write_tx.open_table(...)` | redb requires `&mut Table` for insert |
| `store/mod.rs:119` | `let mut table = write_tx.open_table(...)` | same |
| `store/mod.rs:141` | `let mut table = write_tx.open_table(...)` | same |
| `store/mit.rs:255` | `let mut table = write_tx.open_table(...)` | same |

**Mitigating factor**: All 6 instances are structurally imposed by external crate APIs (sha2, redb). No internal state mutation exists. The functional core / imperative shell boundary is respected — mutation occurs only at the I/O boundary with external crates.

**Severity**: LOW — ecosystem-imposed, not design negligence.

### C4: ZERO_FOR_WHILE — **FAIL**

1 `loop` in production code:

| File:Line | Pattern | Reason |
|-----------|---------|--------|
| `dedup.rs:50-65` | `loop { if slot.get() { ... } thread::yield_now() }` | `OnceLock` has no blocking wait API |

The spin-wait has a 30s deadline (`dedup.rs:49`) and only runs when the owner thread is performing real I/O (network/disk), making CPU overhead negligible. Alternative (condvar) would add complexity without benefit.

**Severity**: LOW — necessary for sync OnceLock pattern; has timeout guard.

### C5: ALL_ERRORS_EXPLICIT — PASS
All fallible operations return `Result`. Cache write failures in `dedup.rs:161-167` are intentionally swallowed (DEFECT-007 acknowledged in code) because the compute result must still be returned to callers. The `eprintln!` is an advisory defect (see below).

### C6: NON_EXHAUSTIVE_ENUMS — PASS
All 14 public enums across both modules carry `#[non_exhaustive]`:
CacheBackend, CacheConfig, CacheType, CacheStats, InFlightKey, InflightDecision, CacheError, ConfigError, EmbeddingError, DocumentError, IndexError, IoError, ValidationError, DocTransformerError.

### C7: CONTENT_ADDRESSED — **FAIL**

Module docstring (`cache/mod.rs:11`) states "Cache keys are SHA-256 hashes of input." The implementation uses SHA-256 correctly. **However**, the public API accepts raw `&[u8]` keys with no compile-time guarantee:

```rust
pub fn put_document<V: Serialize>(&self, key: &[u8], value: &V) -> Result<()>
pub fn get_document<V: DeserializeOwned>(&self, key: &[u8]) -> Result<Option<V>>
```

A caller could pass arbitrary non-hash bytes. A `ContentHash([u8; 32])` newtype would make illegal states unrepresentable (DDD parse-don't-validate principle from AGENTS.md).

**Severity**: MEDIUM — violates DDD parse-at-boundary doctrine. Current design relies on module-level documentation contract rather than type-level enforcement.

### C8: SIZE_LIMITS_ENFORCED — PASS
All 5 write paths validate key size via `validate_key_size()`:
- `put_document` (store/mod.rs:94), `put_scrape` (store/mod.rs:116), `put_transform` (store/mod.rs:138), `get_or_compute` (store/mod.rs:192), `put_raw` (store/mod.rs:247)

All value writes go through `validate_and_insert()` (hash.rs:64-72) which calls `validate_value_size()`. MAX_KEY_SIZE=256, MAX_VALUE_SIZE=10MB.

### C9: EMPTY_KEY_REJECTED — PASS
`validate_key_size` (hash.rs:40) rejects `len == 0` with `CacheError::KeyTooLarge`. All write paths call this. ATTACK 5 test confirms rejection across all 3 cache types and `get_or_compute`.

### C10: CLIPPY_PEDANTIC — PASS
`errors/mod.rs:4` declares `#![warn(clippy::pedantic)]`. Production code uses targeted `#[allow(...)]` suppressions:
- `clippy::result_large_err` — DocCache::open returns Result<Self>
- `clippy::too_many_arguments` — finalize_compute has 7 params (dedup internals)
- `clippy::large_enum_variant` — DocTransformerError wraps diverse error types

All suppressions are justified.

### C11: THISERROR_FOR_ERRORS — PASS
All 8 error enums derive `thiserror::Error` with descriptive messages including context fields.

### C12: CLONE_ON_ERRORS — **FAIL**

7 of 8 error types derive `Clone, PartialEq, Eq`. **DocTransformerError** (errors/mod.rs:35-87) does NOT:

```rust
#[derive(Debug, Error)]  // Missing: Clone, PartialEq, Eq
pub enum DocTransformerError { ... }
```

This prevents equality assertions in tests for the top-level error type and breaks the pattern established by all leaf error types.

**Severity**: MEDIUM — easy to fix, inconsistent with module convention.

### C13: IDEMPOTENT_OPERATIONS — PASS
`put_*` methods use redb's upsert semantics (idempotent). `get_or_compute` returns cached value if exists. `clear_all` deletes and recreates tables (safe to call N times). `initialize_tables` uses `open_table` (create-if-not-exists).

### C14: TOCTOU_SAFE — PASS
DEFECT-004 is fixed. `dedup.rs:173-177` explicitly documents that in-flight entries are NOT removed after completion. DashMap's `entry()` API provides atomic check-and-insert, preventing the classic TOCTOU race. Waiters that arrive after the OnceLock is set still receive the value immediately.

### C15: NO_MAGIC_STRINGS — PASS
All cache operations use typed enums: `CacheBackend` (Memory/File), `CacheType` (Document/Scrape/Transform). Table names are constants: `DOCUMENT_CACHE_TABLE`, `SCRAPE_CACHE_TABLE`, `TRANSFORM_CACHE_TABLE`, `METADATA_TABLE`.

### C16: CLIPPY_DENY_UNWRAP — PASS
`errors/mod.rs:1-3` enforces:
```rust
#![deny(clippy::unwrap_used)]
#![deny(clippy::expect_used)]
#![deny(clippy::panic)]
```

### C17: TEST_COVERAGE — PASS
16 adversarial tests (ATTACK 1-15) + 8 basic/dedup/limit tests covering:
- Extreme concurrency (100 threads, 50 waiters)
- Crypto collision (10k SHA-256 inputs)
- Disk corruption (byte-level .redb tampering)
- Empty/oversized keys and values
- Concurrent clear during read and compute
- Double-open, type mismatch, special bytes
- Error propagation races, all-tables-disabled path

---

## Advisory Defects

### ADV-001: `eprintln!` in production code
**Location**: `dedup.rs:163`
**Issue**: Uses `eprintln!("WARN: cache write failed...")` instead of `log::warn!`.
**Risk**: No structured logging in production; output goes to stderr unconditionally.
**Fix**: Replace with `tracing::warn!` or `log::warn!` gated behind a feature flag.

### ADV-002: `cache/` module lacks clippy deny directives
**Location**: `cache/mod.rs` (top of file)
**Issue**: `errors/mod.rs` has `#![deny(clippy::unwrap_used)]` etc., but `cache/mod.rs` does not. If the crate-level lib.rs also lacks these, the cache module has weaker compile-time guarantees.
**Fix**: Either add deny directives to `cache/mod.rs` or verify they exist at crate root.

---

## Defect Summary

| ID | Constraint | Severity | File(s) | Description |
|----|-----------|----------|---------|-------------|
| D-01 | C3 ZERO_MUT | LOW | hash.rs, store/mod.rs | 6 `mut` bindings imposed by sha2/redb APIs |
| D-02 | C4 ZERO_FOR_WHILE | LOW | dedup.rs:50 | `loop` spin-wait for OnceLock (has 30s timeout) |
| D-03 | C7 CONTENT_ADDRESSED | MEDIUM | store/mod.rs (pub API) | Raw `&[u8]` keys lack type-safe hash wrapper |
| D-04 | C12 CLONE_ON_ERRORS | MEDIUM | errors/mod.rs:35 | DocTransformerError missing Clone, PartialEq, Eq |
| D-05 | ADV-001 | LOW | dedup.rs:163 | eprintln! in production (should use log::warn!) |
| D-06 | ADV-002 | LOW | cache/mod.rs | No clippy deny directives at module level |

---

## Verdict

**STATUS: REJECTED**

4 hard constraints fail. D-03 and D-04 are genuine design defects that should be fixed:
- D-03: Introduce `ContentHash([u8; 32])` newtype, parse at API boundary.
- D-04: Add `Clone, PartialEq, Eq` derives to `DocTransformerError`.

D-01 and D-02 are low-severity pragmatic violations imposed by the Rust ecosystem. Resolution would require either upstream crate changes or significant architectural rework (e.g., async runtime for blocking wait). These are acceptable trade-offs given the alternatives.

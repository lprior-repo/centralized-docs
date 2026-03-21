# Black-Hat Audit Round 14

**Scope**: `centralized-docs/src/cache/` (12 files) + `centralized-docs/src/errors/` (6 files)
**Date**: 2026-03-21
**Method**: Line-by-line adversarial review, zero prior context, every line of every file read
**Skill**: `black-hat-reviewer` not found in available skills -- conducted independently

---

## STATUS: REJECTED

**13/17 constraints PASS | 4 constraints FAIL | 2 advisory defects**

---

## Files Audited (18 files, every line read)

### cache/
| File | Lines | Role |
|------|-------|------|
| `mod.rs` | 28 | Re-exports, module doc |
| `config.rs` | 95 | CacheBackend, CacheConfig, CacheType, CacheStats, size limits |
| `hash.rs` | 118 | SHA-256 hashing, redb table helpers, size validation |
| `store/mod.rs` | 261 | DocCache: typed get/put, get_or_compute, stats, clear_all |
| `store/dedup.rs` | 194 | Lock-free in-flight dedup via DashMap + OnceLock |
| `tests/mod.rs` | 6 | Test module declarations |
| `tests/basic.rs` | 145 | Roundtrip, miss, struct, stats, clear, in-memory, disabled |
| `tests/adversarial.rs` | 234 | ATTACKs 1-5, 10 |
| `tests/adversarial_edge.rs` | 135 | ATTACKs 12-15 |
| `tests/adversarial_stress.rs` | 114 | ATTACKs 8-9 |
| `tests/dedup.rs` | 192 | Dedup correctness, ATTACK 11/11b |
| `tests/limits.rs` | 131 | Key/value size limits, ATTACKs 6-7 |

### errors/
| File | Lines | Role |
|------|-------|------|
| `mod.rs` | 140 | DocTransformerError top-level enum, clippy deny directives |
| `cache.rs` | 20 | CacheError: KeyTooLarge, ValueTooLarge, BackendError |
| `config.rs` | 24 | ConfigError: NotFound, InvalidFormat, MissingKey, InvalidValue |
| `embedding.rs` | 33 | EmbeddingError: GenerationFailed, ApiError, RateLimited, etc. |
| `transformer.rs` | 89 | DocumentError, IndexError, IoError |
| `validation.rs` | 49 | ValidationError: EmptyQuery, QueryTooLong, BrokenLink, etc. |

---

## 17-Constraint Verification

### C1: ZERO_PANICS -- PASS
No `unwrap()`, `expect()`, or `panic!()` in any production code path. All fallible operations use `?` or explicit `Result`. `catch_unwind` in `store/mod.rs:207` wraps user-provided compute closures defensively -- not production logic. Test code uses `expect` (permitted by doctrine).

### C2: ZERO_UNSAFE -- PASS
Zero `unsafe` blocks across all 18 files. `errors/mod.rs:5` enforces `#![forbid(unsafe_code)]`.

### C3: ZERO_MUT -- **FAIL**

6 `mut` bindings in production code:

| File:Line | Binding | Reason |
|-----------|---------|--------|
| `hash.rs:100` | `let mut hasher = Sha256::new()` | sha2 requires `&mut Digest` for `update()` |
| `hash.rs:103` | `let mut array = [0u8; 32]` | `copy_from_slice` requires `&mut [u8]` |
| `store/mod.rs:97` | `let mut table = write_tx.open_table(...)` | redb requires `&mut Table` for insert |
| `store/mod.rs:119` | `let mut table = write_tx.open_table(...)` | redb requires `&mut Table` for insert |
| `store/mod.rs:141` | `let mut table = write_tx.open_table(...)` | redb requires `&mut Table` for insert |
| `store/mod.rs:255` | `let mut table = write_tx.open_table(...)` | redb requires `&mut Table` for insert |

**Mitigating factor**: All 6 instances are structurally imposed by external crate APIs (sha2, redb). No internal state mutation exists. Functional core / imperative shell boundary is respected -- mutation occurs only at the I/O boundary.

**Severity**: LOW -- ecosystem-imposed, not design negligence.

### C4: ZERO_FOR_WHILE -- **FAIL**

1 `loop` in production code:

| File:Line | Pattern | Reason |
|-----------|---------|--------|
| `dedup.rs:50-65` | `loop { if slot.get() { ... } thread::yield_now() }` | `OnceLock` has no blocking wait API |

Spin-wait has a 30s deadline (`dedup.rs:49`) and only runs while the owner thread performs real I/O, making CPU overhead negligible.

**Severity**: LOW -- necessary for sync OnceLock pattern; has timeout guard.

### C5: ALL_ERRORS_EXPLICIT -- PASS
All fallible operations return `Result`. Cache write failures in `dedup.rs:161-167` are intentionally swallowed (documented as DEFECT-007 in code comments) because the compute result must still reach callers.

### C6: NON_EXHAUSTIVE_ENUMS -- PASS
All 14 public enums carry `#[non_exhaustive]`: CacheBackend, CacheConfig, CacheType, CacheStats, InFlightKey, InflightDecision, CacheError, ConfigError, EmbeddingError, DocumentError, IndexError, IoError, ValidationError, DocTransformerError.

### C7: CONTENT_ADDRESSED -- **FAIL**

Module docstring (`cache/mod.rs:11`) states "Cache keys are SHA-256 hashes of input." Implementation uses SHA-256 correctly. However, the public API accepts raw `&[u8]` keys with no compile-time guarantee:

```rust
pub fn put_document<V: Serialize>(&self, key: &[u8], value: &V) -> Result<()>
pub fn get_document<V: DeserializeOwned>(&self, key: &[u8]) -> Result<Option<V>>
```

A caller could pass arbitrary non-hash bytes. A `ContentHash([u8; 32])` newtype would make illegal states unrepresentable (DDD parse-don't-validate from AGENTS.md).

**Severity**: MEDIUM -- violates DDD parse-at-boundary doctrine. Relies on doc-level contract, not type-level enforcement.

### C8: SIZE_LIMITS_ENFORCED -- PASS
All 5 write paths validate key size via `validate_key_size()`:
- `put_document` (store/mod.rs:94), `put_scrape` (store/mod.rs:116), `put_transform` (store/mod.rs:138), `get_or_compute` (store/mod.rs:192), `put_raw` (store/mod.rs:247)

All value writes go through `validate_and_insert()` (hash.rs:64-72) which calls `validate_value_size()`. MAX_KEY_SIZE=256, MAX_VALUE_SIZE=10MB.

### C9: EMPTY_KEY_REJECTED -- PASS
`validate_key_size` (hash.rs:40) rejects `len == 0` with `CacheError::KeyTooLarge`. All write paths call this. ATTACK 5 test confirms rejection across all 3 cache types and `get_or_compute`.

### C10: CLIPPY_PEDANTIC -- PASS
`errors/mod.rs:4` declares `#![warn(clippy::pedantic)]`. Targeted `#[allow(...)]` suppressions are justified:
- `clippy::result_large_err` -- DocCache::open returns Result<Self>
- `clippy::too_many_arguments` -- finalize_compute has 7 params
- `clippy::large_enum_variant` -- DocTransformerError wraps diverse error types

### C11: THISERROR_FOR_ERRORS -- PASS
All 8 error enums derive `thiserror::Error` with descriptive messages including context fields.

### C12: CLONE_ON_ERRORS -- **FAIL**

7 of 8 error types derive `Clone, PartialEq, Eq`. **DocTransformerError** (errors/mod.rs:35-87) does NOT:

```rust
#[derive(Debug, Error)]  // Missing: Clone, PartialEq, Eq
pub enum DocTransformerError { ... }
```

This prevents equality assertions on the top-level error type and breaks the pattern established by all leaf error types.

**Severity**: MEDIUM -- trivially fixable, inconsistent with module convention.

### C13: IDEMPOTENT_OPERATIONS -- PASS
`put_*` methods use redb upsert semantics (idempotent). `get_or_compute` returns cached value if present. `clear_all` deletes and recreates tables (safe to call N times). `initialize_tables` uses `open_table` (create-if-not-exists).

### C14: TOCTOU_SAFE -- PASS
Previous DEFECT-004 is fixed. `dedup.rs:173-177` explicitly documents that in-flight entries are NOT removed after completion. DashMap's `entry()` provides atomic check-and-insert. Late waiters find the already-set OnceLock.

### C15: NO_MAGIC_STRINGS -- PASS
Typed enums: `CacheBackend` (Memory/File), `CacheType` (Document/Scrape/Transform). Table names are constants. No `":memory:"` string comparison -- the `CacheBackend::Memory` enum variant handles this.

### C16: CLIPPY_DENY_UNWRAP -- PASS
`errors/mod.rs:1-3` enforces:
```rust
#![deny(clippy::unwrap_used)]
#![deny(clippy::expect_used)]
#![deny(clippy::panic)]
```

### C17: TEST_COVERAGE -- PASS
16 adversarial tests (ATTACK 1-15) + 8 basic/dedup/limit tests covering:
- Extreme concurrency (100 threads, 50 waiters)
- Crypto collision probe (10k SHA-256 inputs)
- Disk corruption (byte-level .redb tampering)
- Empty/oversized keys and values
- Concurrent clear during read and compute
- Double-open, type mismatch, special bytes
- Error propagation races, all-tables-disabled path
- Stats accuracy (200+150+100 entries)

---

## Advisory Defects

### ADV-001: `eprintln!` in production code
**Location**: `dedup.rs:163`
**Issue**: `eprintln!("WARN: cache write failed...")` instead of structured logging.
**Risk**: No structured logging in production; output to stderr unconditionally.
**Fix**: Replace with `tracing::warn!` or `log::warn!`.

### ADV-002: `cache/` module lacks clippy deny directives
**Location**: `cache/mod.rs`
**Issue**: `errors/mod.rs` has `#![deny(clippy::unwrap_used)]` etc., but `cache/mod.rs` does not.
**Risk**: If crate-level lib.rs also lacks these, cache module has weaker compile-time guarantees.
**Fix**: Add deny directives to `cache/mod.rs` or verify they exist at crate root.

---

## Defect Summary

| ID | Constraint | Severity | File(s) | Description |
|----|-----------|----------|---------|-------------|
| D-01 | C3 ZERO_MUT | LOW | hash.rs, store/mod.rs | 6 `mut` bindings imposed by sha2/redb APIs |
| D-02 | C4 ZERO_FOR_WHILE | LOW | dedup.rs:50 | `loop` spin-wait for OnceLock (30s timeout) |
| D-03 | C7 CONTENT_ADDRESSED | MEDIUM | store/mod.rs (pub API) | Raw `&[u8]` keys lack type-safe hash wrapper |
| D-04 | C12 CLONE_ON_ERRORS | MEDIUM | errors/mod.rs:35 | DocTransformerError missing Clone, PartialEq, Eq |
| D-05 | ADV-001 | LOW | dedup.rs:163 | eprintln! in production |
| D-06 | ADV-002 | LOW | cache/mod.rs | No clippy deny directives at module level |

---

## Regression Check vs Round 12

All defects from Round 12 (D-01 through D-06) persist unchanged. No regressions introduced. No new defects found.

---

## Verdict

**STATUS: REJECTED**

4 hard constraints fail. Same defects as Round 12 -- no progress on fixes.

**Must fix:**
- D-03: Introduce `ContentHash([u8; 32])` newtype, parse at API boundary.
- D-04: Add `Clone, PartialEq, Eq` derives to `DocTransformerError`.

**Acceptable trade-offs:**
- D-01, D-02: Ecosystem-imposed (sha2, redb, std::sync::OnceLock). Fixing would require upstream changes or async runtime.
- D-05, D-06: Advisory, not blocking.

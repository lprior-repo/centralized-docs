# Black Hat Review — Round 9

**Auditor**: black-hat-reviewer
**Date**: 2026-03-21
**Scope**: ALL files in `src/cache/` and `src/errors/`

## 5-Phase Audit

### Phase 1: Structural — File line counts
| File | Lines | Status |
|------|-------|--------|
| cache/mod.rs | 28 | PASS |
| cache/config.rs | 95 | PASS |
| cache/hash.rs | 118 | PASS |
| cache/store/mod.rs | 264 | PASS |
| cache/store/dedup.rs | 194 | PASS |
| cache/tests/mod.rs | 6 | PASS |
| cache/tests/basic.rs | 145 | PASS |
| cache/tests/limits.rs | 131 | PASS |
| cache/tests/dedup.rs | 192 | PASS |
| cache/tests/adversarial.rs | 234 | PASS |
| cache/tests/adversarial_stress.rs | 114 | PASS |
| cache/tests/adversarial_edge.rs | 135 | PASS |
| errors/mod.rs | 140 | PASS |
| errors/transformer.rs | 89 | PASS |
| errors/config.rs | 24 | PASS |
| errors/validation.rs | 49 | PASS |
| errors/embedding.rs | 33 | PASS |
| errors/cache.rs | 20 | PASS |

All files under 300 lines.

### Phase 2: Constraint Verification (grep/read)

| # | Constraint | Method | Result |
|---|-----------|--------|--------|
| 1 | Zero Mutex in prod code | `grep Mutex` — only comment mentions in dedup.rs:4, store/mod.rs:38 | PASS |
| 2 | Zero channels in prod code | `grep channel/mpsc/Sender/Receiver` — only doc-comment mention in dedup.rs:5 | PASS |
| 3 | Zero unwrap/expect in prod code | `grep unwrap/expect` — all 14 matches in `tests/` (under `#[cfg(test)]`) | PASS |
| 4 | No in_flight.remove() | `grep in_flight\.remove` — zero matches | PASS |
| 5 | No spin_loop in cache | `grep spin_loop` — zero matches; only `yield_now` used | PASS |
| 6 | DashMap + OnceLock + catch_unwind | Confirmed: dedup.rs (DashMap import, OnceLock type, wait_once_lock), store/mod.rs:207 (catch_unwind) | PASS |
| 7 | 30s timeout on wait_once_lock | dedup.rs:49: `Duration::from_secs(30)` | PASS |
| 8 | SHA-256 hashing | hash.rs:98-106: `sha2::Sha256`; `grep DefaultHasher` — zero matches | PASS |
| 9 | Key 1..=256 bytes | hash.rs:40: `len == 0 \|\| len > MAX_KEY_SIZE` — rejects empty | PASS |
| 10 | Value max 10MB all paths | `put_cached_value_with_limit` → `validate_and_insert` → `validate_value_size`; `put_raw` calls `validate_value_size` at store/mod.rs:250 AND via `validate_and_insert` at store/mod.rs:259 | PASS |
| 11 | All files under 300 lines | max is store/mod.rs at 264 lines | PASS |
| 12 | #[non_exhaustive] on all public/pub(super) types | CacheBackend, CacheConfig, CacheType, CacheStats, DocCache, InFlightKey, InflightDecision, CacheError, ConfigError, ValidationError, DocumentError, IndexError, IoError, EmbeddingError, DocTransformerError — all annotated | PASS |
| 13 | #[cfg(unix)] on path_hash + re-export | hash.rs:114, mod.rs:25 | PASS |
| 14 | Single validate_and_insert | hash.rs:64-72; called by `put_cached_value_with_limit` (hash.rs:81) and `put_raw` (store/mod.rs:259) | PASS |
| 15 | eprintln! on cache write failure | dedup.rs:163 | PASS |
| 16 | Owner+waiter both get Err on serialization failure | dedup.rs:150-155: `slot.set(Err(...))` + `return Err(...)` — both paths receive Err | PASS |
| 17 | DocCache derives Clone via Arc<Database> | store/mod.rs:40-42: `#[derive(Debug, Clone)]`, `db: Arc<Database>` | PASS |

### Phase 3: Logic Audit (line-by-line)

**dedup.rs finalize_compute (lines 127-180):**
- Serialization failure path (lines 145-155): correctly sets slot with Err AND returns Err to owner. Waiters get Err via wait_once_lock. No divergence.
- Cache write failure path (lines 157-168): put_raw error swallowed (correct — compute result must not be discarded). eprintln! emitted. Slot still set with Ok(bytes).
- No remove() call (lines 173-177): explicit comment explaining why. Correct per DEFECT-004 fix.

**store/mod.rs put_raw (lines 248-263):**
- Validates key_size AND value_size before transaction, then validate_and_insert validates value_size again inside transaction. Redundant but correct.
- Doc comment at line 246 claims "Value size is NOT re-validated" — this is stale. Code DOES validate. Not a correctness issue (code is more conservative than the comment suggests).

**store/mod.rs get_or_compute (lines 187-233):**
- catch_unwind wraps compute closure. Panic payload downcasted to &str, String, or fallback message. Correct.
- Finalize returns Err propagated via `?`. Owner sees the error.
- Deduplication: DashMap entry → OnceLock → wait_once_lock → yield loop. Correct.

### Phase 4: Security Audit

- No unsafe code (errors/mod.rs:5: `#![forbid(unsafe_code)]`).
- No unbounded allocations (key max 256B, value max 10MB).
- No timing side-channels (constant-time comparison not needed — hashes are content-addressed, not secrets).
- No path traversal (paths used only for redb file creation with parent mkdir).
- catch_unwind prevents panic propagation across thread boundaries.

### Phase 5: Correctness Edge Cases

- Empty key rejected at validate_key_size. Tested in adversarial.rs rq_attack_5.
- Concurrent clear_all during compute: in_flight OnceLock survives redb clear. Tested in adversarial_stress.rs rq_attack_9.
- Disabled cache + get_or_compute: dedup still works, value not persisted. Tested in adversarial_edge.rs rq_attack_15.
- Serialization failure: both owner and waiter get Err. Tested in dedup.rs test_get_or_compute_propagates_error_to_waiters.

## INFO Findings (not defects)

- **INFO-001**: Stale doc comment at store/mod.rs:244-247. Claims "Value size is NOT re-validated here" but `put_raw` calls `validate_value_size` at line 250 AND via `validate_and_insert` at line 259. Code is correct (conservative); comment is misleading.
- **INFO-002**: Redundant value size validation in `put_raw` (validated twice: explicit call + via `validate_and_insert`). Harmless overhead.

## Verdict

```
STATUS: APPROVED
```

All 17 hard constraints verified. Zero correctness, security, or performance defects found.
Two INFO-level findings (stale comment, minor redundancy) — neither affects behavior.

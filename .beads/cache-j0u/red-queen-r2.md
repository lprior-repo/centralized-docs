# Red Queen Adversarial Report — Round 2

**Target:** `centralized-docs/src/cache/` (store.rs, hash.rs, config.rs, mod.rs) + `errors.rs`
**Date:** 2026-03-21
**Model:** glm-5-turbo
**Tests:** 15 adversarial attacks executed

## Results Summary

| # | Attack Vector | Test Name | Exit Code | Verdict |
|---|--------------|-----------|-----------|---------|
| 1 | 100 threads concurrent get_or_compute | rq_attack_1_extreme_concurrency_100_threads | 0 | **PASS** |
| 2 | SHA-256 collision probe (10k inputs) | rq_attack_2_sha256_no_collision_proximity | 0 | **PASS** |
| 3 | Disk file corruption (byte-flip .redb) | rq_attack_3_disk_corruption_graceful_error | 0 | **PASS** |
| 4 | InMemory backend — zero disk files | rq_attack_4_inmemory_no_disk_files | 0 | **PASS** |
| 5 | Empty inputs (key, value, hash) | rq_attack_5_empty_inputs | 0 | **PASS** |
| 6 | Maximum limits (256B key, 10MB value) | rq_attack_6_maximum_limits_boundary | 0 | **PASS** |
| 7 | Oversized inputs (257B key, 10MB+1 value) | rq_attack_7_oversized_inputs_rejected | 0 | **PASS** |
| 8 | Concurrent clear_all during read | rq_attack_8_concurrent_clear_during_read | 0 | **PASS** |
| 9 | Concurrent clear_all during compute | rq_attack_9_concurrent_clear_during_compute | 0 | **PASS** |
| 10 | Double open same file path | rq_attack_10_double_open_same_path | 0 | **PASS** |
| 11 | Error propagation race (50 waiters) | rq_attack_11_error_propagation_50_waiters | 0 | **PASS** |
| 12 | Special bytes in keys (null, UTF-8, non-UTF8) | rq_attack_12_special_bytes_in_keys | 0 | **PASS** |
| 13 | Type mismatch same key (String→i64) | rq_attack_13_type_mismatch_same_key | 0 | **PASS** |
| 14 | Stats accuracy (200+150+100 items) | rq_attack_14_stats_accuracy_large_n | 0 | **PASS** |
| 15 | All tables disabled, get_or_compute | rq_attack_15_all_tables_disabled_get_or_compute_still_works | 0 | **PASS** |

## Observations (Non-blocking)

### OBS-1: JSON serialization overhead on value size limit
- `MAX_VALUE_SIZE = 10MB` but serde_json adds 2 bytes for string quotes
- Actual max storable string is `MAX_VALUE_SIZE - 2` bytes
- Not a bug — the limit applies to **serialized** bytes as designed
- Documented behavior in test comment

### OBS-2: get_or_compute with all tables disabled recomputes every call
- When `cache_document_content = false`, `put_document` is a no-op
- `get_or_compute`'s fast path (`self.get()`) returns None, in-flight entry is cleaned up after each call
- Second call becomes owner and recomputes — expected, not a bug
- Users should not use `get_or_compute` with disabled cache types

### OBS-3: Disk corruption handled gracefully
- Corrupting bytes at offset 512 of .redb file
- Either open fails with descriptive error OR subsequent reads return error
- No panics, no undefined behavior

## Regression Verification

Full suite: 32/32 tests pass (17 original + 15 Red Queen)
```
cargo test -p centralized-docs --lib -- "cache"
test result: ok. 32 passed; 0 failed; 0 ignored
```

## Final Verdict

**ALL_ATTACKS_SURVIVED**

Zero defects found. The cache implementation withstands all 15 adversarial attack vectors:
- Concurrency: exact-once compute guaranteed across 100 threads
- Boundary: all size limits enforced correctly
- Corruption: graceful error handling on damaged files
- Race conditions: concurrent clear during read/compute causes no panics
- Error propagation: failures correctly reach all parked waiters
- Type safety: deserialization mismatches produce clear errors

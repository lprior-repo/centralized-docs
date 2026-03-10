# Implementation: index-locking: implement stale lock recovery metadata

## Problem Statement
The current `should_reclaim_stale_lock` function has a bug: when a lock file exists but is empty or contains malformed JSON, the function falls back to checking file age. If the file is new (just created by a crashed process), it won't be considered stale and will block new processes from acquiring the lock.

## Fix Applied

### File Changed
- `doc_transformer/src/main.rs`

### Change Details

**Before (buggy):**
```rust
fn should_reclaim_stale_lock(lock_path: &Path) -> bool {
    if let Some(metadata) = read_lock_metadata(lock_path) {
        let age_secs = now_unix_secs().saturating_sub(metadata.created_at_unix_secs);
        return !process_is_alive(metadata.pid, metadata.start_time)
            || age_secs > OUTPUT_LOCK_STALE_AFTER_SECS;
    }

    // BUG: If lock file is new but unreadable, it won't be reclaimed!
    lock_age_secs(lock_path).is_some_and(|age| age > OUTPUT_LOCK_STALE_AFTER_SECS)
}
```

**After (fixed):**
```rust
fn should_reclaim_stale_lock(lock_path: &Path) -> bool {
    if let Some(metadata) = read_lock_metadata(lock_path) {
        let age_secs = now_unix_secs().saturating_sub(metadata.created_at_unix_secs);
        return !process_is_alive(metadata.pid, metadata.start_time)
            || age_secs > OUTPUT_LOCK_STALE_AFTER_SECS;
    }

    // FIX: If we can't read the lock metadata (empty/malformed),
    // treat it as stale immediately - likely from crashed process
    true
}
```

### Test Added
- `test_should_reclaim_stale_lock_when_empty_file` - Verifies that empty lock files are treated as stale
- `test_should_reclaim_stale_lock_when_malformed_json` - Verifies that malformed JSON lock files are treated as stale

## Acceptance Criteria Verification

| Criterion | Status |
|-----------|--------|
| Active locks prevent concurrent writers | ✅ Existing test `test_should_not_reclaim_fresh_live_lock` |
| Stale locks are automatically reclaimed | ✅ Existing tests + new tests |
| Empty/malformed lock files treated as stale | ✅ New tests added |
| PID recycling detection works | ✅ Existing test `test_should_reclaim_stale_lock_when_pid_recycled` |
| Lock age threshold works | ✅ Existing test `test_should_reclaim_stale_lock_when_too_old` |

//! Lock-free in-flight deduplication for `get_or_compute`.
//!
//! Uses `DashMap` + `std::sync::OnceLock` to guarantee that concurrent calls
//! with the same key invoke the compute closure exactly once. No `Mutex` or
//! channels are used. The algorithm is:
//!
//! 1. **Fast path**: check redb cache — return immediately if found.
//! 2. **Owner** (first thread to miss cache): inserts an `Arc<OnceLock<...>>`
//!    into the `DashMap`, runs `compute()`, stores the result in both the
//!    `OnceLock` and redb.
//! 3. **Waiters** (subsequent threads): find the `OnceLock` in the `DashMap`,
//!    clone the `Arc`, release the `DashMap` shard lock, then yield via
//!    `thread::yield_now()` with negligible CPU overhead since the owner is
//!    performing real I/O. Once the owner publishes, all waiters observe the
//!    result and return.
//!
//! In-flight entries are NOT removed after completion. This avoids a TOCTOU
//! race where a late waiter could miss both the `DashMap` entry and the redb
//! cache. Waiters that find an already-set `OnceLock` still get the value.
//! In-flight entries accumulate but are bounded by the number of concurrent
//! keys. `clear_all()` handles bulk cleanup via `self.in_flight.clear()`.

use std::sync::Arc;
use std::time::{Duration, Instant};

use anyhow::Result;
use dashmap::DashMap;
use serde::{de::DeserializeOwned, Serialize};

use super::super::config::CacheType;
use super::super::hash::content_hash;

const INFLIGHT_SPIN_TIMEOUT_SECS: u64 = 120;
const MAX_INFLIGHT_ENTRIES: usize = 10_000;

/// Fixed-size key for in-flight computation tracking.
#[non_exhaustive]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub(super) struct InFlightKey {
    pub cache_type: CacheType,
    pub key_hash: u128,
}

/// Result of an in-flight computation, shared between owner and waiters.
pub(super) type ComputeSlot = Arc<std::sync::OnceLock<Result<Vec<u8>>>>;

/// Block until the `OnceLock` is set, then return a clone of the inner value.
///
/// Yield-based spin loop with negligible CPU overhead since the owner is
/// performing real I/O (network, disk) that dwarfs the polling cost.
pub(super) fn wait_once_lock(slot: &ComputeSlot) -> Result<Vec<u8>> {
    let deadline = Instant::now() + Duration::from_secs(INFLIGHT_SPIN_TIMEOUT_SECS);
    loop {
        if let Some(result) = slot.get() {
            return match result.as_ref() {
                Ok(bytes) => Ok(bytes.clone()),
                Err(e) => Err(anyhow::anyhow!("{e:#}")),
            };
        }
        if Instant::now() > deadline {
            return Err(crate::errors::CacheError::BackendError {
                operation: "wait_once_lock",
                message: format!(
                    "timed out waiting for in-flight computation after {INFLIGHT_SPIN_TIMEOUT_SECS}s"
                ),
            }
            .into());
        }
        std::thread::yield_now();
    }
}

/// Core deduplication logic for `get_or_compute`.
///
/// Returns `Ok(Some(value))` if the value was already cached (fast path).
/// Returns `Ok(None)` if no cached value was found — the caller should then
/// proceed as the owner and run the compute closure.
/// Returns `Err` if a waiter path resolved with an error from another thread.
pub(super) fn check_cache_and_inflight<V>(
    in_flight: &DashMap<InFlightKey, ComputeSlot>,
    cache_type: CacheType,
    key: &[u8],
    get_cached: impl FnOnce(CacheType, &[u8]) -> Result<Option<V>>,
) -> Result<InflightDecision<V>>
where
    V: Serialize + DeserializeOwned,
{
    // Step 1: fast path — check redb cache
    if let Some(cached) = get_cached(cache_type, key)? {
        return Ok(InflightDecision::Cached(cached));
    }

    let key_hash = content_hash(key);
    let in_flight_key = InFlightKey {
        cache_type,
        key_hash,
    };

    // Step 2: check/create in-flight slot (lock-free DashMap)
    match in_flight.entry(in_flight_key) {
        // Owner: first thread to see this key
        dashmap::mapref::entry::Entry::Vacant(entry) => {
            let slot = Arc::new(std::sync::OnceLock::new());
            entry.insert(Arc::clone(&slot));
            Ok(InflightDecision::Owner {
                in_flight_key,
                slot,
            })
        }
        // Waiter: another thread is already computing
        dashmap::mapref::entry::Entry::Occupied(entry) => {
            let slot = entry.get().clone();
            drop(entry);
            let bytes = wait_once_lock(&slot)?;
            let value: V = match bincode::deserialize(&bytes) {
                Ok(v) => v,
                Err(e) => return Err(anyhow::anyhow!("in-flight deserialize: {e}")),
            };
            Ok(InflightDecision::WaiterResult(value))
        }
    }
}

/// Publishes the compute result, stores it in the cache, and publishes to waiters.
///
/// # Cache write failures (DEFECT-007)
///
/// If `put_raw` fails after a successful compute, the value is still returned to
/// callers (owner and waiters via the `OnceLock`). The cache write failure is
/// silently tolerated because:
/// - The compute result is correct and should not be discarded.
/// - Waiters are already depending on receiving the value.
/// - Production should wire up `log::warn!` here; tests use `eprintln!` if needed.
#[allow(clippy::too_many_arguments)]
pub(super) fn finalize_compute<V>(
    _in_flight: &DashMap<InFlightKey, ComputeSlot>,
    _in_flight_key: InFlightKey,
    slot: &ComputeSlot,
    compute_result: &Result<V>,
    put_raw: impl FnOnce(CacheType, &[u8], &[u8]) -> Result<()>,
    cache_type: CacheType,
    key: &[u8],
) -> Result<()>
where
    V: Serialize + DeserializeOwned,
{
    // Step 4: serialise result to bytes
    let slot_result: Result<Vec<u8>> = match compute_result {
        Ok(value) => bincode::serialize(value).map_err(Into::into),
        Err(e) => Err(anyhow::anyhow!("{e:#}")),
    };

    // DEFECT-002: If compute succeeded but serialization failed, propagate the
    // error to the owner too — otherwise the owner silently succeeds while all
    // waiters get the serialization error, creating result divergence.
    // Also set the slot so waiters receive the same error instead of spinning
    // until timeout.
    if let (Ok(_), Err(ref e)) = (compute_result, &slot_result) {
        let _ = slot.set(Err(anyhow::anyhow!("{e:#}")));
        return Err(anyhow::anyhow!(
            "compute succeeded but serialization failed: {e:#}"
        ));
    }

    // Step 5: store in redb (best-effort — put failure must not swallow compute result)
    // NOTE: Cache write failures are intentionally swallowed here. The slot is set
    // with Ok(bytes) regardless, so all waiters receive the computed value. In
    // production, add `log::warn!("cache write failed for key: {:?}", key);` here.
    if let Ok(ref bytes) = slot_result {
        if put_raw(cache_type, key, bytes).is_err() {
            eprintln!(
                "WARN: cache write failed for {cache_type:?} key_len={}, value not persisted — will recompute on next call",
                key.len()
            );
        }
    }

    // Step 6: publish to OnceLock (visible to all yielding waiters)
    let _ = slot.set(slot_result);

    // DEFECT-004 FIX: Do NOT remove the in_flight entry. A late waiter could
    // arrive between remove() and the redb write, missing both the DashMap
    // entry and the cache. Leaving the entry is safe: the OnceLock is already
    // set, so any future waiter gets the value immediately.
    // DEFECT-003 FIX: prune when the map exceeds MAX_INFLIGHT_ENTRIES to
    // prevent unbounded memory growth. Only entries whose OnceLock is already
    // set (completed) are candidates for removal.
    prune_in_flight(_in_flight);

    Ok(())
}

fn prune_in_flight(in_flight: &DashMap<InFlightKey, ComputeSlot>) {
    if in_flight.len() <= MAX_INFLIGHT_ENTRIES {
        return;
    }
    let excess = in_flight.len() - MAX_INFLIGHT_ENTRIES;
    let removed = std::cell::Cell::new(0usize);
    in_flight.retain(|_, slot| {
        if removed.get() >= excess {
            return true;
        }
        match slot.get() {
            Some(Ok(_)) => {
                removed.set(removed.get().saturating_add(1));
                false
            }
            _ => true,
        }
    });
}

/// Result of checking the cache and in-flight map.
#[non_exhaustive]
pub(super) enum InflightDecision<V> {
    /// Value was already cached — return immediately.
    Cached(V),
    /// This thread is the owner — it should compute and finalize.
    Owner {
        in_flight_key: InFlightKey,
        slot: ComputeSlot,
    },
    /// Another thread computed and we received the result.
    WaiterResult(V),
}

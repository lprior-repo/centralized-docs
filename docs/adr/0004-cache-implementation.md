# ADR-0004: Cache Implementation — redb-backed Idempotent Document Processing

## Status
Accepted

## Context
The `centralized-docs` crate processes documents through a multi-stage pipeline: discovery, scraping, chunking, embedding, and transformation. Without caching, every invocation re-processes every document from scratch, causing:

- **Wasted compute**: Embedding and LLM calls are expensive and rate-limited
- **Wasted I/O**: Re-scraping URLs that haven't changed
- **Wasted time**: Full pipeline runs take minutes even when nothing changed
- **Non-idempotent restarts**: Interrupted runs must start over

## Decision
We implemented a persistent, ACID-compliant cache layer using redb with exact-once computation deduplication.

## Features

### 1. Three Cache Tables
- `documents` — SHA-256 content hash → document metadata (skip reprocessing unchanged files)
- `scrape` — URL hash → scraped content (avoid re-fetching)
- `transforms` — key hash → transformed output (idempotent pipeline runs)

### 2. Content-Addressed Keys
All cache keys are SHA-256 hashes of the input content. This provides:
- Automatic deduplication: identical content produces identical keys
- Deterministic behavior: same input always produces same cache lookup
- No key management: keys are derived, not assigned

### 3. Exact-Once Computation Deduplication
When multiple threads call `get_or_compute` with the same key simultaneously:
- First thread (owner) registers in a `DashMap`, runs compute, stores result
- Subsequent threads (waiters) find the `DashMap` entry, park via `OnceLock`, receive the result
- Implementation: `DashMap` (lock-free concurrent map) + `OnceLock` (parking primitive)
- No `Mutex` or channels used anywhere

### 4. Panic Safety
`compute()` closures are wrapped in `std::panic::catch_unwind`. If a compute closure panics:
- The panic is caught and converted to an error
- Waiters are notified via the `OnceLock` (they get the error, not a deadlock)
- No thread spins forever

### 5. Size Limits
- Keys: 1–256 bytes (rejects empty keys)
- Values: max 10 MB (prevents memory exhaustion)
- Validated in ALL write paths (no bypass)

### 6. Dual Backends
- `CacheBackend::File(path)` — persistent `.redb` file
- `CacheBackend::Memory` — in-memory, lost on process exit (for tests)

### 7. Idempotent Setup
`DocCache::open(config)` is idempotent:
- Tables are created with `IF NOT EXISTS` semantics (redb's `open_table`)
- Can be called multiple times safely
- `clear_all()` atomically deletes and recreates tables in one transaction

## Architecture

```
┌─────────────┐     ┌──────────────┐     ┌─────────────┐
│  DocCache    │────▶│  DashMap     │────▶│  OnceLock   │
│  (public     │     │  in_flight   │     │  (parking)  │
│   API)       │     │  <key, slot> │     │  <result>   │
└──────┬───────┘     └──────────────┘     └─────────────┘
       │                                          │
       ▼                                          ▼
┌─────────────┐                           ┌─────────────┐
│  redb       │                           │  Waiters    │
│  (persist)  │                           │  (recv)     │
└─────────────┘                           └─────────────┘
```

### File Structure
- `cache/mod.rs` — Module declarations and re-exports
- `cache/config.rs` — CacheBackend, CacheConfig, CacheType, CacheStats
- `cache/hash.rs` — SHA-256 hashing, key/value validation, table helpers
- `cache/store/mod.rs` — DocCache struct, open, get/put, clear, stats
- `cache/store/dedup.rs` — In-flight deduplication logic
- `cache/tests/` — Unit tests, adversarial tests, dedup tests

## Design Decisions

### Why DashMap + OnceLock instead of Mutex?
- `Mutex` serializes ALL operations, creating a bottleneck
- `DashMap` uses sharded locking — different keys don't contend
- `OnceLock` provides thread parking with zero CPU overhead
- Result: concurrent lookups for different keys are fully parallel

### Why not remove in_flight entries?
- Removing entries creates a TOCTOU race: a late waiter could miss both the DashMap entry and the redb cache
- Keeping entries is bounded by the number of unique concurrent keys (~40 bytes per entry)
- `clear_all()` handles bulk cleanup

### Why catch_unwind on compute?
- Without it, a panicked compute closure leaves waiters spinning forever
- `OnceLock` is never set, causing a deadlock
- catch_unwind converts the panic to an error, notifies all waiters

### Why best-effort cache writes?
- If compute succeeds but redb write fails (disk full, corruption), we still return the value
- The compute result is correct — discarding it wastes resources
- Future calls will recompute (acceptable degradation)
- `eprintln!` logs the failure for observability

## Testing
- 32 tests total (15 unit, 17 adversarial)
- Adversarial tests cover: 100-thread concurrency, SHA-256 collision resistance, disk corruption, in-memory isolation, empty inputs, double-open, concurrent clear during read/compute, special bytes, type mismatch, stats accuracy, disabled-cache behavior, boundary limits
- 9 rounds of Black Hat code review (R1-R9)
- Truth Serum adversarial audit (10/10 checks passed)

## Consequences
- Positive: Idempotent restarts, exact-once compute, panic-safe, thread-safe
- Trade-off: Single-writer concurrency (redb serializes writes)
- Trade-off: In-flight entries accumulate until `clear_all()` (bounded, acceptable)
- Trade-off: No TTL/expiration (values persist until explicit `clear_all()`)

## References
- Bead: `cache-j0u`
- Contract: `.beads/cache-j0u/contract.md`
- Black Hat reviews: `.beads/cache-j0u/black-hat-r*.md`
- Truth Serum report: `.beads/cache-j0u/truth-serum-report.md`
- ADR-0003 (redb selection): `docs/adr/0003-cache-backend-redb.md`

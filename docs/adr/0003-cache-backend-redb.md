# ADR-0003: Cache Backend — redb

## Status
Accepted

## Context
The `centralized-docs` crate needs a high-performance ACID key-value cache for:
- Document content hashes (skip reprocessing unchanged files)
- Scraped URLs (avoid re-fetching)
- Transform results (idempotent pipeline runs)

Requirements:
- ACID transactions with crash recovery
- Single-file embedded database (no external server)
- Content-addressed storage (SHA-256 keys)
- Thread-safe concurrent access via MVCC
- Pure Rust, zero unsafe code in our layer
- Minimal dependency footprint

## Decision
We use [redb](https://github.com/cberner/redb) as the cache backend.

## Evaluation

### Why redb

| Property | redb |
|----------|------|
| ACID Transactions | Yes — copy-on-write B-tree |
| MVCC Reads | Yes — concurrent readers without blocking |
| Crash Recovery | Yes — write-ahead logging |
| Unsafe Code | None in redb itself |
| Binary Size | ~100KB |
| Dependencies | Minimal |
| API Simplicity | Very simple mental model |
| In-Memory Mode | Yes — `InMemoryBackend` |
| Single-File Storage | Yes — `.redb` file |

### Why not alternatives

**SQLite:** Heavier dependency, C FFI boundary, overkill for key-value caching.

**RocksDB:** C++ dependency, complex build, requires FFI.

**HashMap/BTreeMap:** No persistence, no crash recovery, no ACID.

## Consequences
- Single-writer concurrency: all writes are serialized by redb's transaction system
- Read concurrency: unlimited via MVCC
- Cache writes use `begin_write()` / `commit()` transactions
- Three cache tables: `documents`, `scrape`, `transforms`
- Exact-once computation deduplication via `DashMap` + `OnceLock` (application layer)
- Cache stored at `.cache/ctd_cache.redb` by default, configurable via `CacheConfig`

## References
- redb repository: https://github.com/cberner/redb
- Cache module: `centralized-docs/src/cache/`

//! Idempotent cache layer using redb for high-performance ACID storage
//!
//! Provides caching for:
//! - Document content hashes (skip reprocessing unchanged files)
//! - Scraped URLs (avoid re-fetching)
//! - Transform results (idempotent pipeline runs)
//!
//! # Design Principles
//!
//! - **Idempotency**: Same input always produces same cached result
//! - **Content-addressed**: Cache keys are xxh3-128 hashes of input
//! - **ACID guarantees**: redb provides transactional safety
//! - **Zero-panic**: All operations return Result, no unwrap/expect
//! - **Size limits**: Keys and values are bounded to prevent `DoS` attacks

pub mod config;
pub mod hash;
pub mod store;

#[cfg(test)]
mod tests;

// Re-exports for stable public API
pub use config::{CacheBackend, CacheConfig, CacheStats, CacheType};
#[cfg(unix)]
pub use hash::path_hash;
pub use hash::{content_hash, url_hash};
pub use store::DocCache;

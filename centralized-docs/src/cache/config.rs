//! Cache configuration types and constants.
//!
//! Contains `CacheBackend`, `CacheConfig`, `CacheType`, `CacheStats`,
//! size-limit constants, and redb table definitions.

use redb::TableDefinition;
use std::path::Path;

/// Maximum allowed key size in bytes (256 bytes).
/// Prevents memory exhaustion from oversized keys.
pub(crate) const MAX_KEY_SIZE: usize = 256;

/// Maximum allowed value size in bytes (10 MB).
/// Prevents memory exhaustion from oversized cached values.
pub(crate) const MAX_VALUE_SIZE: usize = 10 * 1024 * 1024;

pub(crate) const DOCUMENT_CACHE_TABLE: TableDefinition<&[u8], &[u8]> =
    TableDefinition::new("documents");
pub(crate) const SCRAPE_CACHE_TABLE: TableDefinition<&[u8], &[u8]> = TableDefinition::new("scrape");
pub(crate) const TRANSFORM_CACHE_TABLE: TableDefinition<&[u8], &[u8]> =
    TableDefinition::new("transforms");
pub(crate) const METADATA_TABLE: TableDefinition<&str, &str> = TableDefinition::new("metadata");

/// Cache backend selection - eliminates magic strings for type safety.
#[non_exhaustive]
#[derive(Debug, Clone)]
pub enum CacheBackend {
    /// In-memory cache using redb's `InMemoryBackend`.
    /// Data is lost when the process exits.
    Memory,
    /// Persistent file-based cache stored at the given path.
    File(std::path::PathBuf),
}

#[non_exhaustive]
#[derive(Debug, Clone)]
pub struct CacheConfig {
    pub backend: CacheBackend,
    pub cache_document_content: bool,
    pub cache_scrape_results: bool,
    pub cache_transforms: bool,
}

impl CacheConfig {
    #[must_use]
    pub fn new(db_path: &Path) -> Self {
        Self {
            backend: CacheBackend::File(db_path.to_path_buf()),
            cache_document_content: true,
            cache_scrape_results: true,
            cache_transforms: true,
        }
    }

    #[must_use]
    pub fn in_memory() -> Self {
        Self {
            backend: CacheBackend::Memory,
            cache_document_content: true,
            cache_scrape_results: true,
            cache_transforms: true,
        }
    }
}

impl Default for CacheConfig {
    fn default() -> Self {
        Self {
            backend: CacheBackend::File(std::path::PathBuf::from(".cache/ctd_cache.redb")),
            cache_document_content: true,
            cache_scrape_results: true,
            cache_transforms: true,
        }
    }
}

/// Cache type selector for `get_or_compute` operations.
///
/// Marked `#[non_exhaustive]` to allow adding new cache types in future
/// versions without breaking changes.
#[non_exhaustive]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum CacheType {
    Document,
    Scrape,
    Transform,
}

#[non_exhaustive]
#[derive(Debug, Clone)]
pub struct CacheStats {
    pub document_entries: u64,
    pub scrape_entries: u64,
    pub transform_entries: u64,
}

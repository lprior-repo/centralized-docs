//! Cache configuration types.

use super::types::CacheType;

// ---------------------------------------------------------------------------
// EnabledTypes (bitfield)
// ---------------------------------------------------------------------------

/// Which cache types are enabled. Defaults to all.
#[derive(Debug, Clone, Copy)]
pub struct EnabledTypes(u8);

impl EnabledTypes {
    pub(crate) fn all() -> Self {
        Self(0xFF)
    }

    pub(crate) fn is_enabled(self, cache_type: CacheType) -> bool {
        self.0 & (1 << cache_type as u8) != 0
    }
}

// ---------------------------------------------------------------------------
// Cache backend selection
// ---------------------------------------------------------------------------

/// Cache backend selection.
#[non_exhaustive]
#[derive(Debug, Clone)]
pub enum CacheBackend {
    /// In-memory cache using redb's `InMemoryBackend`.
    Memory,
    /// Persistent file-based cache.
    File(std::path::PathBuf),
}

// ---------------------------------------------------------------------------
// CacheConfig
// ---------------------------------------------------------------------------

/// Cache configuration.
///
/// Construct via `CacheConfig::new(path)` or `CacheConfig::in_memory()`.
/// Disable specific types with `config.disable(CacheType::Document)`.
#[non_exhaustive]
#[derive(Debug, Clone)]
pub struct CacheConfig {
    pub(crate) backend: CacheBackend,
    pub(crate) enabled: EnabledTypes,
}

impl CacheConfig {
    /// Create a file-backed cache with all types enabled.
    #[must_use]
    pub fn new(db_path: &std::path::Path) -> Self {
        Self {
            backend: CacheBackend::File(db_path.to_path_buf()),
            enabled: EnabledTypes::all(),
        }
    }

    /// Create an in-memory cache with all types enabled.
    #[must_use]
    pub fn in_memory() -> Self {
        Self {
            backend: CacheBackend::Memory,
            enabled: EnabledTypes::all(),
        }
    }

    /// Disable a specific cache type. Returns `&mut Self` for chaining.
    pub fn disable(&mut self, cache_type: CacheType) -> &mut Self {
        self.enabled.0 &= !(1 << cache_type as u8);
        self
    }

    /// Enable a specific cache type. Returns `&mut Self` for chaining.
    pub fn enable(&mut self, cache_type: CacheType) -> &mut Self {
        self.enabled.0 |= 1 << cache_type as u8;
        self
    }
}

impl Default for CacheConfig {
    fn default() -> Self {
        Self {
            backend: CacheBackend::File(std::path::PathBuf::from(".cache/ctd_cache.redb")),
            enabled: EnabledTypes::all(),
        }
    }
}

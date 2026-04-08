use std::time::Duration;

use super::errors::FeatureError;

/// Positive duration in seconds (validated at construction)
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct CacheTtl(u64);

impl CacheTtl {
    /// Creates a new cache TTL, ensuring it's positive
    ///
    /// # Errors
    ///
    /// Returns `FeatureError::InvalidCacheTtl` if `seconds` is zero.
    #[allow(dead_code)]
    pub fn new(seconds: u64) -> Result<Self, FeatureError> {
        if seconds > 0 {
            Ok(Self(seconds))
        } else {
            Err(FeatureError::InvalidCacheTtl(seconds))
        }
    }

    #[must_use]
    #[allow(dead_code)]
    pub fn as_duration(&self) -> Duration {
        Duration::from_secs(self.0)
    }

    #[must_use]
    #[allow(dead_code)]
    pub fn seconds(&self) -> u64 {
        self.0
    }
}

impl Default for CacheTtl {
    fn default() -> Self {
        // Default 5 minutes
        Self(300)
    }
}

/// Cache configuration
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct CacheConfig {
    pub enabled: bool,
    pub ttl: CacheTtl,
}

impl CacheConfig {
    #[must_use]
    pub fn disabled() -> Self {
        Self {
            enabled: false,
            ttl: CacheTtl::default(),
        }
    }

    #[must_use]
    pub fn enabled_with_ttl(ttl: CacheTtl) -> Self {
        Self { enabled: true, ttl }
    }
}

impl Default for CacheConfig {
    fn default() -> Self {
        Self::disabled()
    }
}

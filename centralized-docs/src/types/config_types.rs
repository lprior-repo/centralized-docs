//! Validated configuration parameter types for HNSW and graph settings.

use serde::{Deserialize, Serialize};
use std::cmp::PartialOrd;
use std::fmt;

/// Configuration validation errors.
#[derive(Debug, Clone, thiserror::Error)]
pub enum ConfigError {
    #[error("max_related_chunks must be at least 1, got {0}")]
    MaxRelatedChunksTooSmall(usize),
    #[error("max_related_chunks must be at most 1000, got {0}")]
    MaxRelatedChunksTooLarge(usize),
    #[error("hnsw_m must be at least 4, got {0}")]
    HnswMTooSmall(usize),
    #[error("hnsw_m must be at most 64, got {0}")]
    HnswMTooLarge(usize),
    #[error("hnsw_ef_construction must be at least 50, got {0}")]
    HnswEfConstructionTooSmall(usize),
    #[error("hnsw_ef_construction must be at most 1000, got {0}")]
    HnswEfConstructionTooLarge(usize),
}

/// A validated wrapper for `max_related_chunks` configuration parameter (1-1000).
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize)]
pub struct MaxRelatedChunks(u16); // u16 is sufficient for max 1000

impl MaxRelatedChunks {
    /// Create a new `MaxRelatedChunks`, validating the range (1-1000).
    pub fn new(value: usize) -> Result<Self, ConfigError> {
        if value < 1 {
            return Err(ConfigError::MaxRelatedChunksTooSmall(value));
        }
        if value > 1000 {
            return Err(ConfigError::MaxRelatedChunksTooLarge(value));
        }
        // Safe: value is guaranteed to be <= 1000 by the validation above
        #[allow(clippy::cast_possible_truncation)]
        Ok(MaxRelatedChunks(value as u16))
    }

    /// Get the underlying value.
    #[must_use]
    pub fn get(&self) -> usize {
        self.0 as usize
    }
}

// Comparison with usize for test ergonomics
impl PartialEq<usize> for MaxRelatedChunks {
    fn eq(&self, other: &usize) -> bool {
        self.get() == *other
    }
}

impl PartialEq<MaxRelatedChunks> for usize {
    fn eq(&self, other: &MaxRelatedChunks) -> bool {
        *self == other.get()
    }
}

impl PartialOrd<usize> for MaxRelatedChunks {
    fn partial_cmp(&self, other: &usize) -> Option<std::cmp::Ordering> {
        self.get().partial_cmp(other)
    }
}

impl PartialOrd<MaxRelatedChunks> for usize {
    fn partial_cmp(&self, other: &MaxRelatedChunks) -> Option<std::cmp::Ordering> {
        self.partial_cmp(&other.get())
    }
}

impl fmt::Display for MaxRelatedChunks {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl Default for MaxRelatedChunks {
    fn default() -> Self {
        MaxRelatedChunks(20) // Default value
    }
}

/// A validated wrapper for HNSW M parameter (4-64).
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize)]
pub struct HnswM(u8); // u8 is sufficient for max 64

impl HnswM {
    /// Create a new `HnswM`, validating the range (4-64).
    pub fn new(value: usize) -> Result<Self, ConfigError> {
        if value < 4 {
            return Err(ConfigError::HnswMTooSmall(value));
        }
        if value > 64 {
            return Err(ConfigError::HnswMTooLarge(value));
        }
        // Convert to u8 since we validated the range
        Ok(HnswM(value as u8))
    }

    /// Get the underlying value.
    #[must_use]
    pub fn get(&self) -> usize {
        self.0 as usize
    }
}

// Comparison with usize for test ergonomics
impl PartialEq<usize> for HnswM {
    fn eq(&self, other: &usize) -> bool {
        self.get() == *other
    }
}

impl PartialEq<HnswM> for usize {
    fn eq(&self, other: &HnswM) -> bool {
        *self == other.get()
    }
}

impl PartialOrd<usize> for HnswM {
    fn partial_cmp(&self, other: &usize) -> Option<std::cmp::Ordering> {
        self.get().partial_cmp(other)
    }
}

impl PartialOrd<HnswM> for usize {
    fn partial_cmp(&self, other: &HnswM) -> Option<std::cmp::Ordering> {
        self.partial_cmp(&other.get())
    }
}

impl fmt::Display for HnswM {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl Default for HnswM {
    fn default() -> Self {
        HnswM(16) // Default value
    }
}

/// A validated wrapper for HNSW `ef_construction` parameter (50-1000).
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize)]
pub struct HnswEfConstruction(u16); // u16 is sufficient for max 1000

impl HnswEfConstruction {
    /// Create a new `HnswEfConstruction`, validating the range (50-1000).
    pub fn new(value: usize) -> Result<Self, ConfigError> {
        if value < 50 {
            return Err(ConfigError::HnswEfConstructionTooSmall(value));
        }
        if value > 1000 {
            return Err(ConfigError::HnswEfConstructionTooLarge(value));
        }
        // Convert to u16 since we validated the range
        Ok(HnswEfConstruction(value as u16))
    }

    /// Get the underlying value.
    #[must_use]
    pub fn get(&self) -> usize {
        self.0 as usize
    }
}

// Comparison with usize for test ergonomics
impl PartialEq<usize> for HnswEfConstruction {
    fn eq(&self, other: &usize) -> bool {
        self.get() == *other
    }
}

impl PartialEq<HnswEfConstruction> for usize {
    fn eq(&self, other: &HnswEfConstruction) -> bool {
        *self == other.get()
    }
}

impl PartialOrd<usize> for HnswEfConstruction {
    fn partial_cmp(&self, other: &usize) -> Option<std::cmp::Ordering> {
        self.get().partial_cmp(other)
    }
}

impl PartialOrd<HnswEfConstruction> for usize {
    fn partial_cmp(&self, other: &HnswEfConstruction) -> Option<std::cmp::Ordering> {
        self.partial_cmp(&other.get())
    }
}

impl fmt::Display for HnswEfConstruction {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl Default for HnswEfConstruction {
    fn default() -> Self {
        HnswEfConstruction(200) // Default value
    }
}

//! Graph configuration with validated HNSW parameters.

use crate::types::{HnswEfConstruction, HnswM, MaxRelatedChunks};
use anyhow::Result;
use serde::{Deserialize, Serialize};
use std::fs;
use std::path::Path;

/// Configuration for knowledge graph construction parameters
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GraphConfig {
    /// Maximum number of related chunks to link per document (1-1000)
    pub max_related_chunks: MaxRelatedChunks,
    /// HNSW graph connectivity parameter (4-64)
    pub hnsw_m: HnswM,
    /// HNSW graph construction effort level (50-1000)
    pub hnsw_ef_construction: HnswEfConstruction,
}

#[allow(dead_code)] // Public API - exported for library users, not used internally
impl GraphConfig {
    /// Create `GraphConfig` with default values
    #[must_use]
    pub fn new() -> Self {
        Self {
            max_related_chunks: MaxRelatedChunks::default(),
            hnsw_m: HnswM::default(),
            hnsw_ef_construction: HnswEfConstruction::default(),
        }
    }

    /// Load configuration from a YAML file with validation
    ///
    /// # Errors
    ///
    /// Returns an error if:
    /// - The file cannot be read
    /// - The YAML is invalid
    /// - Parameter validation fails (out of range)
    pub fn load_from_file(path: &Path) -> Result<Self> {
        #[derive(Debug, Clone, Serialize, Deserialize)]
        struct GraphConfigRaw {
            max_related_chunks: usize,
            hnsw_m: usize,
            hnsw_ef_construction: usize,
        }

        let content = fs::read_to_string(path)?;
        let raw_config: GraphConfigRaw = serde_yaml::from_str(&content)?;

        Ok(Self {
            max_related_chunks: MaxRelatedChunks::new(raw_config.max_related_chunks)?,
            hnsw_m: HnswM::new(raw_config.hnsw_m)?,
            hnsw_ef_construction: HnswEfConstruction::new(raw_config.hnsw_ef_construction)?,
        })
    }

    /// Create config with custom values (validates them)
    ///
    /// # Errors
    ///
    /// Returns an error if:
    /// - `max_related_chunks` is not in range [1, 1000]
    /// - `hnsw_m` is not in range [4, 64]
    /// - `hnsw_ef_construction` is not in range [50, 1000]
    pub fn with_params(
        max_related_chunks: usize,
        hnsw_m: usize,
        hnsw_ef_construction: usize,
    ) -> Result<Self> {
        Ok(Self {
            max_related_chunks: MaxRelatedChunks::new(max_related_chunks)?,
            hnsw_m: HnswM::new(hnsw_m)?,
            hnsw_ef_construction: HnswEfConstruction::new(hnsw_ef_construction)?,
        })
    }

    /// Get `max_related_chunks` as usize
    #[must_use]
    pub fn get_max_related_chunks(&self) -> usize {
        self.max_related_chunks.get()
    }

    /// Get `hnsw_m` as usize
    #[must_use]
    pub fn get_hnsw_m(&self) -> usize {
        self.hnsw_m.get()
    }

    /// Get `hnsw_ef_construction` as usize
    #[must_use]
    pub fn get_hnsw_ef_construction(&self) -> usize {
        self.hnsw_ef_construction.get()
    }
}

impl Default for GraphConfig {
    fn default() -> Self {
        Self::new()
    }
}

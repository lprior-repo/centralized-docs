//! Domain types and error taxonomy for the scrape-reuse pipeline.

use crate::state::bulk_load::BulkLoadError;
use crate::state::{StateLoadError, UrlStateRaw};

/// Partition of scraped page indices into unchanged vs changed-or-new.
#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub struct ScrapePageDiff {
    /// Indices into the original pages vec for unchanged pages.
    pub unchanged: Vec<usize>,
    /// Indices for changed or new pages.
    pub changed_or_new: Vec<usize>,
}

/// Statistics about scrape reuse within a single command invocation.
#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub struct ScrapeReuseStats {
    /// Number of pages loaded from archived state (zero-cost reuse).
    pub reused: usize,
    /// Number of pages freshly scraped and processed through the pipeline.
    pub scraped: usize,
}

/// Error type for the scrape-reuse pipeline.
#[derive(Debug, thiserror::Error)]
pub enum ScrapeReuseError {
    /// Failed to load URL states from the state database.
    #[error("failed to load url states: {0}")]
    StateLoad(#[from] StateLoadError),

    /// Failed to load archived scrape outputs from the state database.
    #[error("failed to load archived scrape outputs: {0}")]
    BulkLoad(#[from] BulkLoadError),

    /// A loaded PersistedScrapeResult failed schema validation or deserialization.
    #[error("failed to deserialize archived scrape output for url_hash {key_hex}: {message}")]
    DeserializationFailed {
        /// Hex-encoded key of the corrupt archive.
        key_hex: String,
        /// Error description from rkyv or schema validation.
        message: String,
    },

    /// Hash integrity violation: loaded page's content_hash does not match
    /// the stored UrlStateRaw.content_hash for the same URL.
    #[error("hash mismatch for '{url}': stored={stored_hex}, loaded={loaded_hex}")]
    HashMismatch {
        /// URL of the affected page.
        url: String,
        /// Hex-encoded stored content hash from UrlStateRaw.
        stored_hex: String,
        /// Hex-encoded content hash from the loaded scrape page.
        loaded_hex: String,
    },

    /// No url_state entry exists for a URL that was expected to be unchanged.
    #[error("missing url_state for expected-unchanged URL '{url}'")]
    MissingUrlState {
        /// URL with no url_state entry.
        url: String,
    },
}

// Re-export types used by submodules so they can share them via super::
pub(super) use crate::persisted::{persisted_scraped_page_to_runtime, PersistedScrapeResult};

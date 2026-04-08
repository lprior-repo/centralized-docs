use crate::diff::DiffError;
use crate::state::bulk_load::BulkLoadError;
use crate::state::StateLoadError;

/// Error type for the analysis-reuse pipeline.
#[derive(Debug, thiserror::Error)]
pub enum ReuseAnalysisError {
    /// Failed to load file states from the state database.
    #[error("failed to load file states: {0}")]
    StateLoad(#[from] StateLoadError),

    /// Failed to load archived analyses from the state database.
    #[error("failed to load archived analyses: {0}")]
    BulkLoad(#[from] BulkLoadError),

    /// Failed to compute file diff.
    #[error("failed to compute file diff: {0}")]
    DiffError(#[from] DiffError),

    /// All files failed analysis (no successful analyses).
    #[error("all {count} file(s) failed analysis. Errors: {error_summary}")]
    AllFilesFailed {
        /// Number of files that failed.
        count: usize,
        /// Summary of errors encountered.
        error_summary: String,
    },
}

/// Statistics about analysis reuse within a single `run_index` invocation.
#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub struct AnalyzeReuseStats {
    /// Number of analyses loaded from archived state (zero-cost reuse).
    pub reused: usize,
    /// Number of analyses computed fresh via `analyze_single_file`.
    pub analyzed: usize,
}

//! Implementation for bead cdocs-b5h: Reuse archived analyses for unchanged files.
//!
//! Data flow:
//! ```text
//! Input:  files, source_dir, config_path, session
//!             |
//!             v
//!     [load_file_states(session)]
//!             |
//!             v
//!     HashMap<String, FileStateRaw>
//!             |
//!             v
//!     [build_stored_hashes]
//!             |
//!             v
//!     HashMap<String, StoredHashes>
//!             |
//!             v
//!     [compute_file_diff(files, source_dir, config_path, stored_hashes)]
//!             |
//!             v
//!     FileDiff { unchanged, changed, new, deleted }
//!             |
//!             v
//!     [partition_for_reuse(files, diff)]
//!             |
//!        _____|_____
//!       |           |
//!       v           v
//!  unchanged     changed+new
//!  paths         files
//!       |           |
//!       v           |
//!   [load_archived  |
//!    _analyses]     |
//!       |           |
//!    (Vec<Analysis>,|
//!     fallback)     |
//!       |           |
//!       v           v
//!   fallback + changed + new files
//!       |
//!       v
//!   [analyze_files(subset, source_dir, config_path)]
//!       |
//!       v
//!   Vec<Analysis> (fresh)
//!       |
//!       v
//!   [merge_analyses_in_order]
//!       |
//!       v
//!   (AnalyzeResult, AnalyzeReuseStats)
//! ```

pub mod helpers;
pub mod pipeline;
pub mod types;

// Re-export all public API for backward compatibility
pub use helpers::{build_stored_hashes, merge_analyses_in_order, partition_for_reuse};
pub use pipeline::{analyze_with_reuse, load_archived_analyses};
pub use types::{AnalyzeReuseStats, ReuseAnalysisError};

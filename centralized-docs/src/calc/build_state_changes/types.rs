//! Domain types for state change computation.

use crate::analyze::Analysis;
use crate::chunking_adapter::Chunk;
use crate::discover::DiscoveryFile;
use std::collections::HashMap;

// ---------------------------------------------------------------------------
// FileDiff
// ---------------------------------------------------------------------------

/// Partition of discovered files into unchanged, changed, new, and deleted buckets.
#[derive(Debug, Clone)]
pub struct FileDiff {
    pub unchanged: Vec<(DiscoveryFile, FileStateRaw)>,
    pub changed: Vec<DiscoveryFile>,
    pub new_files: Vec<DiscoveryFile>,
    pub deleted: Vec<String>,
}

// ---------------------------------------------------------------------------
// FileStateRaw
// ---------------------------------------------------------------------------

/// Fixed 200-byte Pod struct holding content/config/analysis/transform/chunk
/// hashes plus timestamp.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(C)]
pub struct FileStateRaw {
    pub content_hash: [u8; 32],
    pub config_hash: [u8; 32],
    pub analysis_hash: [u8; 32],
    pub transform_hash: [u8; 32],
    pub chunk_hash: [u8; 32],
    pub last_processed_secs: u64,
    pub reserved: [u8; 32],
}

// Static assertion: FileStateRaw is exactly 200 bytes.
const _: () = assert!(std::mem::size_of::<FileStateRaw>() == 200);

// ---------------------------------------------------------------------------
// UrlStateRaw
// ---------------------------------------------------------------------------

/// Placeholder type for URL state rows (populated by a separate bead).
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct UrlStateRaw {
    pub placeholder: [u8; 0],
}

// ---------------------------------------------------------------------------
// StateChanges
// ---------------------------------------------------------------------------

/// Batch of updated file rows, deleted file keys, and new payload blobs.
#[derive(Debug, Clone)]
pub struct StateChanges {
    pub updated_files: Vec<(String, FileStateRaw)>,
    pub deleted_files: Vec<String>,
    pub new_analyses: Vec<([u8; 32], Vec<u8>)>,
    pub new_transforms: Vec<([u8; 32], Vec<u8>)>,
    pub new_chunks: Vec<([u8; 32], Vec<u8>)>,
    // URL state fields — populated by a separate calc bead (cdocs-drj).
    pub updated_urls: Vec<(String, UrlStateRaw)>,
    pub deleted_urls: Vec<String>,
    pub new_scrapes: Vec<([u8; 32], Vec<u8>)>,
    pub new_snapshots: Vec<([u8; 32], Vec<u8>)>,
    pub deleted_snapshots: Vec<[u8; 32]>,
}

// ---------------------------------------------------------------------------
// PipelineOutputs
// ---------------------------------------------------------------------------

/// Input bundle for `build_file_state_changes`.
/// Groups all pipeline outputs needed to derive the commit batch.
#[derive(Debug, Clone)]
pub struct PipelineOutputs {
    /// Analysis results keyed by `source_path`.
    pub analyses: HashMap<String, Analysis>,
    /// Transformed markdown content keyed by `source_path`.
    pub transforms: HashMap<String, String>,
    /// Chunked output keyed by `source_path`.
    pub chunks: HashMap<String, Vec<Chunk>>,
    /// SHA-256 of each file's current bytes, keyed by `source_path`.
    pub content_hashes: HashMap<String, [u8; 32]>,
    /// SHA-256 of the category config used for this run.
    pub config_hash: [u8; 32],
    /// Unix timestamp (seconds) for `last_processed_secs`.
    pub now_secs: u64,
}

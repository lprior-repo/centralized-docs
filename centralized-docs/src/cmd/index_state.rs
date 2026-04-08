//! State-aware pipeline helpers for the index command.
//!
//! Provides functions for loading cached analyses from the state database,
//! serializing fresh analyses to rkyv, and building [`StateChanges`] for
//! atomic commits.

use crate::analyze::Analysis;
use crate::cache::ContentHash;
use crate::diff::FileDiff;
use crate::discover::DiscoveryFile;
use crate::persisted::{
    analysis_to_persisted, persisted_analysis_to_runtime, PersistedAnalyzeResult,
};
use crate::state::bulk_load::StateReadSession;
use crate::state::commit::StateChanges;
use crate::state::FileStateRaw;
use anyhow::Result;
use std::collections::{HashMap, HashSet};
use std::path::Path;

#[path = "index_state_cleanup.rs"]
mod cleanup;

pub use cleanup::cleanup_deleted_outputs;

// ---------------------------------------------------------------------------
// Data: CachedAnalyses
// ---------------------------------------------------------------------------

/// Cached analysis data loaded from the state database for unchanged files.
pub struct CachedAnalyses {
    /// Deserialized [`Analysis`] structs for unchanged files.
    pub analyses: Vec<Analysis>,
    /// Raw rkyv bytes keyed by analysis hash (for re-inclusion in [`StateChanges`]).
    pub raw_entries: HashMap<[u8; 32], Vec<u8>>,
}

// ---------------------------------------------------------------------------
// Calculation: load_cached_analyses
// ---------------------------------------------------------------------------

/// Load cached [`Analysis`] structs for unchanged files from the state database.
///
/// For each unchanged file with a non-zero `analysis_hash` in its [`FileStateRaw`],
/// loads the corresponding [`PersistedAnalyzeResult`] from the `analysis_outputs`
/// table, deserializes it, and extracts the matching analysis by `source_path`.
///
/// Files with zeroed `analysis_hash` (never previously analyzed) are silently skipped.
pub fn load_cached_analyses(
    session: &StateReadSession,
    unchanged_paths: &HashSet<String>,
    file_states: &HashMap<String, FileStateRaw>,
) -> Result<CachedAnalyses> {
    let non_zero: Vec<[u8; 32]> = unchanged_paths
        .iter()
        .filter_map(|p| {
            file_states
                .get(p)
                .filter(|s| s.analysis_hash != [0u8; 32])
                .map(|s| s.analysis_hash)
        })
        .collect();

    if non_zero.is_empty() {
        return Ok(CachedAnalyses {
            analyses: Vec::new(),
            raw_entries: HashMap::new(),
        });
    }

    let cached = session
        .load_analyses(&non_zero)
        .map_err(|e| anyhow::anyhow!("load cached analyses: {e}"))?;

    let path_hash: HashMap<String, [u8; 32]> = unchanged_paths
        .iter()
        .filter_map(|p| {
            file_states
                .get(p)
                .filter(|s| s.analysis_hash != [0u8; 32])
                .map(|s| (p.clone(), s.analysis_hash))
        })
        .collect();

    let raw: HashMap<[u8; 32], Vec<u8>> = cached
        .iter()
        .map(|(h, a)| (*h, a.as_bytes().to_vec()))
        .collect();

    let analyses: Vec<Analysis> = path_hash
        .iter()
        .filter_map(|(p, h)| {
            cached.get(h).and_then(|arc| {
                arc.deserialize()
                    .map_err(|e| {
                        tracing::warn!(
                            path = %p,
                            error = %e,
                            "Failed to deserialize cached analysis — skipping"
                        );
                        e
                    })
                    .ok()
                    .and_then(|r: PersistedAnalyzeResult| {
                        r.analyses
                            .iter()
                            .find(|x| x.source_path == *p)
                            .and_then(|pa| {
                                persisted_analysis_to_runtime(pa)
                                    .map_err(|e| {
                                        tracing::warn!(
                                            path = %p,
                                            error = %e,
                                            "Failed to convert persisted analysis — skipping"
                                        );
                                        e
                                    })
                                    .ok()
                            })
                    })
            })
        })
        .collect();

    Ok(CachedAnalyses {
        analyses,
        raw_entries: raw,
    })
}

// ---------------------------------------------------------------------------
// Calculation: serialize_analysis
// ---------------------------------------------------------------------------

/// Serialize a single [`Analysis`] to a per-file [`PersistedAnalyzeResult`],
/// then to rkyv bytes, and compute its SHA-256 hash.
///
/// Returns `(hash, rkyv_bytes)` suitable for [`StateChanges::new_analyses`].
pub fn serialize_analysis(analysis: &Analysis) -> Result<([u8; 32], Vec<u8>)> {
    let persisted = PersistedAnalyzeResult {
        schema_version: 1,
        analyses: vec![analysis_to_persisted(analysis)],
        failed_files: vec![],
        total_discovered: 1,
    };
    let bytes = rkyv::to_bytes::<rkyv::rancor::Error>(&persisted)
        .map_err(|e| anyhow::anyhow!("serialize analysis: {e}"))?
        .to_vec();
    Ok((crate::cache::content_hash(&bytes).into(), bytes))
}

// ---------------------------------------------------------------------------
// Action: build_state_changes
// ---------------------------------------------------------------------------

/// Build [`StateChanges`] for committing pipeline results to the state database.
///
/// Reads file bytes from disk for changed/new files to compute content hashes.
/// Produces per-file [`FileStateRaw`] entries for all discovered files and
/// serialized analysis payloads for both cached (unchanged) and fresh
/// (changed / new) files. Transform and chunk hashes are zeroed (their outputs
/// already exist on disk); full output caching is deferred to a follow-up bead.
pub fn build_state_changes(
    files: &[DiscoveryFile],
    source_dir: &Path,
    file_states: &HashMap<String, FileStateRaw>,
    file_diff: &FileDiff,
    config_hash: &ContentHash,
    cached_raw: &HashMap<[u8; 32], Vec<u8>>,
    fresh: &[Analysis],
) -> Result<StateChanges> {
    let now = std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .map_or(0, |d| d.as_secs());
    let cfg: [u8; 32] = (*config_hash).into();

    let fresh_ser: Vec<(String, [u8; 32], Vec<u8>)> = fresh
        .iter()
        .map(|a| serialize_analysis(a).map(|(h, b)| (a.source_path.clone(), h, b)))
        .collect::<Result<Vec<_>>>()?;

    let fhash: HashMap<&str, [u8; 32]> =
        fresh_ser.iter().map(|(p, h, _)| (p.as_str(), *h)).collect();

    let cn_set: HashSet<&str> = file_diff
        .changed
        .iter()
        .chain(file_diff.new.iter())
        .map(String::as_str)
        .collect();

    // Unchanged files: reuse existing FileStateRaw from state.
    let unchanged: Vec<(String, FileStateRaw)> = file_diff
        .unchanged
        .iter()
        .filter_map(|p| file_states.get(p).map(|s| (p.clone(), *s)))
        .collect();

    // Changed+new files: compute fresh FileStateRaw with current hashes.
    let changed_new: Vec<(String, FileStateRaw)> = files
        .iter()
        .filter(|f| cn_set.contains(f.source_path.as_str()))
        .map(|f| {
            let full = source_dir.join(&f.source_path);
            let bytes =
                std::fs::read(&full).map_err(|e| anyhow::anyhow!("read {}: {e}", f.source_path))?;
            Ok((
                f.source_path.clone(),
                FileStateRaw {
                    content_hash: crate::cache::content_hash(&bytes).into(),
                    config_hash: cfg,
                    analysis_hash: fhash.get(f.source_path.as_str()).map_or([0u8; 32], |&h| h),
                    transform_hash: [0u8; 32],
                    chunk_hash: [0u8; 32],
                    last_processed_secs: now,
                    reserved: [0u8; 32],
                },
            ))
        })
        .collect::<Result<Vec<_>>>()?;

    // new_analyses = cached raw bytes ∪ fresh serialized bytes.
    let new_analyses: Vec<([u8; 32], Vec<u8>)> = cached_raw
        .iter()
        .map(|(h, b)| (*h, b.clone()))
        .chain(fresh_ser.into_iter().map(|(_, h, b)| (h, b)))
        .collect();

    Ok(StateChanges {
        updated_files: unchanged.into_iter().chain(changed_new).collect(),
        deleted_files: file_diff.deleted.iter().cloned().collect(),
        new_analyses,
        ..StateChanges::default()
    })
}

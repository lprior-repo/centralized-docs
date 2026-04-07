//! Pure validation functions for commit preconditions.
//!
//! All functions are deterministic and side-effect-free.
//! They run before opening a write transaction.

#![deny(clippy::unwrap_used)]
#![deny(clippy::expect_used)]
#![deny(clippy::panic)]
#![forbid(unsafe_code)]

use super::{CommitError, StateChanges, MAX_VALUE_SIZE, ZERO_HASH};
use std::collections::HashSet;
use std::fmt::Write;

/// Returns `true` when `existing` and `new` are byte-identical, indicating
/// the write can be safely skipped without changing observable state.
#[must_use]
pub fn should_skip_write(existing: &[u8], new: &[u8]) -> bool {
    existing == new
}

/// Run all precondition checks. Returns the first error found.
/// Order: zero hashes → empty strings → duplicates → payload sizes → references.
pub(crate) fn validate_all(changes: &StateChanges) -> Result<(), CommitError> {
    validate_no_zero_hashes(changes)?;
    validate_no_empty_string_keys(changes)?;
    validate_no_duplicate_keys(changes)?;
    validate_payload_sizes(changes)?;
    validate_reference_integrity(changes)?;
    Ok(())
}

/// P1: Reject any zero hash key in payload vecs (except snapshots, where
/// the URL hash is the key and a zero hash is a valid — if unlikely — value).
pub(crate) fn validate_no_zero_hashes(changes: &StateChanges) -> Result<(), CommitError> {
    check_zero_hash(&changes.new_analyses, "analysis_outputs")?;
    check_zero_hash(&changes.new_transforms, "transform_outputs")?;
    check_zero_hash(&changes.new_chunks, "chunk_outputs")?;
    check_zero_hash(&changes.new_scrapes, "scrape_outputs")?;
    Ok(())
}

fn check_zero_hash(
    entries: &[([u8; 32], Vec<u8>)],
    table: &'static str,
) -> Result<(), CommitError> {
    entries
        .iter()
        .enumerate()
        .find_map(|(idx, (hash, _))| {
            (*hash == ZERO_HASH).then_some(CommitError::ZeroHashKey { table, index: idx })
        })
        .map_or(Ok(()), Err)
}

/// P2: Reject empty/whitespace-only string keys.
pub(crate) fn validate_no_empty_string_keys(changes: &StateChanges) -> Result<(), CommitError> {
    check_empty_string_keys(&changes.updated_files, "file_state")?;
    check_empty_string_keys(&changes.updated_urls, "url_state")?;
    Ok(())
}

fn check_empty_string_keys<S>(
    entries: &[(String, S)],
    table: &'static str,
) -> Result<(), CommitError> {
    entries
        .iter()
        .enumerate()
        .find_map(|(idx, (key, _))| {
            key.trim()
                .is_empty()
                .then_some(CommitError::EmptyStringKey { table, index: idx })
        })
        .map_or(Ok(()), Err)
}

/// P3: Reject duplicate string keys in state-table vecs.
pub(crate) fn validate_no_duplicate_keys(changes: &StateChanges) -> Result<(), CommitError> {
    check_duplicate_keys(&changes.updated_files, "file_state")?;
    check_duplicate_keys(&changes.updated_urls, "url_state")?;
    Ok(())
}

fn check_duplicate_keys<S>(
    entries: &[(String, S)],
    table: &'static str,
) -> Result<(), CommitError> {
    let mut seen = HashSet::new();
    entries
        .iter()
        .find_map(|(key, _)| {
            if seen.contains(key) {
                Some(CommitError::DuplicateStateKey {
                    table,
                    key: key.clone(),
                })
            } else {
                seen.insert(key.clone());
                None
            }
        })
        .map_or(Ok(()), Err)
}

/// P6: Reject payloads exceeding [`MAX_VALUE_SIZE`].
pub(crate) fn validate_payload_sizes(changes: &StateChanges) -> Result<(), CommitError> {
    check_payload_size(&changes.new_analyses, "analysis_outputs")?;
    check_payload_size(&changes.new_transforms, "transform_outputs")?;
    check_payload_size(&changes.new_chunks, "chunk_outputs")?;
    check_payload_size(&changes.new_scrapes, "scrape_outputs")?;
    check_payload_size(&changes.new_snapshots, "snapshots")?;
    Ok(())
}

pub(crate) fn check_payload_size(
    entries: &[([u8; 32], Vec<u8>)],
    table: &'static str,
) -> Result<(), CommitError> {
    entries
        .iter()
        .find_map(|(_, value)| {
            (value.len() > MAX_VALUE_SIZE).then_some(CommitError::PayloadTooLarge {
                table,
                size: value.len(),
                max: MAX_VALUE_SIZE,
            })
        })
        .map_or(Ok(()), Err)
}

/// P4: Reference integrity — every non-zero hash in state rows resolves to a payload entry.
pub(crate) fn validate_reference_integrity(changes: &StateChanges) -> Result<(), CommitError> {
    let analysis_set: HashSet<[u8; 32]> = changes.new_analyses.iter().map(|(h, _)| *h).collect();
    let transform_set: HashSet<[u8; 32]> = changes.new_transforms.iter().map(|(h, _)| *h).collect();
    let chunk_set: HashSet<[u8; 32]> = changes.new_chunks.iter().map(|(h, _)| *h).collect();
    let scrape_set: HashSet<[u8; 32]> = changes.new_scrapes.iter().map(|(h, _)| *h).collect();

    for (_, state) in &changes.updated_files {
        check_ref(
            &state.analysis_hash,
            &analysis_set,
            "file_state",
            "analysis_hash",
            "analysis_outputs",
        )?;
        check_ref(
            &state.transform_hash,
            &transform_set,
            "file_state",
            "transform_hash",
            "transform_outputs",
        )?;
        check_ref(
            &state.chunk_hash,
            &chunk_set,
            "file_state",
            "chunk_hash",
            "chunk_outputs",
        )?;
    }

    for (_, state) in &changes.updated_urls {
        check_ref(
            &state.url_hash,
            &scrape_set,
            "url_state",
            "url_hash",
            "scrape_outputs",
        )?;
    }

    Ok(())
}

fn check_ref(
    hash: &[u8; 32],
    known: &HashSet<[u8; 32]>,
    table: &'static str,
    field: &'static str,
    payload_table: &'static str,
) -> Result<(), CommitError> {
    if *hash == ZERO_HASH {
        return Ok(());
    }
    known
        .contains(hash)
        .then_some(())
        .ok_or_else(|| CommitError::MissingReference {
            table,
            field,
            hash_hex: hash_to_hex(hash),
            payload_table,
        })
}

/// Convert a 32-byte hash to a lowercase hex string.
pub(crate) fn hash_to_hex(hash: &[u8; 32]) -> String {
    hash.iter().fold(String::with_capacity(64), |mut acc, b| {
        let _ = write!(acc, "{b:02x}");
        acc
    })
}

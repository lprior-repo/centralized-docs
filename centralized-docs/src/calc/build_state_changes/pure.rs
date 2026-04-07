//! Pure functions: hashing, serialization, and FileStateRaw construction.

use serde::Serialize;
use sha2::{Digest, Sha256};

use super::error::BatchBuildError;
use super::types::FileStateRaw;

/// Compute the SHA-256 hash of arbitrary bytes.
///
/// Returns a non-zero `[u8; 32]` for any non-empty input.
/// Deterministic: same input always produces same output.
#[must_use]
pub fn hash_payload(rkyv_bytes: &[u8]) -> [u8; 32] {
    let digest = Sha256::digest(rkyv_bytes);
    let mut array = [0u8; 32];
    array.copy_from_slice(&digest);
    array
}

/// Serialize an artifact and return both the bytes and their content hash.
///
/// # Errors
///
/// Returns `BatchBuildError::AnalysisSerializationFailed` if serialization fails.
pub fn serialize_and_hash<T: Serialize + ?Sized>(
    value: &T,
    path: &str,
) -> Result<([u8; 32], Vec<u8>), BatchBuildError> {
    let bytes =
        serde_json::to_vec(value).map_err(|e| BatchBuildError::AnalysisSerializationFailed {
            path: path.to_string(),
            reason: e.to_string(),
        })?;
    Ok((hash_payload(&bytes), bytes))
}

/// Construct a `FileStateRaw` from individual hash components.
///
/// All hash fields are set to the provided values, `last_processed_secs` to `now_secs`,
/// and `reserved` is zeroed. Total struct size is exactly 200 bytes.
#[allow(clippy::too_many_arguments)]
#[must_use]
pub fn build_file_state_raw(
    content_hash: [u8; 32],
    config_hash: [u8; 32],
    analysis_hash: [u8; 32],
    transform_hash: [u8; 32],
    chunk_hash: [u8; 32],
    now_secs: u64,
) -> FileStateRaw {
    FileStateRaw {
        content_hash,
        config_hash,
        analysis_hash,
        transform_hash,
        chunk_hash,
        last_processed_secs: now_secs,
        reserved: [0u8; 32],
    }
}

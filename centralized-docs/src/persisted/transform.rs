//! Persisted types and conversions for the Transform pipeline phase.

use super::error::{require_non_empty, require_schema_v1, PersistError};
use crate::transform::{TransformError, TransformResult};

// ---------------------------------------------------------------------------
// Persisted Record Types — Transform Family
// ---------------------------------------------------------------------------

/// Persisted transform error: `source_path` + error message.
#[derive(Debug, Clone, rkyv::Archive, rkyv::Serialize, rkyv::Deserialize)]
pub struct PersistedTransformError {
    /// Path of the file that failed transform.
    pub source_path: String,
    /// Error message from the failed transform.
    pub error: String,
}

/// Persisted batch transform result.
#[derive(Debug, Clone, rkyv::Archive, rkyv::Serialize, rkyv::Deserialize)]
pub struct PersistedTransformResult {
    /// Schema version (always 1).
    pub schema_version: u32,
    /// Number of successfully transformed documents.
    pub success_count: usize,
    /// Total documents attempted.
    pub total_count: usize,
    /// Number of failed transforms.
    pub error_count: usize,
    /// Detailed errors.
    pub errors: Vec<PersistedTransformError>,
}

// ===========================================================================
// Conversions: Runtime → Persisted (Infallible)
// ===========================================================================

/// Convert a runtime [`TransformError`] to its persisted form.
#[must_use]
pub fn transform_error_to_persisted(e: &TransformError) -> PersistedTransformError {
    PersistedTransformError {
        source_path: e.source_path.clone(),
        error: e.error.clone(),
    }
}

/// Convert a runtime [`TransformResult`] to its persisted form.
#[must_use]
pub fn transform_result_to_persisted(r: &TransformResult) -> PersistedTransformResult {
    PersistedTransformResult {
        schema_version: 1,
        success_count: r.success_count,
        total_count: r.total_count,
        error_count: r.error_count,
        errors: r.errors.iter().map(transform_error_to_persisted).collect(),
    }
}

// ===========================================================================
// Conversions: Persisted → Runtime (Fallible)
// ===========================================================================

/// Convert a persisted transform error back to runtime form.
///
/// # Errors
///
/// Returns [`PersistError::EmptyField`] if `source_path` or error is empty.
pub fn persisted_transform_error_to_runtime(
    p: &PersistedTransformError,
) -> Result<TransformError, PersistError> {
    require_non_empty(&p.source_path, "source_path")?;
    require_non_empty(&p.error, "error")?;
    Ok(TransformError {
        source_path: p.source_path.clone(),
        error: p.error.clone(),
    })
}

/// Convert a persisted transform result back to runtime form.
///
/// # Errors
///
/// Returns [`PersistError::SchemaVersionMismatch`] if `schema_version` != 1.
/// Propagates any errors from nested error conversions.
pub fn persisted_transform_result_to_runtime(
    p: &PersistedTransformResult,
) -> Result<TransformResult, PersistError> {
    require_schema_v1(p.schema_version)?;
    let errors = p
        .errors
        .iter()
        .map(persisted_transform_error_to_runtime)
        .collect::<Result<Vec<_>, _>>()?;
    Ok(TransformResult {
        success_count: p.success_count,
        total_count: p.total_count,
        error_count: p.error_count,
        errors,
    })
}

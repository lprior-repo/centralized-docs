//! Persisted types and conversions for the Assign pipeline phase.

use super::error::{require_non_empty, PersistError};
use crate::assign::IdMapping;

// ---------------------------------------------------------------------------
// Persisted Record Types — Assign Family
// ---------------------------------------------------------------------------

/// Persisted ID mapping: `source_path` + assigned document identity.
#[derive(Debug, Clone, rkyv::Archive, rkyv::Serialize, rkyv::Deserialize)]
pub struct PersistedIdMapping {
    /// Source file path (key into `link_map`).
    pub source_path: String,
    /// Assigned document ID (e.g., "concept/general/my-doc").
    pub id: String,
    /// Output filename (e.g., "concept-general-my-doc.md").
    pub filename: String,
    /// Subcategory extracted from path.
    pub subcategory: String,
    /// URL-safe slug.
    pub slug: String,
}

// ===========================================================================
// Conversions: Runtime → Persisted (Infallible)
// ===========================================================================

/// Convert a `source_path` and runtime [`IdMapping`] to its persisted form.
#[must_use]
pub fn id_mapping_to_persisted(source_path: &str, m: &IdMapping) -> PersistedIdMapping {
    PersistedIdMapping {
        source_path: source_path.to_string(),
        id: m.id.clone(),
        filename: m.filename.clone(),
        subcategory: m.subcategory.clone(),
        slug: m.slug.clone(),
    }
}

// ===========================================================================
// Conversions: Persisted → Runtime (Fallible)
// ===========================================================================

/// Convert a persisted ID mapping back to runtime form.
///
/// # Errors
///
/// Returns [`PersistError::EmptyField`] if id, filename, subcategory, or slug is empty.
pub fn persisted_id_mapping_to_runtime(
    p: &PersistedIdMapping,
) -> Result<(String, IdMapping), PersistError> {
    require_non_empty(&p.source_path, "source_path")?;
    require_non_empty(&p.id, "id")?;
    require_non_empty(&p.filename, "filename")?;
    require_non_empty(&p.subcategory, "subcategory")?;
    require_non_empty(&p.slug, "slug")?;
    Ok((
        p.source_path.clone(),
        IdMapping {
            id: p.id.clone(),
            filename: p.filename.clone(),
            subcategory: p.subcategory.clone(),
            slug: p.slug.clone(),
        },
    ))
}

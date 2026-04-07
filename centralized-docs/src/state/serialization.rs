//! Snapshot serialization helpers.

#![deny(clippy::unwrap_used)]
#![deny(clippy::expect_used)]
#![deny(clippy::panic)]
#![warn(clippy::pedantic)]
#![forbid(unsafe_code)]

use super::error::StateError;

/// Serialize a [`crate::watch::Snapshot`] into rkyv bytes.
///
/// Serializes directly using `Snapshot`'s rkyv `Archive` impl.
/// The `DateTime<Utc>` field is handled via the `DateTimeWrap` wrapper
/// which stores timestamps as ISO 8601 strings.
///
/// # Errors
///
/// Returns [`StateError::SerializationFailed`] if rkyv serialization fails.
pub fn serialize_snapshot(snapshot: &crate::watch::Snapshot) -> Result<Vec<u8>, StateError> {
    rkyv::to_bytes::<rkyv::rancor::Error>(snapshot)
        .map(|aligned| aligned.to_vec())
        .map_err(|e| StateError::SerializationFailed {
            type_name: "Snapshot",
            message: e.to_string(),
        })
}

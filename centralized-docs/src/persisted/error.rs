use thiserror::Error;

/// Errors that can occur during persisted ↔ runtime type conversions.
#[derive(Debug, Clone, Error)]
pub enum PersistError {
    /// A required String field was empty or whitespace-only.
    #[error("field '{field}' must be non-empty")]
    EmptyField {
        /// Name of the empty field.
        field: String,
    },

    /// A numeric field was outside its valid range.
    #[error("field '{field}' value {value} is out of range {min}..={max}")]
    OutOfRange {
        /// Name of the out-of-range field.
        field: String,
        /// The actual value observed.
        value: i64,
        /// Minimum allowed value (inclusive).
        min: i64,
        /// Maximum allowed value (inclusive).
        max: i64,
    },

    /// The persisted record's schema version does not match expected.
    #[error("schema version mismatch: expected {expected}, got {actual}")]
    SchemaVersionMismatch {
        /// Expected schema version.
        expected: u32,
        /// Actual schema version found in the record.
        actual: u32,
    },

    /// rkyv serialization failed (buffer allocation or write error).
    #[error("serialization failed: {reason}")]
    SerializationFailed {
        /// Description of the failure.
        reason: String,
    },

    /// rkyv deserialization / validation failed (corrupted bytes).
    #[error("deserialization failed: {reason}")]
    DeserializationFailed {
        /// Description of the failure.
        reason: String,
    },

    /// An enum variant that doesn't map to any known runtime value.
    #[error("unknown enum variant for '{type_name}'")]
    UnknownVariant {
        /// The type name whose variant was unknown.
        type_name: String,
    },

    /// A float field was `NaN` or Infinite where finite was expected.
    #[error("field '{field}' must be a finite number, got {value}")]
    NonFiniteFloat {
        /// Name of the non-finite field.
        field: String,
        /// String representation of the invalid value.
        value: String,
    },

    /// A content hash was not exactly 32 bytes.
    #[error("content hash must be exactly 32 bytes, got {actual_len}")]
    InvalidHashLength {
        /// The actual byte length observed.
        actual_len: usize,
    },
}

// ---------------------------------------------------------------------------
// Helper Validation Functions (pure)
// ---------------------------------------------------------------------------

pub(super) fn require_non_empty(value: &str, field: &str) -> Result<(), PersistError> {
    if value.trim().is_empty() {
        return Err(PersistError::EmptyField {
            field: field.to_string(),
        });
    }
    Ok(())
}

pub(super) fn require_range(
    value: i64,
    min: i64,
    max: i64,
    field: &str,
) -> Result<(), PersistError> {
    if !(min..=max).contains(&value) {
        return Err(PersistError::OutOfRange {
            field: field.to_string(),
            value,
            min,
            max,
        });
    }
    Ok(())
}

pub(super) fn require_schema_v1(version: u32) -> Result<(), PersistError> {
    if version != 1 {
        return Err(PersistError::SchemaVersionMismatch {
            expected: 1,
            actual: version,
        });
    }
    Ok(())
}

pub(super) fn require_finite_f32(value: f32, field: &str) -> Result<(), PersistError> {
    if !value.is_finite() {
        return Err(PersistError::NonFiniteFloat {
            field: field.to_string(),
            value: format!("{value}"),
        });
    }
    Ok(())
}

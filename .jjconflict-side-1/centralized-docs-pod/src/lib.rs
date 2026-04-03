//! Fixed-size Pod state types for zero-copy redb storage.
//!
//! This crate defines `FileStateRaw` (104 bytes) and `UrlStateRaw` (112 bytes) as
//! `#[repr(C)]` Pod structs that implement `bytemuck::Pod` and `bytemuck::Zeroable`.
//! These enable safe zero-copy byte casting for redb table values, eliminating
//! `serde_json` serialization overhead.
//!
//! # Why a separate crate?
//!
//! The parent crate `centralized-docs` has `#![forbid(unsafe_code)]` at the crate level.
//! Bytemuck's derive macros for `Pod`/`Zeroable` emit `unsafe impl` blocks, which are
//! rejected by `forbid(unsafe_code)`. This crate intentionally allows `unsafe_code`
//! solely for bytemuck's derive expansions — no manual unsafe code is written.

#![deny(clippy::unwrap_used)]
#![deny(clippy::expect_used)]
#![deny(clippy::panic)]
#![warn(clippy::pedantic)]
#![allow(clippy::missing_errors_doc)]
#![allow(clippy::doc_markdown)]
#![allow(unknown_lints)]
#![allow(clippy::pub_underscore_fields)]
#![allow(clippy::used_underscore_binding)]

use bytemuck::{Pod, Zeroable};
use thiserror::Error;

// ---------------------------------------------------------------------------
// Constants
// ---------------------------------------------------------------------------

/// Current schema version for both Pod structs.
const SCHEMA_VERSION: u8 = 1;

/// Expected size of `FileStateRaw` in bytes.
pub const FILE_STATE_RAW_SIZE: usize = 104;

/// Expected size of `UrlStateRaw` in bytes.
pub const URL_STATE_RAW_SIZE: usize = 112;

/// Offset of `reserved` field in `FileStateRaw`.
const FILE_RESERVED_OFFSET: usize = 50;

/// Offset of `_pad1` field in `UrlStateRaw`.
const URL_PAD1_OFFSET: usize = 34;

/// Offset of `reserved` field in `UrlStateRaw`.
const URL_RESERVED_OFFSET: usize = 58;

/// Offset of `version` field in `FileStateRaw` (test-only for byte manipulation).
#[cfg(test)]
const FILE_VERSION_OFFSET: usize = 48;

/// Offset of `status` field in `FileStateRaw` (test-only for byte manipulation).
#[cfg(test)]
const FILE_STATUS_OFFSET: usize = 49;

/// Offset of `version` field in `UrlStateRaw` (test-only for byte manipulation).
#[cfg(test)]
const URL_VERSION_OFFSET: usize = 56;

/// Offset of `status` field in `UrlStateRaw` (test-only for byte manipulation).
#[cfg(test)]
const URL_STATUS_OFFSET: usize = 57;

// ---------------------------------------------------------------------------
// Error taxonomy
// ---------------------------------------------------------------------------

/// Errors that can occur when constructing or validating Pod state types.
#[non_exhaustive]
#[derive(Debug, Clone, PartialEq, Eq, Error)]
pub enum PodStateError {
    /// The status byte is not a valid discriminant for `FileStateStatus`.
    #[error("invalid file state status byte: {0} (valid: 0-3)")]
    InvalidFileStatus(u8),

    /// The status byte is not a valid discriminant for `UrlStateStatus`.
    #[error("invalid URL state status byte: {0} (valid: 0-3)")]
    InvalidUrlStatus(u8),

    /// A byte slice does not match the expected struct size.
    #[error("byte slice has wrong size for {type_name}: got {actual}, expected {expected}")]
    WrongByteSize {
        /// Type name for error context.
        type_name: &'static str,
        /// Actual byte count received.
        actual: usize,
        /// Expected byte count.
        expected: usize,
    },

    /// The version field does not match the expected schema version.
    #[error("unsupported schema version for {type_name}: got {actual}, expected {expected}")]
    VersionMismatch {
        /// Type name for error context.
        type_name: &'static str,
        /// Version found in the byte slice.
        actual: u8,
        /// Expected schema version.
        expected: u8,
    },

    /// Reserved or padding bytes contain non-zero values.
    #[error("reserved bytes contain non-zero values in {type_name} at offset {offset}")]
    ReservedBytesNonZero {
        /// Type name for error context.
        type_name: &'static str,
        /// Byte offset of the first non-zero value.
        offset: usize,
    },
}

// ---------------------------------------------------------------------------
// Status enums
// ---------------------------------------------------------------------------

/// Status of a cached file entry.
#[repr(u8)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum FileStateStatus {
    /// File state is unknown (default/zero value).
    Unknown = 0,
    /// File has not changed since last processing.
    Unchanged = 1,
    /// File has been modified since last processing.
    Modified = 2,
    /// File has been deleted.
    Deleted = 3,
}

impl FileStateStatus {
    /// Convert from a raw `u8` discriminant.
    ///
    /// Returns `Ok` for values 0–3, `Err` otherwise.
    pub fn from_discriminant(value: u8) -> Result<Self, PodStateError> {
        match value {
            0 => Ok(Self::Unknown),
            1 => Ok(Self::Unchanged),
            2 => Ok(Self::Modified),
            3 => Ok(Self::Deleted),
            v => Err(PodStateError::InvalidFileStatus(v)),
        }
    }
}

/// Status of a cached URL entry.
#[repr(u8)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum UrlStateStatus {
    /// URL state is unknown (default/zero value).
    Unknown = 0,
    /// URL content is fresh (recently fetched).
    Fresh = 1,
    /// URL content is stale (needs re-fetch).
    Stale = 2,
    /// URL fetch resulted in an error.
    Error = 3,
}

impl UrlStateStatus {
    /// Convert from a raw `u8` discriminant.
    ///
    /// Returns `Ok` for values 0–3, `Err` otherwise.
    pub fn from_discriminant(value: u8) -> Result<Self, PodStateError> {
        match value {
            0 => Ok(Self::Unknown),
            1 => Ok(Self::Fresh),
            2 => Ok(Self::Stale),
            3 => Ok(Self::Error),
            v => Err(PodStateError::InvalidUrlStatus(v)),
        }
    }
}

// ---------------------------------------------------------------------------
// FileStateRaw
// ---------------------------------------------------------------------------

/// Fixed-size Pod struct for cached file state.
///
/// # Layout (104 bytes total)
///
/// | Offset | Size | Field              | Type      |
/// |--------|------|--------------------|-----------|
/// | 0      | 32   | content_hash       | [u8; 32]  |
/// | 32     | 8    | file_size          | u64       |
/// | 40     | 8    | last_modified_ms   | u64       |
/// | 48     | 1    | version            | u8        |
/// | 49     | 1    | status             | u8        |
/// | 50     | 54   | reserved           | [u8; 54]  |
#[repr(C)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Pod, Zeroable)]
pub struct FileStateRaw {
    /// SHA-256 content hash.
    pub content_hash: [u8; 32],
    /// File size in bytes.
    pub file_size: u64,
    /// Last modification time in milliseconds since Unix epoch.
    pub last_modified_ms: u64,
    /// Schema version (currently 1).
    pub version: u8,
    /// Status discriminant (`FileStateStatus`).
    pub status: u8,
    /// Reserved bytes for future schema evolution. MUST be zero.
    pub reserved: [u8; 54],
}

impl FileStateRaw {
    /// Create a new `FileStateRaw` with the given fields.
    ///
    /// Sets `version` to `1` and zeroes `reserved`.
    #[must_use]
    pub fn new(
        content_hash: [u8; 32],
        file_size: u64,
        last_modified_ms: u64,
        status: FileStateStatus,
    ) -> Self {
        Self {
            content_hash,
            file_size,
            last_modified_ms,
            version: SCHEMA_VERSION,
            status: status as u8,
            reserved: [0u8; 54],
        }
    }

    /// Create a zeroed `FileStateRaw` (all bytes zero).
    ///
    /// Note: `version` will be 0, so this is NOT a valid domain instance.
    /// Use `new()` for domain-valid instances.
    #[must_use]
    pub fn zeroed() -> Self {
        bytemuck::Zeroable::zeroed()
    }

    /// Parse a `FileStateRaw` from a byte slice, validating all invariants.
    ///
    /// Validation order: size → version → status → reserved bytes (fail-fast).
    pub fn from_bytes_checked(bytes: &[u8]) -> Result<Self, PodStateError> {
        if bytes.len() != FILE_STATE_RAW_SIZE {
            return Err(PodStateError::WrongByteSize {
                type_name: "FileStateRaw",
                actual: bytes.len(),
                expected: FILE_STATE_RAW_SIZE,
            });
        }

        let candidate: Self = *bytemuck::from_bytes::<Self>(bytes);

        candidate.validate()?;

        Ok(candidate)
    }

    /// Parse a `FileStateRaw` from a byte slice WITHOUT validation.
    ///
    /// Returns a zero-copy reference into the original bytes.
    ///
    /// # Safety (via bytemuck)
    ///
    /// - `bytes.len()` MUST equal `size_of::<FileStateRaw>()`.
    /// - Caller guarantees the bytes represent a valid instance.
    #[must_use]
    pub fn from_bytes_unchecked(bytes: &[u8]) -> &Self {
        bytemuck::from_bytes(bytes)
    }

    /// Borrow this instance as a byte slice (Pod cast).
    #[must_use]
    pub fn as_bytes(&self) -> &[u8] {
        bytemuck::bytes_of(self)
    }

    /// Get the status as a typed enum.
    pub fn status(&self) -> Result<FileStateStatus, PodStateError> {
        FileStateStatus::from_discriminant(self.status)
    }

    /// Validate all invariants of this instance.
    pub fn validate(&self) -> Result<(), PodStateError> {
        if self.version != SCHEMA_VERSION {
            return Err(PodStateError::VersionMismatch {
                type_name: "FileStateRaw",
                actual: self.version,
                expected: SCHEMA_VERSION,
            });
        }

        if FileStateStatus::from_discriminant(self.status).is_err() {
            return Err(PodStateError::InvalidFileStatus(self.status));
        }

        validate_reserved_zero("FileStateRaw", FILE_RESERVED_OFFSET, &self.reserved)
    }
}

// ---------------------------------------------------------------------------
// UrlStateRaw
// ---------------------------------------------------------------------------

/// Fixed-size Pod struct for cached URL state.
///
/// # Layout (112 bytes total)
///
/// | Offset | Size | Field              | Type      |
/// |--------|------|--------------------|-----------|
/// | 0      | 32   | `content_hash`       | [u8; 32]  |
/// | 32     | 2    | `http_status`        | u16       |
/// | 34     | 6    | `_pad1`              | [u8; 6]   |
/// | 40     | 8    | `content_length`     | u64       |
/// | 48     | 8    | `last_fetched_ms`    | u64       |
/// | 56     | 1    | `version`            | u8        |
/// | 57     | 1    | `status`             | u8        |
/// | 58     | 54   | `reserved`           | [u8; 54]  |
///
/// **Note**: `_pad1` is `[u8; 6]` (not `[u8; 2]`) because `u64` requires 8-byte
/// alignment under `#[repr(C)]`. After `http_status` (u16, offset 32), we need
/// 6 explicit padding bytes to reach offset 40 for `content_length` (u64).
#[repr(C)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Pod, Zeroable)]
pub struct UrlStateRaw {
    /// SHA-256 content hash.
    pub content_hash: [u8; 32],
    /// HTTP response status code (e.g., 200, 404).
    pub http_status: u16,
    /// Explicit padding to align `content_length` to 8-byte boundary.
    /// After `http_status` (2 bytes at offset 32), `_pad1` (6 bytes at offset 34)
    /// brings us to offset 40, which is 8-byte aligned.
    pub _pad1: [u8; 6],
    /// Content-Length from HTTP response.
    pub content_length: u64,
    /// Last fetch time in milliseconds since Unix epoch.
    pub last_fetched_ms: u64,
    /// Schema version (currently 1).
    pub version: u8,
    /// Status discriminant (`UrlStateStatus`).
    pub status: u8,
    /// Reserved bytes for future schema evolution. MUST be zero.
    pub reserved: [u8; 54],
}

impl UrlStateRaw {
    /// Create a new `UrlStateRaw` with the given fields.
    ///
    /// Sets `version` to `1`, zeroes `_pad1` and `reserved`.
    #[must_use]
    pub fn new(
        content_hash: [u8; 32],
        http_status: u16,
        content_length: u64,
        last_fetched_ms: u64,
        status: UrlStateStatus,
    ) -> Self {
        Self {
            content_hash,
            http_status,
            _pad1: [0u8; 6],
            content_length,
            last_fetched_ms,
            version: SCHEMA_VERSION,
            status: status as u8,
            reserved: [0u8; 54],
        }
    }

    /// Create a zeroed `UrlStateRaw` (all bytes zero).
    ///
    /// Note: `version` will be 0, so this is NOT a valid domain instance.
    /// Use `new()` for domain-valid instances.
    #[must_use]
    pub fn zeroed() -> Self {
        bytemuck::Zeroable::zeroed()
    }

    /// Parse a `UrlStateRaw` from a byte slice, validating all invariants.
    ///
    /// Validation order: size → version → status → pad/reserved bytes (fail-fast).
    pub fn from_bytes_checked(bytes: &[u8]) -> Result<Self, PodStateError> {
        if bytes.len() != URL_STATE_RAW_SIZE {
            return Err(PodStateError::WrongByteSize {
                type_name: "UrlStateRaw",
                actual: bytes.len(),
                expected: URL_STATE_RAW_SIZE,
            });
        }

        let candidate: Self = *bytemuck::from_bytes::<Self>(bytes);

        candidate.validate()?;

        Ok(candidate)
    }

    /// Parse a `UrlStateRaw` from a byte slice WITHOUT validation.
    ///
    /// Returns a zero-copy reference into the original bytes.
    ///
    /// # Safety (via bytemuck)
    ///
    /// - `bytes.len()` MUST equal `size_of::<UrlStateRaw>()`.
    /// - Caller guarantees the bytes represent a valid instance.
    #[must_use]
    pub fn from_bytes_unchecked(bytes: &[u8]) -> &Self {
        bytemuck::from_bytes(bytes)
    }

    /// Borrow this instance as a byte slice (Pod cast).
    #[must_use]
    pub fn as_bytes(&self) -> &[u8] {
        bytemuck::bytes_of(self)
    }

    /// Get the status as a typed enum.
    pub fn status(&self) -> Result<UrlStateStatus, PodStateError> {
        UrlStateStatus::from_discriminant(self.status)
    }

    /// Validate all invariants of this instance.
    pub fn validate(&self) -> Result<(), PodStateError> {
        if self.version != SCHEMA_VERSION {
            return Err(PodStateError::VersionMismatch {
                type_name: "UrlStateRaw",
                actual: self.version,
                expected: SCHEMA_VERSION,
            });
        }

        if UrlStateStatus::from_discriminant(self.status).is_err() {
            return Err(PodStateError::InvalidUrlStatus(self.status));
        }

        validate_reserved_zero("UrlStateRaw", URL_PAD1_OFFSET, &self._pad1)?;

        validate_reserved_zero("UrlStateRaw", URL_RESERVED_OFFSET, &self.reserved)
    }
}

// ---------------------------------------------------------------------------
// Shared validation helper (pure Calculation)
// ---------------------------------------------------------------------------

/// Validate that all bytes in `region` are zero.
///
/// Returns the offset of the first non-zero byte as an error.
fn validate_reserved_zero(
    type_name: &'static str,
    base_offset: usize,
    region: &[u8],
) -> Result<(), PodStateError> {
    region
        .iter()
        .enumerate()
        .find(|(_, &b)| b != 0)
        .map_or(Ok(()), |(i, _)| {
            Err(PodStateError::ReservedBytesNonZero {
                type_name,
                offset: base_offset + i,
            })
        })
}

// ---------------------------------------------------------------------------
// Tests
// ---------------------------------------------------------------------------

#[cfg(test)]
mod tests {
    use super::*;
    use std::mem::{align_of, size_of};

    // =======================================================================
    // Size & Layout (B01–B06)
    // =======================================================================

    #[test]
    fn file_state_raw_has_size_104_bytes() {
        assert_eq!(size_of::<FileStateRaw>(), FILE_STATE_RAW_SIZE);
    }

    #[test]
    fn file_state_raw_has_alignment_8() {
        assert_eq!(align_of::<FileStateRaw>(), 8);
    }

    #[test]
    fn url_state_raw_has_size_112_bytes() {
        assert_eq!(size_of::<UrlStateRaw>(), URL_STATE_RAW_SIZE);
    }

    #[test]
    fn url_state_raw_has_alignment_8() {
        assert_eq!(align_of::<UrlStateRaw>(), 8);
    }

    #[test]
    fn file_state_raw_has_no_compiler_inserted_padding() {
        let expected: usize = 32 + 8 + 8 + 1 + 1 + 54;
        assert_eq!(size_of::<FileStateRaw>(), expected);
    }

    #[test]
    fn url_state_raw_has_no_compiler_inserted_padding() {
        let expected: usize = 32 + 2 + 6 + 8 + 8 + 1 + 1 + 54;
        assert_eq!(size_of::<UrlStateRaw>(), expected);
    }

    // =======================================================================
    // FileStateRaw Construction (B07–B12)
    // =======================================================================

    #[test]
    fn file_state_raw_new_returns_correct_field_values() {
        let hash = [0xAB_u8; 32];
        let raw = FileStateRaw::new(hash, 4096, 1_700_000_000_000, FileStateStatus::Modified);

        assert_eq!(raw.content_hash, hash);
        assert_eq!(raw.file_size, 4096);
        assert_eq!(raw.last_modified_ms, 1_700_000_000_000);
        assert_eq!(raw.status, FileStateStatus::Modified as u8);
        assert_eq!(raw.version, 1);
        assert_eq!(raw.reserved, [0u8; 54]);
    }

    #[test]
    fn file_state_raw_new_sets_version_to_1() {
        let raw = FileStateRaw::new([0u8; 32], 0, 0, FileStateStatus::Unknown);
        assert_eq!(raw.version, 1);
    }

    #[test]
    fn file_state_raw_new_zeroes_reserved_bytes() {
        let raw = FileStateRaw::new([0xFF; 32], u64::MAX, u64::MAX, FileStateStatus::Deleted);
        assert_eq!(raw.reserved, [0u8; 54]);
    }

    #[test]
    fn file_state_raw_zeroed_returns_all_zeros() {
        let raw = FileStateRaw::zeroed();
        let bytes = raw.as_bytes();
        assert!(bytes.iter().all(|&b| b == 0));
        assert_eq!(bytes.len(), FILE_STATE_RAW_SIZE);
    }

    #[test]
    fn file_state_raw_compiles_with_bytemuck_bytes_of() {
        let raw = FileStateRaw::new([0u8; 32], 0, 0, FileStateStatus::Unknown);
        let _bytes: &[u8] = bytemuck::bytes_of(&raw);
    }

    #[test]
    fn file_state_raw_compiles_with_bytemuck_zeroed() {
        let _raw: FileStateRaw = bytemuck::Zeroable::zeroed();
    }

    // =======================================================================
    // UrlStateRaw Construction (B13–B18)
    // =======================================================================

    #[test]
    fn url_state_raw_new_returns_correct_field_values() {
        let hash = [0xCD_u8; 32];
        let raw = UrlStateRaw::new(hash, 200, 8192, 1_700_000_000_000, UrlStateStatus::Fresh);

        assert_eq!(raw.content_hash, hash);
        assert_eq!(raw.http_status, 200);
        assert_eq!(raw.content_length, 8192);
        assert_eq!(raw.last_fetched_ms, 1_700_000_000_000);
        assert_eq!(raw.status, UrlStateStatus::Fresh as u8);
        assert_eq!(raw.version, 1);
        assert_eq!(raw._pad1, [0u8; 6]);
        assert_eq!(raw.reserved, [0u8; 54]);
    }

    #[test]
    fn url_state_raw_new_sets_version_to_1() {
        let raw = UrlStateRaw::new([0u8; 32], 0, 0, 0, UrlStateStatus::Unknown);
        assert_eq!(raw.version, 1);
    }

    #[test]
    fn url_state_raw_new_zeroes_pad_and_reserved_bytes() {
        let raw = UrlStateRaw::new(
            [0xFF; 32],
            u16::MAX,
            u64::MAX,
            u64::MAX,
            UrlStateStatus::Error,
        );
        assert_eq!(raw._pad1, [0u8; 6]);
        assert_eq!(raw.reserved, [0u8; 54]);
    }

    #[test]
    fn url_state_raw_zeroed_returns_all_zeros() {
        let raw = UrlStateRaw::zeroed();
        let bytes = raw.as_bytes();
        assert!(bytes.iter().all(|&b| b == 0));
        assert_eq!(bytes.len(), URL_STATE_RAW_SIZE);
    }

    #[test]
    fn url_state_raw_compiles_with_bytemuck_bytes_of() {
        let raw = UrlStateRaw::new([0u8; 32], 0, 0, 0, UrlStateStatus::Unknown);
        let _bytes: &[u8] = bytemuck::bytes_of(&raw);
    }

    #[test]
    fn url_state_raw_compiles_with_bytemuck_zeroed() {
        let _raw: UrlStateRaw = bytemuck::Zeroable::zeroed();
    }

    // =======================================================================
    // Byte Round-Trip (B19–B24)
    // =======================================================================

    #[test]
    fn file_state_raw_round_trips_through_bytes_losslessly() {
        let raw = FileStateRaw::new([1u8; 32], 999, 12345, FileStateStatus::Unchanged);
        let bytes = raw.as_bytes();
        let restored = FileStateRaw::from_bytes_checked(bytes).unwrap();

        assert_eq!(restored, raw);
        assert_eq!(restored.content_hash, [1u8; 32]);
        assert_eq!(restored.file_size, 999);
        assert_eq!(restored.last_modified_ms, 12345);
        assert_eq!(restored.status, FileStateStatus::Unchanged as u8);
        assert_eq!(restored.version, 1);
    }

    #[test]
    fn url_state_raw_round_trips_through_bytes_losslessly() {
        let raw = UrlStateRaw::new([2u8; 32], 404, 0, 99999, UrlStateStatus::Stale);
        let bytes = raw.as_bytes();
        let restored = UrlStateRaw::from_bytes_checked(bytes).unwrap();

        assert_eq!(restored, raw);
        assert_eq!(restored.http_status, 404);
        assert_eq!(restored.content_length, 0);
        assert_eq!(restored.last_fetched_ms, 99999);
        assert_eq!(restored.status, UrlStateStatus::Stale as u8);
    }

    #[test]
    fn file_state_raw_as_bytes_returns_104_byte_slice() {
        let raw = FileStateRaw::new([0u8; 32], 0, 0, FileStateStatus::Unknown);
        assert_eq!(raw.as_bytes().len(), FILE_STATE_RAW_SIZE);
    }

    #[test]
    fn url_state_raw_as_bytes_returns_112_byte_slice() {
        let raw = UrlStateRaw::new([0u8; 32], 0, 0, 0, UrlStateStatus::Unknown);
        assert_eq!(raw.as_bytes().len(), URL_STATE_RAW_SIZE);
    }

    #[test]
    fn file_state_raw_equal_instances_have_identical_bytes() {
        let a = FileStateRaw::new([0x42u8; 32], 100, 200, FileStateStatus::Modified);
        let b = FileStateRaw::new([0x42u8; 32], 100, 200, FileStateStatus::Modified);
        assert_eq!(a, b);
        assert_eq!(a.as_bytes(), b.as_bytes());
    }

    #[test]
    fn url_state_raw_equal_instances_have_identical_bytes() {
        let a = UrlStateRaw::new([0x42u8; 32], 200, 100, 200, UrlStateStatus::Fresh);
        let b = UrlStateRaw::new([0x42u8; 32], 200, 100, 200, UrlStateStatus::Fresh);
        assert_eq!(a, b);
        assert_eq!(a.as_bytes(), b.as_bytes());
    }

    // =======================================================================
    // FileStateRaw from_bytes_checked — Error Paths (B25–B28)
    // =======================================================================

    #[test]
    fn file_state_raw_from_bytes_checked_rejects_wrong_size() {
        let result = FileStateRaw::from_bytes_checked(&[0u8; 50]);
        assert!(matches!(
            result,
            Err(PodStateError::WrongByteSize {
                type_name: "FileStateRaw",
                actual: 50,
                expected: 104,
            })
        ));
    }

    #[test]
    fn file_state_raw_from_bytes_checked_rejects_empty_slice() {
        let result = FileStateRaw::from_bytes_checked(&[]);
        assert!(matches!(
            result,
            Err(PodStateError::WrongByteSize {
                type_name: "FileStateRaw",
                actual: 0,
                expected: 104,
            })
        ));
    }

    #[test]
    fn file_state_raw_from_bytes_checked_rejects_one_byte_short() {
        let result = FileStateRaw::from_bytes_checked(&[0u8; 103]);
        assert!(matches!(
            result,
            Err(PodStateError::WrongByteSize {
                type_name: "FileStateRaw",
                actual: 103,
                expected: 104,
            })
        ));
    }

    #[test]
    fn file_state_raw_from_bytes_checked_rejects_one_byte_over() {
        let result = FileStateRaw::from_bytes_checked(&[0u8; 105]);
        assert!(matches!(
            result,
            Err(PodStateError::WrongByteSize {
                type_name: "FileStateRaw",
                actual: 105,
                expected: 104,
            })
        ));
    }

    #[test]
    fn file_state_raw_from_bytes_checked_rejects_invalid_status_byte_4() {
        let mut bytes = make_valid_file_state_bytes();
        bytes[FILE_STATUS_OFFSET] = 4;
        let result = FileStateRaw::from_bytes_checked(&bytes);
        assert!(matches!(result, Err(PodStateError::InvalidFileStatus(4))));
    }

    #[test]
    fn file_state_raw_from_bytes_checked_rejects_invalid_status_byte_255() {
        let mut bytes = make_valid_file_state_bytes();
        bytes[FILE_STATUS_OFFSET] = 255;
        let result = FileStateRaw::from_bytes_checked(&bytes);
        assert!(matches!(result, Err(PodStateError::InvalidFileStatus(255))));
    }

    #[test]
    fn file_state_raw_from_bytes_checked_rejects_invalid_status_byte_128() {
        let mut bytes = make_valid_file_state_bytes();
        bytes[FILE_STATUS_OFFSET] = 128;
        let result = FileStateRaw::from_bytes_checked(&bytes);
        assert!(matches!(result, Err(PodStateError::InvalidFileStatus(128))));
    }

    #[test]
    fn file_state_raw_from_bytes_checked_rejects_version_2() {
        let mut bytes = make_valid_file_state_bytes();
        bytes[FILE_VERSION_OFFSET] = 2;
        let result = FileStateRaw::from_bytes_checked(&bytes);
        assert!(matches!(
            result,
            Err(PodStateError::VersionMismatch {
                type_name: "FileStateRaw",
                actual: 2,
                expected: 1,
            })
        ));
    }

    #[test]
    fn file_state_raw_from_bytes_checked_rejects_version_0() {
        let mut bytes = make_valid_file_state_bytes();
        bytes[FILE_VERSION_OFFSET] = 0;
        let result = FileStateRaw::from_bytes_checked(&bytes);
        assert!(matches!(
            result,
            Err(PodStateError::VersionMismatch {
                type_name: "FileStateRaw",
                actual: 0,
                expected: 1,
            })
        ));
    }

    #[test]
    fn file_state_raw_from_bytes_checked_rejects_version_255() {
        let mut bytes = make_valid_file_state_bytes();
        bytes[FILE_VERSION_OFFSET] = 255;
        let result = FileStateRaw::from_bytes_checked(&bytes);
        assert!(matches!(
            result,
            Err(PodStateError::VersionMismatch {
                type_name: "FileStateRaw",
                actual: 255,
                expected: 1,
            })
        ));
    }

    #[test]
    fn file_state_raw_from_bytes_checked_rejects_nonzero_reserved_bytes() {
        let mut bytes = make_valid_file_state_bytes();
        bytes[FILE_RESERVED_OFFSET] = 0x01;
        let result = FileStateRaw::from_bytes_checked(&bytes);
        assert!(matches!(
            result,
            Err(PodStateError::ReservedBytesNonZero {
                type_name: "FileStateRaw",
                offset: 50,
            })
        ));
    }

    #[test]
    fn file_state_raw_from_bytes_checked_rejects_nonzero_last_reserved_byte() {
        let mut bytes = make_valid_file_state_bytes();
        let last_offset = FILE_STATE_RAW_SIZE - 1;
        bytes[last_offset] = 0x01;
        let result = FileStateRaw::from_bytes_checked(&bytes);
        let expected_offset = FILE_STATE_RAW_SIZE - 1;
        assert!(matches!(
            result,
            Err(PodStateError::ReservedBytesNonZero {
                type_name: "FileStateRaw",
                offset,
            }) if offset == expected_offset
        ));
    }

    #[test]
    fn file_state_raw_from_bytes_checked_rejects_all_ff_reserved_bytes() {
        let mut bytes = make_valid_file_state_bytes();
        bytes[FILE_RESERVED_OFFSET..FILE_STATE_RAW_SIZE].fill(0xFF);
        let result = FileStateRaw::from_bytes_checked(&bytes);
        assert!(matches!(
            result,
            Err(PodStateError::ReservedBytesNonZero {
                type_name: "FileStateRaw",
                offset: FILE_RESERVED_OFFSET,
            })
        ));
    }

    // =======================================================================
    // UrlStateRaw from_bytes_checked — Error Paths (B29–B32)
    // =======================================================================

    #[test]
    fn url_state_raw_from_bytes_checked_rejects_wrong_size() {
        let result = UrlStateRaw::from_bytes_checked(&[0u8; 50]);
        assert!(matches!(
            result,
            Err(PodStateError::WrongByteSize {
                type_name: "UrlStateRaw",
                actual: 50,
                expected: 112,
            })
        ));
    }

    #[test]
    fn url_state_raw_from_bytes_checked_rejects_empty_slice() {
        let result = UrlStateRaw::from_bytes_checked(&[]);
        assert!(matches!(
            result,
            Err(PodStateError::WrongByteSize {
                type_name: "UrlStateRaw",
                actual: 0,
                expected: 112,
            })
        ));
    }

    #[test]
    fn url_state_raw_from_bytes_checked_rejects_one_byte_short() {
        let result = UrlStateRaw::from_bytes_checked(&[0u8; 111]);
        assert!(matches!(
            result,
            Err(PodStateError::WrongByteSize {
                type_name: "UrlStateRaw",
                actual: 111,
                expected: 112,
            })
        ));
    }

    #[test]
    fn url_state_raw_from_bytes_checked_rejects_one_byte_over() {
        let result = UrlStateRaw::from_bytes_checked(&[0u8; 113]);
        assert!(matches!(
            result,
            Err(PodStateError::WrongByteSize {
                type_name: "UrlStateRaw",
                actual: 113,
                expected: 112,
            })
        ));
    }

    #[test]
    fn url_state_raw_from_bytes_checked_rejects_invalid_status_byte_5() {
        let mut bytes = make_valid_url_state_bytes();
        bytes[URL_STATUS_OFFSET] = 5;
        let result = UrlStateRaw::from_bytes_checked(&bytes);
        assert!(matches!(result, Err(PodStateError::InvalidUrlStatus(5))));
    }

    #[test]
    fn url_state_raw_from_bytes_checked_rejects_invalid_status_byte_255() {
        let mut bytes = make_valid_url_state_bytes();
        bytes[URL_STATUS_OFFSET] = 255;
        let result = UrlStateRaw::from_bytes_checked(&bytes);
        assert!(matches!(result, Err(PodStateError::InvalidUrlStatus(255))));
    }

    #[test]
    fn url_state_raw_from_bytes_checked_rejects_invalid_status_byte_200() {
        let mut bytes = make_valid_url_state_bytes();
        bytes[URL_STATUS_OFFSET] = 200;
        let result = UrlStateRaw::from_bytes_checked(&bytes);
        assert!(matches!(result, Err(PodStateError::InvalidUrlStatus(200))));
    }

    #[test]
    fn url_state_raw_from_bytes_checked_rejects_version_99() {
        let mut bytes = make_valid_url_state_bytes();
        bytes[URL_VERSION_OFFSET] = 99;
        let result = UrlStateRaw::from_bytes_checked(&bytes);
        assert!(matches!(
            result,
            Err(PodStateError::VersionMismatch {
                type_name: "UrlStateRaw",
                actual: 99,
                expected: 1,
            })
        ));
    }

    #[test]
    fn url_state_raw_from_bytes_checked_rejects_version_0() {
        let mut bytes = make_valid_url_state_bytes();
        bytes[URL_VERSION_OFFSET] = 0;
        let result = UrlStateRaw::from_bytes_checked(&bytes);
        assert!(matches!(
            result,
            Err(PodStateError::VersionMismatch {
                type_name: "UrlStateRaw",
                actual: 0,
                expected: 1,
            })
        ));
    }

    #[test]
    fn url_state_raw_from_bytes_checked_rejects_nonzero_pad1_bytes() {
        let mut bytes = make_valid_url_state_bytes();
        bytes[URL_PAD1_OFFSET] = 0xFF;
        bytes[URL_PAD1_OFFSET + 1] = 0xFF;
        let result = UrlStateRaw::from_bytes_checked(&bytes);
        assert!(matches!(
            result,
            Err(PodStateError::ReservedBytesNonZero {
                type_name: "UrlStateRaw",
                offset: URL_PAD1_OFFSET,
            })
        ));
    }

    #[test]
    fn url_state_raw_from_bytes_checked_rejects_nonzero_reserved_bytes() {
        let mut bytes = make_valid_url_state_bytes();
        bytes[URL_RESERVED_OFFSET] = 0x42;
        let result = UrlStateRaw::from_bytes_checked(&bytes);
        assert!(matches!(
            result,
            Err(PodStateError::ReservedBytesNonZero {
                type_name: "UrlStateRaw",
                offset: URL_RESERVED_OFFSET,
            })
        ));
    }

    // =======================================================================
    // Status Enum Discriminants (B33–B36)
    // =======================================================================

    #[test]
    fn file_state_status_from_discriminant_0_returns_unknown() {
        assert_eq!(
            FileStateStatus::from_discriminant(0).unwrap(),
            FileStateStatus::Unknown
        );
    }

    #[test]
    fn file_state_status_from_discriminant_1_returns_unchanged() {
        assert_eq!(
            FileStateStatus::from_discriminant(1).unwrap(),
            FileStateStatus::Unchanged
        );
    }

    #[test]
    fn file_state_status_from_discriminant_2_returns_modified() {
        assert_eq!(
            FileStateStatus::from_discriminant(2).unwrap(),
            FileStateStatus::Modified
        );
    }

    #[test]
    fn file_state_status_from_discriminant_3_returns_deleted() {
        assert_eq!(
            FileStateStatus::from_discriminant(3).unwrap(),
            FileStateStatus::Deleted
        );
    }

    #[test]
    fn file_state_status_from_discriminant_4_returns_invalid_file_status() {
        assert!(matches!(
            FileStateStatus::from_discriminant(4),
            Err(PodStateError::InvalidFileStatus(4))
        ));
    }

    #[test]
    fn file_state_status_from_discriminant_255_returns_invalid_file_status() {
        assert!(matches!(
            FileStateStatus::from_discriminant(255),
            Err(PodStateError::InvalidFileStatus(255))
        ));
    }

    #[test]
    fn url_state_status_from_discriminant_0_returns_unknown() {
        assert_eq!(
            UrlStateStatus::from_discriminant(0).unwrap(),
            UrlStateStatus::Unknown
        );
    }

    #[test]
    fn url_state_status_from_discriminant_1_returns_fresh() {
        assert_eq!(
            UrlStateStatus::from_discriminant(1).unwrap(),
            UrlStateStatus::Fresh
        );
    }

    #[test]
    fn url_state_status_from_discriminant_2_returns_stale() {
        assert_eq!(
            UrlStateStatus::from_discriminant(2).unwrap(),
            UrlStateStatus::Stale
        );
    }

    #[test]
    fn url_state_status_from_discriminant_3_returns_error() {
        assert_eq!(
            UrlStateStatus::from_discriminant(3).unwrap(),
            UrlStateStatus::Error
        );
    }

    #[test]
    fn url_state_status_from_discriminant_4_returns_invalid_url_status() {
        assert!(matches!(
            UrlStateStatus::from_discriminant(4),
            Err(PodStateError::InvalidUrlStatus(4))
        ));
    }

    #[test]
    fn url_state_status_from_discriminant_255_returns_invalid_url_status() {
        assert!(matches!(
            UrlStateStatus::from_discriminant(255),
            Err(PodStateError::InvalidUrlStatus(255))
        ));
    }

    #[test]
    fn file_state_status_discriminants_match_repr_u8_values() {
        assert_eq!(FileStateStatus::Unknown as u8, 0);
        assert_eq!(FileStateStatus::Unchanged as u8, 1);
        assert_eq!(FileStateStatus::Modified as u8, 2);
        assert_eq!(FileStateStatus::Deleted as u8, 3);
    }

    #[test]
    fn url_state_status_discriminants_match_repr_u8_values() {
        assert_eq!(UrlStateStatus::Unknown as u8, 0);
        assert_eq!(UrlStateStatus::Fresh as u8, 1);
        assert_eq!(UrlStateStatus::Stale as u8, 2);
        assert_eq!(UrlStateStatus::Error as u8, 3);
    }

    // =======================================================================
    // Validate Method (B37–B38 + extended)
    // =======================================================================

    #[test]
    fn file_state_raw_validate_accepts_well_constructed_instance() {
        let raw = FileStateRaw::new([0u8; 32], 0, 0, FileStateStatus::Unknown);
        assert!(raw.validate().is_ok());
    }

    #[test]
    fn file_state_raw_validate_rejects_corrupted_status() {
        let mut raw = FileStateRaw::new([0u8; 32], 0, 0, FileStateStatus::Unknown);
        raw.status = 99;
        assert!(matches!(
            raw.validate(),
            Err(PodStateError::InvalidFileStatus(99))
        ));
    }

    #[test]
    fn file_state_raw_validate_rejects_wrong_version() {
        let mut raw = FileStateRaw::new([0u8; 32], 0, 0, FileStateStatus::Unknown);
        raw.version = 0;
        assert!(matches!(
            raw.validate(),
            Err(PodStateError::VersionMismatch {
                type_name: "FileStateRaw",
                actual: 0,
                expected: 1,
            })
        ));
    }

    #[test]
    fn file_state_raw_validate_rejects_nonzero_reserved() {
        let mut raw = FileStateRaw::new([0u8; 32], 0, 0, FileStateStatus::Unknown);
        raw.reserved[0] = 1;
        assert!(matches!(
            raw.validate(),
            Err(PodStateError::ReservedBytesNonZero {
                type_name: "FileStateRaw",
                offset: FILE_RESERVED_OFFSET,
            })
        ));
    }

    #[test]
    fn url_state_raw_validate_accepts_well_constructed_instance() {
        let raw = UrlStateRaw::new([0u8; 32], 0, 0, 0, UrlStateStatus::Unknown);
        assert!(raw.validate().is_ok());
    }

    #[test]
    fn url_state_raw_validate_rejects_corrupted_status() {
        let mut raw = UrlStateRaw::new([0u8; 32], 0, 0, 0, UrlStateStatus::Unknown);
        raw.status = 99;
        assert!(matches!(
            raw.validate(),
            Err(PodStateError::InvalidUrlStatus(99))
        ));
    }

    #[test]
    fn url_state_raw_validate_rejects_nonzero_pad1() {
        let mut raw = UrlStateRaw::new([0u8; 32], 0, 0, 0, UrlStateStatus::Unknown);
        raw._pad1 = [0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF];
        assert!(matches!(
            raw.validate(),
            Err(PodStateError::ReservedBytesNonZero {
                type_name: "UrlStateRaw",
                offset: URL_PAD1_OFFSET,
            })
        ));
    }

    // =======================================================================
    // status() Accessor Method
    // =======================================================================

    #[test]
    fn file_state_raw_status_returns_modified_for_discriminant_2() {
        let raw = FileStateRaw::new([0u8; 32], 0, 0, FileStateStatus::Modified);
        assert_eq!(raw.status().unwrap(), FileStateStatus::Modified);
    }

    #[test]
    fn file_state_raw_status_returns_error_for_invalid_discriminant() {
        let mut raw = FileStateRaw::new([0u8; 32], 0, 0, FileStateStatus::Unknown);
        raw.status = 200;
        assert!(matches!(
            raw.status(),
            Err(PodStateError::InvalidFileStatus(200))
        ));
    }

    #[test]
    fn url_state_raw_status_returns_error_variant_for_discriminant_3() {
        let raw = UrlStateRaw::new([0u8; 32], 0, 0, 0, UrlStateStatus::Error);
        assert_eq!(raw.status().unwrap(), UrlStateStatus::Error);
    }

    #[test]
    fn url_state_raw_status_returns_error_for_invalid_discriminant() {
        let mut raw = UrlStateRaw::new([0u8; 32], 0, 0, 0, UrlStateStatus::Unknown);
        raw.status = 200;
        assert!(matches!(
            raw.status(),
            Err(PodStateError::InvalidUrlStatus(200))
        ));
    }

    // =======================================================================
    // All-Zero and All-FF Edge Cases
    // =======================================================================

    #[test]
    fn file_state_raw_all_zeros_rejected_due_to_version_0() {
        let zeros = [0u8; FILE_STATE_RAW_SIZE];
        let result = FileStateRaw::from_bytes_checked(&zeros);
        assert!(matches!(
            result,
            Err(PodStateError::VersionMismatch {
                type_name: "FileStateRaw",
                actual: 0,
                expected: 1,
            })
        ));
    }

    #[test]
    fn file_state_raw_all_ff_bytes_rejected() {
        let all_ff = [0xFF_u8; FILE_STATE_RAW_SIZE];
        let result = FileStateRaw::from_bytes_checked(&all_ff);
        // Version check runs first: version=255 triggers VersionMismatch
        assert!(result.is_err());
    }

    #[test]
    fn url_state_raw_all_zeros_rejected_due_to_version_0() {
        let zeros = [0u8; URL_STATE_RAW_SIZE];
        let result = UrlStateRaw::from_bytes_checked(&zeros);
        assert!(matches!(
            result,
            Err(PodStateError::VersionMismatch {
                type_name: "UrlStateRaw",
                actual: 0,
                expected: 1,
            })
        ));
    }

    #[test]
    fn url_state_raw_all_ff_bytes_rejected() {
        let all_ff = [0xFF_u8; URL_STATE_RAW_SIZE];
        let result = UrlStateRaw::from_bytes_checked(&all_ff);
        assert!(result.is_err());
    }

    // =======================================================================
    // Error Display Messages
    // =======================================================================

    #[test]
    fn pod_state_error_invalid_file_status_displays_correct_message() {
        let err = PodStateError::InvalidFileStatus(42);
        let msg = err.to_string();
        assert!(msg.contains("42"), "expected '42' in: {msg}");
        assert!(
            msg.contains("valid: 0-3"),
            "expected 'valid: 0-3' in: {msg}"
        );
    }

    #[test]
    fn pod_state_error_invalid_url_status_displays_correct_message() {
        let err = PodStateError::InvalidUrlStatus(99);
        let msg = err.to_string();
        assert!(msg.contains("99"), "expected '99' in: {msg}");
        assert!(
            msg.contains("valid: 0-3"),
            "expected 'valid: 0-3' in: {msg}"
        );
    }

    #[test]
    fn pod_state_error_wrong_byte_size_displays_correct_message() {
        let err = PodStateError::WrongByteSize {
            type_name: "FileStateRaw",
            actual: 50,
            expected: 104,
        };
        let msg = err.to_string();
        assert!(
            msg.contains("FileStateRaw"),
            "expected 'FileStateRaw' in: {msg}"
        );
        assert!(msg.contains("50"), "expected '50' in: {msg}");
        assert!(msg.contains("104"), "expected '104' in: {msg}");
    }

    #[test]
    fn pod_state_error_version_mismatch_displays_correct_message() {
        let err = PodStateError::VersionMismatch {
            type_name: "UrlStateRaw",
            actual: 5,
            expected: 1,
        };
        let msg = err.to_string();
        assert!(
            msg.contains("UrlStateRaw"),
            "expected 'UrlStateRaw' in: {msg}"
        );
        assert!(msg.contains("5"), "expected '5' in: {msg}");
        assert!(msg.contains("1"), "expected '1' in: {msg}");
    }

    #[test]
    fn pod_state_error_reserved_bytes_nonzero_displays_correct_message() {
        let err = PodStateError::ReservedBytesNonZero {
            type_name: "FileStateRaw",
            offset: FILE_RESERVED_OFFSET,
        };
        let msg = err.to_string();
        assert!(
            msg.contains("FileStateRaw"),
            "expected 'FileStateRaw' in: {msg}"
        );
        assert!(msg.contains("50"), "expected '50' in: {msg}");
    }

    // =======================================================================
    // Trait Proofs (Copy + Send + Sync)
    // =======================================================================

    #[test]
    fn file_state_raw_is_copy_send_sync() {
        fn assert_copy_send_sync<T: Copy + Send + Sync>() {}
        assert_copy_send_sync::<FileStateRaw>();
    }

    #[test]
    fn url_state_raw_is_copy_send_sync() {
        fn assert_copy_send_sync<T: Copy + Send + Sync>() {}
        assert_copy_send_sync::<UrlStateRaw>();
    }

    // =======================================================================
    // Proptests
    // =======================================================================

    #[cfg(test)]
    mod proptests {
        use super::*;
        use proptest::prelude::*;

        proptest! {
            #[test]
            fn file_state_raw_round_trip(
                hash in any::<[u8; 32]>(),
                file_size in any::<u64>(),
                last_modified in any::<u64>(),
                status_disc in 0u8..=3u8,
            ) {
                let status = FileStateStatus::from_discriminant(status_disc).unwrap();
                let raw = FileStateRaw::new(hash, file_size, last_modified, status);
                let bytes = raw.as_bytes();
                let restored = FileStateRaw::from_bytes_checked(bytes).unwrap();
                prop_assert_eq!(restored, raw);
            }

            #[test]
            fn url_state_raw_round_trip(
                hash in any::<[u8; 32]>(),
                http_status in any::<u16>(),
                content_length in any::<u64>(),
                last_fetched in any::<u64>(),
                status_disc in 0u8..=3u8,
            ) {
                let status = UrlStateStatus::from_discriminant(status_disc).unwrap();
                let raw = UrlStateRaw::new(hash, http_status, content_length, last_fetched, status);
                let bytes = raw.as_bytes();
                let restored = UrlStateRaw::from_bytes_checked(bytes).unwrap();
                prop_assert_eq!(restored, raw);
            }

            #[test]
            fn file_state_raw_byte_length_invariant(
                hash in any::<[u8; 32]>(),
                file_size in any::<u64>(),
                last_modified in any::<u64>(),
                status_disc in 0u8..=3u8,
            ) {
                let status = FileStateStatus::from_discriminant(status_disc).unwrap();
                let raw = FileStateRaw::new(hash, file_size, last_modified, status);
                prop_assert_eq!(raw.as_bytes().len(), 104);
            }

            #[test]
            fn url_state_raw_byte_length_invariant(
                hash in any::<[u8; 32]>(),
                http_status in any::<u16>(),
                content_length in any::<u64>(),
                last_fetched in any::<u64>(),
                status_disc in 0u8..=3u8,
            ) {
                let status = UrlStateStatus::from_discriminant(status_disc).unwrap();
                let raw = UrlStateRaw::new(hash, http_status, content_length, last_fetched, status);
                prop_assert_eq!(raw.as_bytes().len(), 112);
            }

            #[test]
            fn file_state_status_from_discriminant_validity(disc in any::<u8>()) {
                let result = FileStateStatus::from_discriminant(disc);
                match disc {
                    0..=3 => prop_assert!(result.is_ok()),
                    _ => prop_assert!(result.is_err()),
                }
            }

            #[test]
            fn url_state_status_from_discriminant_validity(disc in any::<u8>()) {
                let result = UrlStateStatus::from_discriminant(disc);
                match disc {
                    0..=3 => prop_assert!(result.is_ok()),
                    _ => prop_assert!(result.is_err()),
                }
            }

            #[test]
            fn file_state_raw_validate_catches_corrupted_status(
                status_byte in 4u8..=255u8,
            ) {
                let raw = FileStateRaw::new([0u8; 32], 0, 0, FileStateStatus::Unknown);
                let mut corrupted = raw;
                corrupted.status = status_byte;
                prop_assert!(corrupted.validate().is_err());
            }

            #[test]
            fn url_state_raw_validate_catches_corrupted_status(
                status_byte in 4u8..=255u8,
            ) {
                let raw = UrlStateRaw::new([0u8; 32], 0, 0, 0, UrlStateStatus::Unknown);
                let mut corrupted = raw;
                corrupted.status = status_byte;
                prop_assert!(corrupted.validate().is_err());
            }
        }
    }

    // =======================================================================
    // Test helpers
    // =======================================================================

    /// Create a valid 104-byte array representing a `FileStateRaw`.
    fn make_valid_file_state_bytes() -> [u8; FILE_STATE_RAW_SIZE] {
        let raw = FileStateRaw::new([0u8; 32], 0, 0, FileStateStatus::Unknown);
        let src = raw.as_bytes();
        let mut arr = [0u8; FILE_STATE_RAW_SIZE];
        arr.copy_from_slice(src);
        arr
    }

    /// Create a valid 112-byte array representing a `UrlStateRaw`.
    fn make_valid_url_state_bytes() -> [u8; URL_STATE_RAW_SIZE] {
        let raw = UrlStateRaw::new([0u8; 32], 0, 0, 0, UrlStateStatus::Unknown);
        let src = raw.as_bytes();
        let mut arr = [0u8; URL_STATE_RAW_SIZE];
        arr.copy_from_slice(src);
        arr
    }
}

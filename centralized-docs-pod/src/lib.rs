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

mod file_state;
mod newtypes;
mod url_state;

pub use file_state::FileStateRaw;
pub use newtypes::{HttpStatus, Sha256Hash};
pub use url_state::UrlStateRaw;

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

#[cfg(test)]
mod tests;

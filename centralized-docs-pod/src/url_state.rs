//! Fixed-size Pod struct for cached URL state (112 bytes).

use bytemuck::{Pod, Zeroable};

use crate::validate_reserved_zero;
use crate::PodStateError;
use crate::SCHEMA_VERSION;
use crate::URL_PAD1_OFFSET;
use crate::URL_RESERVED_OFFSET;
use crate::URL_STATE_RAW_SIZE;
use crate::{HttpStatus, Sha256Hash, UrlStateStatus};

/// Fixed-size Pod struct for cached URL state.
///
/// # Layout (112 bytes total)
///
/// | Offset | Size | Field              | Type      |
/// |--------|------|--------------------|-----------|
/// | 0      | 32   | `content_hash`       | Sha256Hash|
/// | 32     | 2    | `http_status`        | HttpStatus|
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
    pub content_hash: Sha256Hash,
    /// HTTP response status code (e.g., 200, 404).
    pub http_status: HttpStatus,
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
            content_hash: Sha256Hash::from(content_hash),
            http_status: HttpStatus::from(http_status),
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
        debug_assert!(
            bytes.len() == std::mem::size_of::<Self>(),
            "from_bytes_unchecked: expected {} bytes, got {}",
            std::mem::size_of::<Self>(),
            bytes.len()
        );
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

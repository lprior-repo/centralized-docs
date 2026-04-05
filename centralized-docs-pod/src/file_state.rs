//! Fixed-size Pod struct for cached file state (104 bytes).

use bytemuck::{Pod, Zeroable};

use crate::validate_reserved_zero;
use crate::FILE_RESERVED_OFFSET;
use crate::FILE_STATE_RAW_SIZE;
use crate::SCHEMA_VERSION;
use crate::{FileStateStatus, PodStateError, Sha256Hash};

/// Fixed-size Pod struct for cached file state.
///
/// # Layout (104 bytes total)
///
/// | Offset | Size | Field              | Type      |
/// |--------|------|--------------------|-----------|
/// | 0      | 32   | content_hash       | Sha256Hash|
/// | 32     | 8    | file_size          | u64       |
/// | 40     | 8    | last_modified_ms   | u64       |
/// | 48     | 1    | version            | u8        |
/// | 49     | 1    | status             | u8        |
/// | 50     | 54   | reserved           | [u8; 54]  |
#[repr(C)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Pod, Zeroable)]
pub struct FileStateRaw {
    /// SHA-256 content hash.
    pub content_hash: Sha256Hash,
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
            content_hash: Sha256Hash::from(content_hash),
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

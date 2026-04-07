//! Pod type definitions for fixed-layout state storage.
//!
//! [`FileStateRaw`] (200 bytes) and [`UrlStateRaw`] (120 bytes) are `#[repr(C)]` fixed-size
//! structs designed for zero-copy reads from redb. Safe conversion via `from_bytes`/`to_bytes`.

#![deny(clippy::unwrap_used)]
#![deny(clippy::expect_used)]
#![deny(clippy::panic)]
#![warn(clippy::pedantic)]
#![forbid(unsafe_code)]

use super::error::StateError;
use super::tables::{TABLE_NAME_FILE_STATE, TABLE_NAME_URL_STATE};
use std::mem::size_of;

// Compile-time size assertions
const _: () = assert!(size_of::<FileStateRaw>() == 200);
const _: () = assert!(size_of::<UrlStateRaw>() == 120);

// ---------------------------------------------------------------------------
// Pod type: FileStateRaw (200 bytes)
// ---------------------------------------------------------------------------

/// Fixed-size file state. 200 bytes. Zero-copy read from redb.
///
/// # Layout
///
/// ```text
/// offset   size  field
/// 0        32    content_hash          SHA-256 of file bytes
/// 32       32    config_hash           SHA-256 of category config (or zeroed)
/// 64       32    analysis_hash         FK -> analysis_outputs key
/// 96       32    transform_hash        FK -> transform_outputs key
/// 128      32    chunk_hash            FK -> chunk_outputs key
/// 160       8    last_processed_secs   unix timestamp
/// 168      32    reserved              future-proof padding
/// ```
#[derive(Debug, Clone, Copy, PartialEq, Eq, rkyv::Archive, rkyv::Serialize, rkyv::Deserialize)]
#[repr(C)]
pub struct FileStateRaw {
    pub content_hash: [u8; 32],
    pub config_hash: [u8; 32],
    pub analysis_hash: [u8; 32],
    pub transform_hash: [u8; 32],
    pub chunk_hash: [u8; 32],
    pub last_processed_secs: u64,
    pub reserved: [u8; 32],
}

impl FileStateRaw {
    /// Fixed byte size.
    pub const SIZE: usize = 200;

    /// Create an all-zero instance.
    #[must_use]
    pub const fn zeroed() -> Self {
        Self {
            content_hash: [0u8; 32],
            config_hash: [0u8; 32],
            analysis_hash: [0u8; 32],
            transform_hash: [0u8; 32],
            chunk_hash: [0u8; 32],
            last_processed_secs: 0,
            reserved: [0u8; 32],
        }
    }

    /// Deserialize from exactly 200 raw bytes.
    ///
    /// # Errors
    ///
    /// Returns [`StateError::PodSizeMismatch`] or [`StateError::PodCastFailed`].
    pub fn from_bytes(bytes: &[u8]) -> Result<Self, StateError> {
        if bytes.len() != Self::SIZE {
            return Err(StateError::PodSizeMismatch {
                table: TABLE_NAME_FILE_STATE,
                expected: Self::SIZE,
                actual: bytes.len(),
            });
        }
        Ok(Self {
            content_hash: read_array::<32>(bytes, 0)?,
            config_hash: read_array::<32>(bytes, 32)?,
            analysis_hash: read_array::<32>(bytes, 64)?,
            transform_hash: read_array::<32>(bytes, 96)?,
            chunk_hash: read_array::<32>(bytes, 128)?,
            last_processed_secs: u64::from_le_bytes(read_array::<8>(bytes, 160)?),
            reserved: read_array::<32>(bytes, 168)?,
        })
    }

    /// Serialize to exactly 200 raw bytes.
    #[must_use]
    pub fn to_bytes(&self) -> [u8; Self::SIZE] {
        let mut r = [0u8; Self::SIZE];
        copy_into(&mut r, 0, &self.content_hash);
        copy_into(&mut r, 32, &self.config_hash);
        copy_into(&mut r, 64, &self.analysis_hash);
        copy_into(&mut r, 96, &self.transform_hash);
        copy_into(&mut r, 128, &self.chunk_hash);
        copy_into(&mut r, 160, &self.last_processed_secs.to_le_bytes());
        copy_into(&mut r, 168, &self.reserved);
        r
    }
}

// ---------------------------------------------------------------------------
// Pod type: UrlStateRaw (120 bytes)
// ---------------------------------------------------------------------------

/// Fixed-size URL state. 120 bytes. Zero-copy read from redb.
///
/// # Layout
///
/// ```text
/// offset   size  field
/// 0        32    content_hash          SHA-256 of scraped content
/// 32       32    url_hash              FK -> scrape_outputs key
/// 64        8    last_fetched_secs     unix timestamp
/// 72        2    status_code           last HTTP status
/// 74       46    reserved              future ETag/Last-Modified slot
/// ```
#[derive(Debug, Clone, Copy, PartialEq, Eq, rkyv::Archive, rkyv::Serialize, rkyv::Deserialize)]
#[repr(C)]
pub struct UrlStateRaw {
    pub content_hash: [u8; 32],
    pub url_hash: [u8; 32],
    pub last_fetched_secs: u64,
    pub status_code: u16,
    pub reserved: [u8; 46],
}

impl UrlStateRaw {
    /// Fixed byte size.
    pub const SIZE: usize = 120;

    /// Create an all-zero instance.
    #[must_use]
    pub const fn zeroed() -> Self {
        Self {
            content_hash: [0u8; 32],
            url_hash: [0u8; 32],
            last_fetched_secs: 0,
            status_code: 0,
            reserved: [0u8; 46],
        }
    }

    /// Deserialize from exactly 120 raw bytes.
    ///
    /// # Errors
    ///
    /// Returns [`StateError::PodSizeMismatch`] or [`StateError::PodCastFailed`].
    pub fn from_bytes(bytes: &[u8]) -> Result<Self, StateError> {
        if bytes.len() != Self::SIZE {
            return Err(StateError::PodSizeMismatch {
                table: TABLE_NAME_URL_STATE,
                expected: Self::SIZE,
                actual: bytes.len(),
            });
        }
        Ok(Self {
            content_hash: read_array::<32>(bytes, 0)?,
            url_hash: read_array::<32>(bytes, 32)?,
            last_fetched_secs: u64::from_le_bytes(read_array::<8>(bytes, 64)?),
            status_code: u16::from_le_bytes(read_array::<2>(bytes, 72)?),
            reserved: read_array::<46>(bytes, 74)?,
        })
    }

    /// Serialize to exactly 120 raw bytes.
    #[must_use]
    pub fn to_bytes(&self) -> [u8; Self::SIZE] {
        let mut r = [0u8; Self::SIZE];
        copy_into(&mut r, 0, &self.content_hash);
        copy_into(&mut r, 32, &self.url_hash);
        copy_into(&mut r, 64, &self.last_fetched_secs.to_le_bytes());
        copy_into(&mut r, 72, &self.status_code.to_le_bytes());
        copy_into(&mut r, 74, &self.reserved);
        r
    }
}

// ---------------------------------------------------------------------------
// Internal byte helpers (safe, no bytemuck)
// ---------------------------------------------------------------------------

/// Copy `src` into `dest` at the given offset.
pub(crate) fn copy_into(dest: &mut [u8], offset: usize, src: &[u8]) {
    let end = offset.saturating_add(src.len());
    if let Some(slice) = dest.get_mut(offset..end) {
        slice.copy_from_slice(src);
    }
}

/// Read a fixed-size array from `src` at the given offset.
pub(crate) fn read_array<const N: usize>(src: &[u8], offset: usize) -> Result<[u8; N], StateError> {
    let end = offset
        .checked_add(N)
        .ok_or_else(|| StateError::PodCastFailed {
            type_name: "slice",
            message: format!("offset overflow at offset {offset}"),
        })?;
    let slice = src
        .get(offset..end)
        .ok_or_else(|| StateError::PodCastFailed {
            type_name: "slice",
            message: format!("slice [{offset}..{end}) out of bounds (len={})", src.len()),
        })?;
    slice.try_into().map_err(|_| StateError::PodCastFailed {
        type_name: "slice",
        message: format!("failed to convert {N}-byte slice to array"),
    })
}

// ---------------------------------------------------------------------------
// Pod read helpers
// ---------------------------------------------------------------------------

/// Read and validate a `FileStateRaw` from raw redb bytes.
pub fn read_file_state_raw(bytes: &[u8]) -> Result<FileStateRaw, StateError> {
    FileStateRaw::from_bytes(bytes)
}

/// Read and validate a `UrlStateRaw` from raw redb bytes.
pub fn read_url_state_raw(bytes: &[u8]) -> Result<UrlStateRaw, StateError> {
    UrlStateRaw::from_bytes(bytes)
}

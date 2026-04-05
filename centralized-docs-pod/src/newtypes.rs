//! Domain newtypes for type-safe field access.
//!
//! All newtypes use `#[repr(transparent)]` to guarantee zero layout overhead
//! relative to the underlying representation. This is critical because the
//! parent structs (`FileStateRaw`, `UrlStateRaw`) are `#[repr(C)]` Pod types
//! whose byte layout must remain stable for zero-copy redb storage.

use std::ops::Deref;

/// A SHA-256 hash (32 bytes).
///
/// Zero-cost wrapper — `#[repr(transparent)]` ensures identical layout to `[u8; 32]`.
/// Derives `Pod`/`Zeroable` automatically because the inner type implements them.
#[repr(transparent)]
#[derive(
    Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, bytemuck::Pod, bytemuck::Zeroable,
)]
pub struct Sha256Hash(pub [u8; 32]);

impl Deref for Sha256Hash {
    type Target = [u8; 32];

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl From<[u8; 32]> for Sha256Hash {
    fn from(bytes: [u8; 32]) -> Self {
        Self(bytes)
    }
}

/// A valid HTTP status code (100–599).
///
/// `#[repr(transparent)]` ensures identical layout to `u16`.
/// Derives `Pod`/`Zeroable` automatically because the inner type implements them.
#[repr(transparent)]
#[derive(
    Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, bytemuck::Pod, bytemuck::Zeroable,
)]
pub struct HttpStatus(pub u16);

impl HttpStatus {
    /// Create a validated `HttpStatus`.
    ///
    /// # Errors
    ///
    /// Returns an error if `value` is not in the range 100–599.
    pub fn new(value: u16) -> Result<Self, &'static str> {
        match value {
            100..=599 => Ok(Self(value)),
            _ => Err("HTTP status code must be between 100 and 599"),
        }
    }
}

impl Deref for HttpStatus {
    type Target = u16;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl From<u16> for HttpStatus {
    fn from(value: u16) -> Self {
        Self(value)
    }
}

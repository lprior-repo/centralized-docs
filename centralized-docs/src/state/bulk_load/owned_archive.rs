//! Owned wrapper around rkyv-archived bytes.

use std::marker::PhantomData;

use super::error::BulkLoadError;
use super::hex_encode::hex_encode;

/// Owned wrapper around rkyv-archived bytes.
///
/// Decouples the archived view from the redb transaction lifetime by copying
/// bytes into a `Box<[u8]>` on construction. The archived data is validated
/// via rkyv bytecheck at construction time (see [`try_from_bytes`]).
///
/// [`try_from_bytes`]: OwnedArchive::try_from_bytes
#[derive(Debug)]
pub struct OwnedArchive<T: rkyv::Archive> {
    bytes: Box<[u8]>,
    _marker: PhantomData<T>,
}

impl<T> OwnedArchive<T>
where
    T: rkyv::Archive,
    T::Archived: rkyv::Portable
        + for<'a> rkyv::bytecheck::CheckBytes<
            rkyv::rancor::Strategy<
                rkyv::validation::Validator<
                    rkyv::validation::archive::ArchiveValidator<'a>,
                    rkyv::validation::shared::SharedValidator,
                >,
                rkyv::rancor::Error,
            >,
        >,
{
    /// Construct from raw bytes, validating rkyv bytecheck on construction.
    ///
    /// The bytes are copied into a `Box<[u8]>` that is fully owned by the
    /// resulting `OwnedArchive`, independent of any redb `AccessGuard`.
    ///
    /// # Errors
    ///
    /// Returns [`BulkLoadError::CorruptPayload`] if bytecheck validation fails.
    pub fn try_from_bytes(
        table: &'static str,
        key: &[u8; 32],
        bytes: Box<[u8]>,
    ) -> Result<Self, BulkLoadError> {
        // Validate first (borrows bytes), then move on success.
        if let Err(e) = rkyv::access::<T::Archived, rkyv::rancor::Error>(&bytes) {
            return Err(BulkLoadError::CorruptPayload {
                table,
                key_hex: hex_encode(key),
                message: e.to_string(),
            });
        }
        Ok(Self {
            bytes,
            _marker: PhantomData,
        })
    }

    /// Return the raw archived bytes.
    pub fn as_bytes(&self) -> &[u8] {
        &self.bytes
    }

    /// Zero-copy access to the archived root.
    ///
    /// Lifetime is tied to `&self`, NOT to any redb transaction.
    /// Re-validates via bytecheck (necessary due to `forbid(unsafe_code)`),
    /// which is guaranteed to succeed since [`try_from_bytes`] already
    /// validated the same bytes.
    ///
    /// # Errors
    ///
    /// Theoretically unreachable after successful [`try_from_bytes`]
    /// construction. Returns [`BulkLoadError::CorruptPayload`] only if
    /// the internal bytes were somehow corrupted after construction.
    pub fn archived(&self) -> Result<&T::Archived, BulkLoadError> {
        rkyv::access::<T::Archived, rkyv::rancor::Error>(&self.bytes).map_err(|e| {
            BulkLoadError::CorruptPayload {
                table: "<archived>",
                key_hex: String::new(),
                message: e.to_string(),
            }
        })
    }
}

impl<T> OwnedArchive<T>
where
    T: rkyv::Archive,
    T::Archived: rkyv::Portable
        + for<'a> rkyv::bytecheck::CheckBytes<
            rkyv::rancor::Strategy<
                rkyv::validation::Validator<
                    rkyv::validation::archive::ArchiveValidator<'a>,
                    rkyv::validation::shared::SharedValidator,
                >,
                rkyv::rancor::Error,
            >,
        >,
    T::Archived: rkyv::Deserialize<T, rkyv::api::high::HighDeserializer<rkyv::rancor::Error>>,
{
    /// Full deserialize into an owned value.
    ///
    /// # Errors
    ///
    /// Documented as structurally unreachable: rkyv's bytecheck validation
    /// (performed in [`try_from_bytes`]) is strictly stronger than
    /// deserialization. If bytecheck passes, deserialization must succeed.
    /// Returns [`BulkLoadError::CorruptPayload`] for API completeness.
    pub fn deserialize(&self) -> Result<T, BulkLoadError> {
        rkyv::from_bytes::<T, rkyv::rancor::Error>(&self.bytes).map_err(|e| {
            BulkLoadError::CorruptPayload {
                table: "<deserialize>",
                key_hex: String::new(),
                message: e.to_string(),
            }
        })
    }
}

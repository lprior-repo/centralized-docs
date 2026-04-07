//! Bulk loader methods for archived output tables.
//!
//! Provides [`StateReadSession`] bulk loader methods for the four variable-size
//! output tables in the redb state database. Each loader accepts a slice of
//! `[u8; 32]` hash keys, reads every matching value inside the shared read
//! transaction, validates the rkyv bytes via bytecheck, and returns a `HashMap`
//! keyed by the same hashes.
//!
//! # Architecture (Data → Calc → Actions)
//!
//! - **Data**: [`BulkLoadError`], [`OwnedArchive`]
//! - **Calculations**: [`hex_encode`], [`load_entries`] (generic bulk loader)
//! - **Actions**: [`StateReadSession::new`] (opens read transaction),
//!   bulk loader methods (reads from redb)

#![cfg_attr(
    not(test),
    deny(clippy::unwrap_used, clippy::expect_used, clippy::panic)
)]
#![forbid(unsafe_code)]
#![warn(clippy::pedantic)]
#![allow(clippy::module_name_repetitions)]
#![allow(clippy::missing_errors_doc)]
#![allow(clippy::must_use_candidate)]
#![allow(clippy::return_self_not_must_use)]
#![allow(clippy::type_complexity)]
#![allow(clippy::min_ident_chars)]
#[cfg_attr(
    test,
    allow(
        clippy::unwrap_used,
        clippy::expect_used,
        clippy::panic,
        clippy::arithmetic_side_effects
    )
)]
mod error;
mod hex_encode;
mod owned_archive;
mod session;

pub use error::BulkLoadError;
pub use hex_encode::hex_encode;
pub use owned_archive::OwnedArchive;
pub use session::StateReadSession;

#[cfg(test)]
mod tests;

//! Persisted output records for archive-safe storage with rkyv zero-copy deserialization.
//!
//! Defines append-only persisted record types that mirror the runtime domain types across
//! all five pipeline phases. These records derive `rkyv::Archive`, `rkyv::Serialize`, and
//! `rkyv::Deserialize` for zero-copy deserialization from mmapped files.
//!
//! # Key Conversions
//!
//! - `Arc<str>` → `String`
//! - `HashMap<K, V>` → sorted `Vec<(K, V)>` (deterministic serialization)
//! - `DateTime<Utc>` → `i64` (unix epoch seconds, lossy: sub-second precision dropped)
//!
//! # Design Principles
//!
//! - Infallible `*_to_persisted` conversions (runtime data is already validated)
//! - Fallible `persisted_*_to_runtime` conversions (validates field constraints)
//! - All top-level batch records carry `schema_version: u32` (currently 1)
//! - Zero mutation in conversion functions; pure transforms only

#![deny(clippy::unwrap_used)]
#![deny(clippy::expect_used)]
#![deny(clippy::panic)]
#![forbid(unsafe_code)]
#![allow(clippy::too_many_lines)]
#![allow(clippy::module_name_repetitions)]
#![allow(clippy::missing_errors_doc)]
#![allow(clippy::must_use_candidate)]
#![allow(clippy::return_self_not_must_use)]

mod analysis;
mod assign;
mod chunk;
mod error;
mod scrape;
mod transform;
mod watch;

pub use analysis::*;
pub use assign::*;
pub use chunk::*;
pub use error::PersistError;
pub use scrape::*;
pub use transform::*;
pub use watch::*;

//! Bulk loader integration tests for `StateReadSession`.
//!
//! Tests Behaviors 1–35 from the approved test plan for bead cdocs-4s3.

#![allow(clippy::pedantic)]
#![allow(clippy::unwrap_used)]
#![allow(clippy::expect_used)]
#![allow(clippy::panic)]
#![allow(clippy::indexing_slicing)]
#![allow(clippy::bool_assert_comparison)]
#![allow(clippy::bool_to_int_with_if)]

pub mod boundary_tests;
pub mod common;
pub mod load_analyses_tests;
pub mod load_chunks_tests;
pub mod load_scrapes_tests;
pub mod load_transforms_tests;
pub mod owned_archive_tests;
pub mod session_lifecycle_tests;

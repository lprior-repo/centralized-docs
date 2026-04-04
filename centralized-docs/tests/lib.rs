#![allow(clippy::pedantic)]
#![allow(clippy::unwrap_used)]
#![allow(clippy::expect_used)]
#![allow(clippy::panic)]

//! Integration test suite for ctd
//!
//! This is the root of the integration test module tree. Tests are organized
//! by capability rather than by mechanism.

// Common fixtures and utilities - public for all sub-modules to use
pub mod common;

// Capability-based test modules
pub mod bulk_load;
pub mod document_indexing;
pub mod e2e;
pub mod full_pipeline;
pub mod markdown_transform;
pub mod proptests;
pub mod scrape_reuse_tests;
pub mod scrape_state_wiring;
pub mod semantic_search;

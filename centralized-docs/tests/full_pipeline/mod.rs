#![allow(clippy::pedantic)]
#![allow(clippy::unwrap_used)]
#![allow(clippy::expect_used)]
#![allow(clippy::panic)]

//! Integration Tests
//!
//! Full pipeline integration tests covering:
//! - Complete end-to-end pipeline
//! - Table-driven test cases
//! - Behavior-focused scenarios
//! - Standalone integration tests

mod full_pipeline_integration;
mod integration_tests;
mod pipeline_integration_tests;
mod scenario_tests;
mod standalone_integration_tests;

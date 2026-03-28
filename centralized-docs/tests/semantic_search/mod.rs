#![allow(clippy::pedantic)]
#![allow(clippy::unwrap_used)]
#![allow(clippy::expect_used)]
#![allow(clippy::panic)]


//! Semantic Search Tests
//!
//! Tests for the semantic search capability:
//! - Limit validation
//! - Score handling (negative zero)
//! - Query sanitization
//! - Search edge cases
//! - Error handling

mod limit_validation_tests;
mod negative_zero_score_tests;
mod sanitize_query_tests;
mod search_edge_cases_tests;
mod search_error_tests;
mod search_path_format_tests;

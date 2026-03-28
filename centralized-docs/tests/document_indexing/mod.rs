#![allow(clippy::pedantic)]
#![allow(clippy::unwrap_used)]
#![allow(clippy::expect_used)]
#![allow(clippy::panic)]


//! Document Indexing Tests
//!
//! Tests for the document indexing capability:
//! - Empty directory handling
//! - File discovery
//! - Index schema compliance
//! - Single file indexing
//! - Path handling

mod empty_directory_tests;
mod file_discovery_tests;
mod index_schema_tests;
mod path_handling_tests;
mod single_file_indexing_tests;
mod test_tantivy_index_populated;

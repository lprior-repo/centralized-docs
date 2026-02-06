//! Integration test suite for doc_transformer
//!
//! This is the root of the integration test module tree. Tests are organized
//! by capability rather than by mechanism.

// Common fixtures and utilities - public for all sub-modules to use
pub mod common;

// Capability-based test modules
pub mod document_indexing;
pub mod full_pipeline;
pub mod markdown_transform;
pub mod proptests;
pub mod semantic_search;

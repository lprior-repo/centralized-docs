//! Document and query validation.
//!
//! Provides validation for markdown documents (frontmatter, heading structure,
//! required fields) and search query/limit validation.

pub mod file_validation;
pub mod query_validation;
pub mod types;

// Re-export all public API for backward compatibility
pub use file_validation::validate_all;
pub use query_validation::{validate_limit, validate_query};
pub use types::{FileValidationResult, ValidationError, ValidationResult};

#[cfg(test)]
mod query_limit_tests;

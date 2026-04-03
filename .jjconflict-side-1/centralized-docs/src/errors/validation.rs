//! Validation-related error types.

use thiserror::Error;

/// Validation-related errors.
#[non_exhaustive]
#[derive(Debug, Error, Clone, PartialEq, Eq)]
#[allow(dead_code)]
pub enum ValidationError {
    #[error("query cannot be empty")]
    EmptyQuery,

    #[error("query too long: {length} bytes (max {max})")]
    QueryTooLong { length: usize, max: usize },

    #[error("query contains regex patterns which are not allowed")]
    RegexQuery,

    #[error("search limit must be positive, got {limit}")]
    InvalidLimit { limit: i64 },

    #[error("file validation failed: {path}")]
    FileValidation { path: String, message: String },

    #[error("document has {h1_count} H1 heading(s); expected exactly 1")]
    MultipleH1Headings { h1_count: usize },

    #[error("document has no H1 heading")]
    MissingH1Heading,

    #[error("document too short: {word_count} words (min {min})")]
    DocumentTooShort { word_count: usize, min: usize },

    #[error("missing required frontmatter field: {field}")]
    MissingFrontmatter { field: String },

    #[error("invalid frontmatter format: {message}")]
    InvalidFrontmatter { message: String },

    #[error("category '{category}' is not defined in configuration")]
    UnknownCategory { category: String },

    #[error("broken link detected: {link} -> {target} (file not found)")]
    BrokenLink {
        link: String,
        target: String,
        source_file: String,
    },
}

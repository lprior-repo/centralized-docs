use serde::{Deserialize, Serialize};
use thiserror::Error;

/// Query validation errors
#[derive(Debug, Error, PartialEq, Eq)]
pub enum ValidationError {
    #[error("Query cannot be empty")]
    EmptyQuery,

    #[error("Query too long ({length} bytes, max {max})")]
    QueryTooLong { length: usize, max: usize },

    #[error("Regex queries not allowed (potential ReDoS attack)")]
    RegexNotAllowed,

    #[error("Query contains null bytes which are not allowed")]
    NullBytesNotAllowed,

    #[error("Limit must be positive (cannot return negative results), got {0}")]
    InvalidLimitNegative(i64),

    #[error("limit must be at least 1 (use --limit 1 or higher)")]
    InvalidLimitZero,

    #[error("limit must be at most 1000 results, got {0}")]
    InvalidLimitTooLarge(i64),
}

#[derive(Debug, Serialize, Deserialize)]
pub struct FileValidationResult {
    pub file_path: String,
    pub errors: Vec<String>,
    pub warnings: Vec<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ValidationResult {
    pub files_checked: usize,
    pub files_passed: usize,
    pub total_errors: usize,
    pub total_warnings: usize,
    pub failed_files: Vec<FileValidationResult>,
}

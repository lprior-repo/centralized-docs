use anyhow::Result;

use super::types::ValidationError;

/// Validate query length for search operations
///
/// ## Design by Contract
///
/// **Preconditions:**
/// - Query may be any length (including 0)
/// - Validation happens before expensive operations
///
/// **Postconditions:**
/// - Queries < 1 char (trimmed) rejected with `EmptyQuery`
/// - Queries > 1024 bytes rejected with `QueryTooLong`
/// - Valid queries (1-1024 bytes) return Ok with trimmed query
///
/// **Invariants:**
/// - No expensive operations on invalid input
/// - Error messages are user-friendly
/// - Validation is consistent across all entry points
///
/// ## Error Handling
///
/// Returns `ValidationError` for invalid queries:
/// - `EmptyQuery`: Query is empty or whitespace-only after trimming
/// - `QueryTooLong`: Query exceeds 1024 byte limit
///
/// ## Example
///
/// ```
/// use doc_transformer::validate::{validate_query, ValidationError};
///
/// // Valid query
/// assert!(validate_query("rust programming").is_ok());
///
/// // Empty query
/// assert!(matches!(validate_query(""), Err(ValidationError::EmptyQuery)));
/// assert!(matches!(validate_query("   "), Err(ValidationError::EmptyQuery)));
///
/// // Too long query (limit is 1024 bytes)
/// let long = "a".repeat(1025);
/// assert!(matches!(validate_query(&long), Err(ValidationError::QueryTooLong{..})));
/// ```
pub fn validate_query(query: &str) -> Result<&str, ValidationError> {
    const MAX_QUERY_LENGTH: usize = 1024;

    // Check for null bytes before trimming - these should be rejected
    // as they may cause unexpected behavior in search backends
    if query.contains('\0') {
        return Err(ValidationError::NullBytesNotAllowed);
    }

    let trimmed = query.trim();

    if trimmed.is_empty() {
        return Err(ValidationError::EmptyQuery);
    }

    if trimmed.len() > MAX_QUERY_LENGTH {
        return Err(ValidationError::QueryTooLong {
            length: trimmed.len(),
            max: MAX_QUERY_LENGTH,
        });
    }

    if contains_regex_pattern(trimmed) {
        return Err(ValidationError::RegexNotAllowed);
    }

    Ok(trimmed)
}

/// Validate a limit value (must be greater than 0)
/// Used by internal library functions to prevent Tantivy panics
pub fn validate_limit(s: &str) -> Result<usize, ValidationError> {
    let value = s
        .parse::<i64>()
        .map_err(|_| ValidationError::InvalidLimitNegative(0))?;

    if value < 0 {
        return Err(ValidationError::InvalidLimitNegative(value));
    }

    if value == 0 {
        return Err(ValidationError::InvalidLimitZero);
    }

    if value > 1000 {
        return Err(ValidationError::InvalidLimitTooLarge(value));
    }

    value
        .try_into()
        .map_err(|_| ValidationError::InvalidLimitTooLarge(value))
}

fn contains_regex_pattern(query: &str) -> bool {
    let chars: Vec<char> = query.chars().collect();
    let len = chars.len();

    (0..len).any(|i| {
        if chars[i] != '/' {
            return false;
        }
        let next_idx = i.saturating_add(1);
        if next_idx < len && chars[next_idx] == '/' {
            return false;
        }
        let start_idx = i.saturating_add(1);
        (start_idx..len).any(|j| {
            chars[j] == '/' && {
                let next_j = j.saturating_add(1);
                next_j >= len || chars[next_j] != '/'
            }
        })
    })
}

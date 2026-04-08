//! Query and limit validation tests.
//!
//! Separated from `query_validation.rs` to keep that file under 300 lines.

use super::query_validation::{validate_limit, validate_query};
use super::types::ValidationError;

// ============================================================================
// Query validation tests
// ============================================================================

#[test]
fn test_validate_query_empty() {
    let result = validate_query("");
    assert!(matches!(result, Err(ValidationError::EmptyQuery)));
}

#[test]
fn test_validate_query_whitespace_only() {
    let result = validate_query("   ");
    assert!(matches!(result, Err(ValidationError::EmptyQuery)));
}

#[test]
fn test_validate_query_tabs_and_newlines() {
    let result = validate_query("\t\n  \r\n");
    assert!(matches!(result, Err(ValidationError::EmptyQuery)));
}

#[test]
fn test_validate_query_single_char() {
    assert_eq!(validate_query("a"), Ok("a"));
}

#[test]
fn test_validate_query_normal() {
    assert_eq!(validate_query("rust programming"), Ok("rust programming"));
}

#[test]
fn test_validate_query_trimmed() {
    assert_eq!(
        validate_query("  rust programming  "),
        Ok("rust programming")
    );
}

#[test]
fn test_validate_query_at_limit() {
    let query = "a".repeat(1024);
    let result = validate_query(&query);
    assert!(result.is_ok());
    assert_eq!(result.map(str::len), Ok(1024));
}

#[test]
fn test_validate_query_exceeds_limit() {
    let query = "a".repeat(1025);
    let result = validate_query(&query);
    assert!(matches!(
        result,
        Err(ValidationError::QueryTooLong {
            length: 1025,
            max: 1024
        })
    ));
}

#[test]
fn test_validate_query_far_exceeds_limit() {
    let query = "a".repeat(5000);
    let result = validate_query(&query);
    assert!(matches!(
        result,
        Err(ValidationError::QueryTooLong {
            length: 5000,
            max: 1024
        })
    ));
}

#[test]
fn test_validate_query_unicode() {
    assert_eq!(validate_query("café rust"), Ok("café rust"));
}

#[test]
fn test_validate_query_unicode_at_limit() {
    // Euro sign "€" is 3 bytes, so 341 reps = 1023 bytes + "a" = 1024 bytes
    let query = format!("{}a", "€".repeat(341));
    assert_eq!(query.len(), 1024);
    assert!(validate_query(&query).is_ok());
}

#[test]
fn test_validate_query_unicode_exceeds_limit() {
    // Euro sign "€" is 3 bytes, 342 reps = 1026 bytes
    let query = "€".repeat(342);
    assert_eq!(query.len(), 1026);
    let result = validate_query(&query);
    assert!(matches!(
        result,
        Err(ValidationError::QueryTooLong {
            length: 1026,
            max: 1024
        })
    ));
}

#[test]
fn test_validate_query_special_chars() {
    assert_eq!(
        validate_query("rust-lang & systems *2025*"),
        Ok("rust-lang & systems *2025*")
    );
}

#[test]
fn test_validate_query_error_message_empty() {
    let result = validate_query("");
    assert!(result.is_err());
    // Convert error to string for message validation
    let err_str = result.as_ref().map_err(ToString::to_string);
    assert!(matches!(err_str, Err(ref msg) if msg == "Query cannot be empty"));
}

#[test]
fn test_validate_query_error_message_too_long() {
    let query = "a".repeat(1025);
    let result = validate_query(&query);
    assert!(result.is_err());
    // Convert error to string for message validation
    let err_msg = result.as_ref().map_err(ToString::to_string);
    if let Err(msg) = err_msg {
        assert!(msg.contains("1025"));
        assert!(msg.contains("1024"));
        assert!(msg.contains("too long"));
    }
}

#[test]
fn test_validate_query_rejects_simple_regex() {
    assert!(matches!(
        validate_query("/rust/"),
        Err(ValidationError::RegexNotAllowed)
    ));
}

#[test]
fn test_validate_query_rejects_nested_regex() {
    assert!(matches!(
        validate_query("/((a+)+)b/"),
        Err(ValidationError::RegexNotAllowed)
    ));
}

#[test]
fn test_validate_query_rejects_regex_in_middle() {
    assert!(matches!(
        validate_query("search /rust/ programming"),
        Err(ValidationError::RegexNotAllowed)
    ));
}

#[test]
fn test_validate_query_rejects_multiple_regex() {
    assert!(matches!(
        validate_query("/rust/ OR /python/"),
        Err(ValidationError::RegexNotAllowed)
    ));
}

#[test]
fn test_validate_query_accepts_slash_without_regex() {
    assert_eq!(validate_query("rust/python"), Ok("rust/python"));
}

#[test]
fn test_validate_query_accepts_double_slash() {
    assert_eq!(validate_query("// comment"), Ok("// comment"));
}

#[test]
fn test_validate_query_accepts_double_slash_at_end() {
    assert_eq!(validate_query("rust //"), Ok("rust //"));
}

#[test]
fn test_validate_query_rejects_regex_error_message() {
    let result = validate_query("/((a+)+)b/");
    assert!(result.is_err());
    let err_msg = result.as_ref().map_err(ToString::to_string);
    assert!(matches!(err_msg, Err(ref msg) if msg.contains("Regex")));
}

#[test]
fn test_validate_query_rejects_null_byte() {
    let result = validate_query("test\0query");
    assert!(matches!(result, Err(ValidationError::NullBytesNotAllowed)));
}

#[test]
fn test_validate_query_rejects_null_byte_at_start() {
    let result = validate_query("\0test");
    assert!(matches!(result, Err(ValidationError::NullBytesNotAllowed)));
}

#[test]
fn test_validate_query_rejects_null_byte_at_end() {
    let result = validate_query("test\0");
    assert!(matches!(result, Err(ValidationError::NullBytesNotAllowed)));
}

#[test]
fn test_validate_query_rejects_multiple_null_bytes() {
    let result = validate_query("test\0\0query");
    assert!(matches!(result, Err(ValidationError::NullBytesNotAllowed)));
}

// ============================================================================
// Limit validation tests
// ============================================================================

#[test]
fn test_validate_limit_zero() {
    let result = validate_limit("0");
    assert!(result.is_err());
    let err_msg = result.as_ref().map_err(ToString::to_string);
    assert!(matches!(err_msg, Err(ref msg) if msg.contains("at least 1")));
}

#[test]
fn test_validate_limit_one() {
    assert_eq!(validate_limit("1"), Ok(1));
}

#[test]
fn test_validate_limit_normal() {
    assert_eq!(validate_limit("10"), Ok(10));
    assert_eq!(validate_limit("100"), Ok(100));
    assert_eq!(validate_limit("1000"), Ok(1000));
}

#[test]
fn test_validate_limit_negative() {
    let result = validate_limit("-1");
    assert!(result.is_err());
    let err_msg = result.as_ref().map_err(ToString::to_string);
    assert!(matches!(err_msg, Err(ref msg) if msg.contains("positive")));
}

#[test]
fn test_validate_limit_too_large() {
    let result = validate_limit("1001");
    assert!(result.is_err());
    let err_msg = result.as_ref().map_err(ToString::to_string);
    assert!(matches!(err_msg, Err(ref msg) if msg.contains("at most 1000")));
}

#[test]
fn test_validate_limit_error_message() {
    let result = validate_limit("0");
    assert!(result.is_err());
    let err_msg = result.as_ref().map_err(ToString::to_string);
    assert!(matches!(err_msg, Err(ref msg) if msg.contains("at least 1")));
}

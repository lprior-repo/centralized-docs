//! Tests for title extraction and regex safety.

#![allow(clippy::unwrap_used)]

use crate::scrape::validation::compile_safe_regex;
use crate::scrape::validation::extract_title;

#[test]
fn test_extract_title() {
    let md = "# Getting Started\n\nThis is content.";
    assert_eq!(extract_title(md, "https://example.com"), "Getting Started");

    let md_no_h1 = "Some content without header";
    assert_eq!(
        extract_title(md_no_h1, "https://example.com/getting-started"),
        "getting started"
    );
}

#[test]
fn test_extract_title_encoding_fallback_unicode() {
    let md = "# Getting Started \u{a9} 2024\n\nContent here.";
    assert_eq!(
        extract_title(md, "https://example.com"),
        "Getting Started \u{a9} 2024"
    );

    let md_accent = "# R\u{e9}sum\u{e9}\n\nContent here.";
    assert_eq!(
        extract_title(md_accent, "https://example.com"),
        "R\u{e9}sum\u{e9}"
    );

    let md_emoji = "# Hello World \u{1f30d}\n\nContent here.";
    assert_eq!(
        extract_title(md_emoji, "https://example.com"),
        "Hello World \u{1f30d}"
    );
}

#[test]
fn test_extract_title_url_encoding_fallback() {
    let md_no_h1 = "Some content without header";

    assert_eq!(
        extract_title(md_no_h1, "https://example.com/hello%20world"),
        "hello world"
    );

    assert_eq!(
        extract_title(md_no_h1, "https://example.com/hello_world"),
        "hello world"
    );

    assert_eq!(
        extract_title(md_no_h1, "https://example.com/hello-world"),
        "hello world"
    );

    assert_eq!(
        extract_title(md_no_h1, "https://example.com/my%20test_page"),
        "my test page"
    );

    assert_eq!(
        extract_title(md_no_h1, "https://example.com/docs/api/v2"),
        "v2"
    );
}

#[test]
fn test_extract_title_edge_cases() {
    assert_eq!(
        extract_title("", "https://example.com/my-document"),
        "my document"
    );

    assert_eq!(
        extract_title("   \n\n   ", "https://example.com/doc"),
        "doc"
    );

    let md_whitespace = "#   \n\nContent";
    assert_eq!(
        extract_title(md_whitespace, "https://example.com/fallback"),
        "fallback"
    );

    let md_multi = "# First Title\n\n# Second Title";
    assert_eq!(
        extract_title(md_multi, "https://example.com"),
        "First Title"
    );

    let md_trim = "#   Trimmed Title   \n\nContent";
    assert_eq!(
        extract_title(md_trim, "https://example.com"),
        "Trimmed Title"
    );
}

#[test]
fn test_extract_title_invalid_url_fallback() {
    let md_no_h1 = "Some content";
    assert_eq!(extract_title(md_no_h1, "not-a-valid-url"), "Untitled");
    assert_eq!(extract_title(md_no_h1, ""), "Untitled");
}

#[test]
fn test_compile_safe_regex_rejects_redos_pattern() {
    let redos_pattern = "([a-z]+)+$";
    let start = std::time::Instant::now();
    let result = compile_safe_regex(redos_pattern);

    assert!(
        result.is_err(),
        "ReDoS pattern should be rejected, got: {result:?}"
    );
    assert!(
        start.elapsed() < std::time::Duration::from_secs(1),
        "ReDoS rejection should be fast"
    );

    let error_msg = match &result {
        Err(e) => e.to_string(),
        Ok(_) => unreachable!(),
    };
    assert!(
        error_msg.contains("ReDoS"),
        "Error message should mention ReDoS: {error_msg}"
    );
}

#[test]
fn test_compile_safe_regex_rejects_nested_star_pattern() {
    let nested_star = "(.*)*";
    let result = compile_safe_regex(nested_star);

    assert!(result.is_err(), "Nested star pattern should be rejected");

    let error_msg = match &result {
        Err(e) => e.to_string(),
        Ok(_) => unreachable!(),
    };
    assert!(
        error_msg.contains("ReDoS"),
        "Error message should mention ReDoS: {error_msg}"
    );
}

#[test]
fn test_compile_safe_regex_rejects_long_pattern() {
    let long_pattern = "a".repeat(1000);
    let result = compile_safe_regex(&long_pattern);

    assert!(result.is_err(), "Pattern > 500 chars should be rejected");

    let error_msg = match &result {
        Err(e) => e.to_string(),
        Ok(_) => unreachable!(),
    };
    assert!(
        error_msg.contains("too long"),
        "Error message should mention length limit: {error_msg}"
    );
}

#[test]
fn test_compile_safe_regex_accepts_valid_pattern() {
    let valid_pattern = r"^/docs/.*\.md$";
    let result = compile_safe_regex(valid_pattern);

    assert!(
        result.is_ok(),
        "Valid pattern should be accepted, got: {result:?}"
    );
}

#[test]
fn test_compile_safe_regex_rejects_invalid_syntax() {
    let invalid_syntax = "(?P<invalid";
    let result = compile_safe_regex(invalid_syntax);

    assert!(result.is_err(), "Invalid syntax should be rejected");

    let error_msg = match &result {
        Err(e) => e.to_string(),
        Ok(_) => unreachable!(),
    };
    assert!(
        error_msg.contains("Invalid") || error_msg.contains("regex"),
        "Error message should describe the error: {error_msg}"
    );
}

#[test]
fn test_compile_safe_regex_accepts_empty_string() {
    let empty_pattern = "";
    let result = compile_safe_regex(empty_pattern);

    assert!(result.is_ok(), "Empty pattern should be valid");
}

#[test]
fn test_compile_safe_regex_rejects_single_char_nested_quantifier() {
    let result = compile_safe_regex("(a+)+$");
    assert!(result.is_err(), "(a+)+$ must be rejected as ReDoS");
}

#[test]
fn test_compile_safe_regex_rejects_alternation_nested_quantifier() {
    let result = compile_safe_regex("(a|a)+");
    assert!(result.is_err(), "(a|a)+ must be rejected as ReDoS");
}

#[test]
fn test_compile_safe_regex_rejects_word_char_nested_quantifier() {
    let result = compile_safe_regex(r"(\w)+");
    assert!(result.is_err(), "(\\w)+ must be rejected as ReDoS");
}

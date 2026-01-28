//! Tests for query sanitization (doc-tx-6aq)

use doc_transformer::validate::sanitize_query;

#[test]
fn test_sanitize_query_basic_special_chars() {
    assert_eq!(sanitize_query("<test>"), "\\<test\\>");
    assert_eq!(sanitize_query("&test"), "\\&test");
    assert_eq!(sanitize_query("\"test\""), "\\\"test\\\"");
    assert_eq!(sanitize_query("'test'"), "\\'test\\'");
    assert_eq!(sanitize_query("(test)"), "\\(test\\)");
    assert_eq!(sanitize_query("!test"), "\\!test");
    assert_eq!(sanitize_query("|test"), "\\|test");
    assert_eq!(sanitize_query("+test"), "\\+test");
    assert_eq!(sanitize_query("-test"), "\\-test");
    assert_eq!(sanitize_query("/test"), "\\/test");
    assert_eq!(sanitize_query("\\test"), "\\\\test");
}

#[test]
fn test_sanitize_query_html_tags() {
    assert_eq!(
        sanitize_query("<script>alert(1)</script>"),
        "\\<script\\>alert\\(1\\)\\<\\/script\\>"
    );
    assert_eq!(
        sanitize_query("<div>content</div>"),
        "\\<div\\>content\\<\\/div\\>"
    );
}

#[test]
fn test_sanitize_query_boolean_operators() {
    assert_eq!(sanitize_query("rust AND python"), "rust AND python");
    assert_eq!(sanitize_query("rust OR python"), "rust OR python");
    assert_eq!(sanitize_query("rust NOT python"), "rust NOT python");
}

#[test]
fn test_sanitize_query_xss_examples() {
    assert_eq!(
        sanitize_query("<img src=x onerror=alert(1)>"),
        "\\<img src=x onerror=alert\\(1\\)\\>"
    );
    assert_eq!(
        sanitize_query("<svg/onload=alert(1)>"),
        "\\<svg\\/onload=alert\\(1\\)\\>"
    );
}

#[test]
fn test_sanitize_query_normal_text() {
    assert_eq!(sanitize_query("rust programming"), "rust programming");
    assert_eq!(sanitize_query("hello world"), "hello world");
}

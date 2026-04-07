//! Tests for URL validation.

#![allow(clippy::unwrap_used)]

use super::*;

#[test]
fn test_validate_url_valid() {
    assert!(validate_url("https://example.com").is_ok());
    assert!(validate_url("http://docs.rust-lang.org/book").is_ok());
}

#[test]
fn test_validate_url_invalid() {
    assert!(validate_url("not-a-url").is_err());
    assert!(validate_url("").is_err());
    assert!(validate_url("   ").is_err());
    assert!(validate_url("ftp://example.com").is_err());
    assert!(validate_url("example.com").is_err());
}

#[test]
fn test_validate_url_missing_host() {
    assert!(validate_url("https://").is_err());
    assert!(validate_url("https://?foo=bar").is_err());
}

#[test]
fn test_validate_url_valid_hosts() {
    assert!(validate_url("https://example.com").is_ok());
    assert!(validate_url("https://localhost:3000").is_ok());
    assert!(validate_url("https://[::1]:3000/docs").is_ok());
}

#[test]
fn test_validate_url_rejects_spaces() {
    let result = validate_url("https://example.com/foo bar");
    assert!(result.is_err(), "URL with spaces should be rejected");

    let error_msg = result.unwrap_err().to_string();
    assert!(
        error_msg.contains("space"),
        "Error should mention spaces: {error_msg}"
    );
}

#[test]
fn test_validate_url_rejects_control_characters() {
    let tab = validate_url("https://example.com/foo\tbar");
    assert!(tab.is_err(), "URL with tab should be rejected");

    let newline = validate_url("https://example.com/foo\nbar");
    assert!(newline.is_err(), "URL with newline should be rejected");
}

#[test]
fn test_validate_url_rejects_unencoded_special_chars() {
    let special_urls = [
        "https://example.com/foo[bar]",
        "https://example.com/foo{bar}",
        "https://example.com/foo|bar",
        "https://example.com/foo^bar",
        "https://example.com/foo`bar",
        "https://example.com/foo<bar>",
    ];

    for url in special_urls {
        let result = validate_url(url);
        assert!(
            result.is_err(),
            "URL with special chars should be rejected: {url}"
        );

        let error_msg = result.unwrap_err().to_string();
        assert!(
            error_msg.contains("unencoded") || error_msg.contains("percent-encoded"),
            "Error should mention encoding: {error_msg}"
        );
    }
}

#[test]
fn test_validate_url_accepts_percent_encoded() {
    assert!(validate_url("https://example.com/foo%20bar").is_ok());
    assert!(validate_url("https://example.com/foo%5Bbar%5D").is_ok());
}

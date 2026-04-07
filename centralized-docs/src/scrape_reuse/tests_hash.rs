//! Tests for content hash computation.

#![allow(clippy::unwrap_used)]
#![allow(clippy::expect_used)]
#![allow(clippy::panic)]

use super::compute_page_content_hash;

#[test]
fn compute_page_content_hash_returns_sha256_when_given_nonempty_markdown() {
    let hash = compute_page_content_hash("# Hello\n\nWorld");
    let expected: [u8; 32] = [
        0xad, 0x6e, 0x0b, 0xf8, 0x88, 0xda, 0x96, 0x4a, 0xb5, 0x79, 0x92, 0xe8, 0x6c, 0x6f, 0x89,
        0x4a, 0xae, 0xc3, 0x32, 0x5d, 0x7b, 0x18, 0x35, 0x5a, 0xb9, 0x2c, 0x81, 0xba, 0xbe, 0x81,
        0xc4, 0xa3,
    ];
    assert_eq!(hash, expected);
    assert_ne!(hash, [0u8; 32]);
}

#[test]
fn compute_page_content_hash_returns_sha256_of_empty_when_given_empty_string() {
    let hash = compute_page_content_hash("");
    let expected: [u8; 32] = [
        0xe3, 0xb0, 0xc4, 0x42, 0x98, 0xfc, 0x1c, 0x14, 0x9a, 0xfb, 0xf4, 0xc8, 0x99, 0x6f, 0xb9,
        0x24, 0x27, 0xae, 0x41, 0xe4, 0x64, 0x9b, 0x93, 0x4c, 0xa4, 0x95, 0x99, 0x1b, 0x78, 0x52,
        0xb8, 0x55,
    ];
    assert_eq!(hash, expected);
    assert_ne!(hash, [0u8; 32]);
}

#[test]
fn compute_page_content_hash_is_deterministic() {
    let h1 = compute_page_content_hash("test string");
    let h2 = compute_page_content_hash("test string");
    assert_eq!(h1, h2);
}

#[test]
fn compute_page_content_hash_differs_for_different_inputs() {
    let h1 = compute_page_content_hash("string a");
    let h2 = compute_page_content_hash("string b");
    assert_ne!(h1, h2);
}

#[test]
fn compute_page_content_hash_returns_32_bytes() {
    let hash = compute_page_content_hash("anything");
    assert_eq!(hash.len(), 32);
}

//! `hex_encode` pure function tests.

use super::*;

#[test]
fn hex_encode_returns_empty_string_for_empty_input() {
    assert_eq!(hex_encode(&[]), "");
}

#[test]
fn hex_encode_returns_00_for_zero_byte() {
    assert_eq!(hex_encode(&[0x00]), "00");
}

#[test]
fn hex_encode_returns_ff_for_255_byte() {
    assert_eq!(hex_encode(&[0xFF]), "ff");
}

#[test]
fn hex_encode_returns_lowercase_hex_for_mixed_bytes() {
    assert_eq!(hex_encode(&[0xDE, 0xAD, 0xBE, 0xEF]), "deadbeef");
}

#[test]
fn hex_encode_returns_two_chars_per_byte_for_single_byte() {
    assert_eq!(hex_encode(&[0x0A]).len(), 2);
    assert_eq!(hex_encode(&[0x0A]), "0a");
}

#[test]
fn hex_encode_output_length_is_double_input_length() {
    let input: Vec<u8> = vec![0x42; 100];
    assert_eq!(hex_encode(&input).len(), 200);
}

#[test]
fn hex_encode_handles_32_byte_hash_correctly() {
    let hash: [u8; 32] = [0xAB; 32];
    let encoded = hex_encode(&hash);
    assert_eq!(encoded.len(), 64);
    assert_eq!(encoded, "ab".repeat(32));
}

#[test]
fn hex_encode_preserves_leading_zeros() {
    assert_eq!(hex_encode(&[0x01, 0x02, 0x03]), "010203");
}

#[test]
fn hex_encode_all_zero_bytes() {
    let bytes = [0u8; 16];
    assert_eq!(hex_encode(&bytes), "0".repeat(32));
}

#[test]
fn hex_encode_all_ff_bytes() {
    let bytes = [0xFFu8; 8];
    assert_eq!(hex_encode(&bytes), "f".repeat(16));
}

#[test]
fn hex_encode_single_byte_boundary_min() {
    assert_eq!(hex_encode(&[0x00]), "00");
}

#[test]
fn hex_encode_single_byte_boundary_max() {
    assert_eq!(hex_encode(&[0xFF]), "ff");
}

#[test]
fn hex_encode_two_bytes_boundary() {
    assert_eq!(hex_encode(&[0x00, 0xFF]), "00ff");
    assert_eq!(hex_encode(&[0xFF, 0x00]), "ff00");
}

#[test]
fn hex_encode_produces_only_hex_digits() {
    let bytes: Vec<u8> = (0u8..=255).collect();
    let encoded = hex_encode(&bytes);
    assert!(
        encoded.chars().all(|c| c.is_ascii_hexdigit()),
        "hex_encode output should only contain hex digits"
    );
}

#[test]
fn hex_encode_produces_lowercase_output() {
    let bytes: Vec<u8> = (0u8..=255).collect();
    let encoded = hex_encode(&bytes);
    assert!(
        encoded
            .chars()
            .filter(char::is_ascii_alphabetic)
            .all(|c| c.is_ascii_lowercase()),
        "hex_encode output should be lowercase"
    );
}

// =======================================================================
// hex_encode proptest
// =======================================================================

#[test]
fn proptest_hex_encode_output_is_valid_lowercase_hex_double_length() {
    use proptest::prelude::*;
    proptest!(|(bytes in proptest::collection::vec(any::<u8>(), 0..100))| {
        let encoded = hex_encode(&bytes);
        prop_assert_eq!(encoded.len(), bytes.len() * 2);
        prop_assert!(encoded.chars().all(|c| c.is_ascii_hexdigit()));
        prop_assert!(encoded.chars().filter(char::is_ascii_alphabetic).all(|c| c.is_ascii_lowercase()));
    });
}

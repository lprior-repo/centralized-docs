use super::*;
use crate::state::UrlStateRaw;

// build_url_state_raw sets all fields correctly
#[test]
fn build_url_state_raw_sets_all_fields_correctly() {
    let result = build_url_state_raw([0xAA; 32], [0xBB; 32], 1_700_000_000, 200);
    assert_eq!(result.content_hash, [0xAA; 32]);
    assert_eq!(result.url_hash, [0xBB; 32]);
    assert_eq!(result.last_fetched_secs, 1_700_000_000);
    assert_eq!(result.status_code, 200);
    assert_eq!(std::mem::size_of::<UrlStateRaw>(), 120);
}

// build_url_state_raw zeroes reserved field
#[test]
fn build_url_state_raw_zeroes_reserved_field() {
    let result = build_url_state_raw([0x11; 32], [0x22; 32], 42, 0);
    assert_eq!(result.reserved, [0u8; 46]);
    assert_eq!(result.content_hash, [0x11; 32]);
    assert_eq!(result.url_hash, [0x22; 32]);
}

// build_url_state_raw output is 120 bytes
#[test]
fn build_url_state_raw_output_is_120_bytes() {
    let result = build_url_state_raw([0u8; 32], [0u8; 32], 0, 0);
    let bytes = result.to_bytes();
    assert_eq!(bytes.len(), 120);
}

// build_url_state_raw byte round-trip
#[test]
fn build_url_state_raw_roundtrips_through_bytes() {
    let original = build_url_state_raw([0xAA; 32], [0xBB; 32], 1_700_000_000, 301);
    let bytes = original.to_bytes();
    assert_eq!(bytes.len(), 120);
    let restored = UrlStateRaw::from_bytes(&bytes).expect("from_bytes should succeed");
    assert_eq!(restored.content_hash, [0xAA; 32]);
    assert_eq!(restored.url_hash, [0xBB; 32]);
    assert_eq!(restored.last_fetched_secs, 1_700_000_000);
    assert_eq!(restored.status_code, 301);
    assert_eq!(restored.reserved, [0u8; 46]);
}

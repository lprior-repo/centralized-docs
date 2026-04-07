//! Kani verification harnesses.

use super::pure::{build_file_state_raw, hash_payload};

#[kani::proof]
fn kani_file_state_raw_size() {
    let content: [u8; 32] = kani::any();
    let config: [u8; 32] = kani::any();
    let analysis: [u8; 32] = kani::any();
    let transform: [u8; 32] = kani::any();
    let chunk: [u8; 32] = kani::any();
    let now: u64 = kani::any();
    let raw = build_file_state_raw(content, config, analysis, transform, chunk, now);
    assert!(std::mem::size_of_val(&raw) == 200);
    assert!(raw.reserved == [0u8; 32]);
}

#[kani::proof]
fn kani_hash_payload_nonzero() {
    let len: usize = kani::any();
    kani::assume(len > 0 && len <= 32);
    let mut bytes = vec![0u8; len];
    for b in bytes.iter_mut() {
        *b = kani::any();
    }
    let hash = hash_payload(&bytes);
    assert!(hash != [0u8; 32]);
}

#[kani::proof]
fn kani_file_state_raw_preserves_fields() {
    let content: [u8; 32] = kani::any();
    let config: [u8; 32] = kani::any();
    let analysis: [u8; 32] = kani::any();
    let transform: [u8; 32] = kani::any();
    let chunk: [u8; 32] = kani::any();
    let now: u64 = kani::any();
    let raw = build_file_state_raw(content, config, analysis, transform, chunk, now);
    assert!(raw.content_hash == content);
    assert!(raw.config_hash == config);
    assert!(raw.analysis_hash == analysis);
    assert!(raw.transform_hash == transform);
    assert!(raw.chunk_hash == chunk);
    assert!(raw.last_processed_secs == now);
    assert!(raw.reserved == [0u8; 32]);
}

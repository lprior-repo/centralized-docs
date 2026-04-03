//! Fuzz target: compute_link_map_fingerprint with adversarial keys
//!
//! Verifies that compute_link_map_fingerprint never panics for any
//! HashMap<String, IdMapping> deserialized from arbitrary JSON input.

#![no_main]

use centralized_docs::assign::IdMapping;
use centralized_docs::transform::compute_link_map_fingerprint;
use libfuzzer_sys::fuzz_target;
use std::collections::HashMap;

fuzz_target!(|data: &[u8]| {
    // Must not panic — any input is valid to attempt parsing
    if let Ok(s) = std::str::from_utf8(data) {
        if let Ok(map) = serde_json::from_str::<HashMap<String, IdMapping>>(s) {
            let _ = compute_link_map_fingerprint(&map);
        }
    }
});

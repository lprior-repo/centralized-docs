//! Fuzz target: TransformArtifact deserialization
//!
//! Verifies that serde_json::from_str::<TransformArtifact>(input) never panics
//! for any arbitrary input bytes. Any panic inside deserialization is a bug.

#![no_main]

use centralized_docs::transform::TransformArtifact;
use libfuzzer_sys::fuzz_target;

fuzz_target!(|data: &[u8]| {
    // Must not panic — any input is valid to attempt parsing
    if let Ok(s) = std::str::from_utf8(data) {
        let _ = serde_json::from_str::<TransformArtifact>(s);
    }
});

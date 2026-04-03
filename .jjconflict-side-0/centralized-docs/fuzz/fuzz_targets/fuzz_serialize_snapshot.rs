//! Fuzz target: serialize_snapshot with arbitrary Snapshot-like input.
//!
//! Interprets raw bytes as field values for constructing a Snapshot,
//! then calls serialize_snapshot. Validates that the function never panics
//! and always produces valid rkyv output.
//!
//! Corpus seeds:
//! - Minimal Snapshot (empty pages)
//! - Maximal Snapshot (1000 pages, long URLs, long titles)
//! - Snapshot with non-ASCII target_url
//! - Snapshot with zero timestamp

#![no_main]
use libfuzzer_sys::fuzz_target;

fuzz_target!(|data: &[u8]| {
    // Strategy: Interpret data as field values, construct Snapshot, serialize.
    // serialize_snapshot takes &Snapshot (Rust type), so the primary risk
    // is in the rkyv serializer itself. This validates that assumption.
    //
    // For RED phase: Implementation is todo!(). When implementation arrives,
    // this constructs a Snapshot from the fuzz data and serializes it.

    let _ = data; // RED phase: implementation pending
});

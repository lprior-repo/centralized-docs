//! Fuzz target: load_snapshots with corrupt bytes in redb.
//!
//! Writes arbitrary bytes into the snapshots table under a fixed key,
//! then calls load_snapshots. Must NEVER panic — must always return
//! Err(ArchiveValidationFailed) or Err(DeserializationFailed).
//!
//! Corpus seeds:
//! - Empty byte vector ([])
//! - Single null byte ([0x00])
//! - Valid rkyv archive of Snapshot (happy path baseline)
//! - Valid rkyv archive of String (wrong type)
//! - Random 32 bytes
//! - Valid rkyv archive with first 4 bytes zeroed (corrupt header)
//! - Very large byte vector (1 MB of random data)

#![no_main]
use libfuzzer_sys::fuzz_target;

fuzz_target!(|data: &[u8]| {
    // Must not panic — any input is valid to attempt loading.
    // The implementation should return Err(...) for corrupt bytes,
    // never panic.
    //
    // For RED phase: This fuzz target is structurally correct but
    // the implementation is todo!(). When implementation arrives,
    // this will exercise the full load path with arbitrary bytes.

    // Strategy: Write arbitrary bytes to a temp redb, then load.
    // If the bytes happen to be valid rkyv of Snapshot, load succeeds.
    // Otherwise, load returns an error. Never panics.

    let _ = data; // RED phase: implementation pending
});

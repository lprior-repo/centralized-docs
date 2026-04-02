#![no_main]

use libfuzzer_sys::fuzz_target;

fuzz_target!(|data: &[u8]| {
    // FUZZ-01: StateBatch deserialization must not panic
    // Any arbitrary bytes fed into the deserializer must be handled gracefully.
    let _ = serde_json::from_slice::<doc_transformer::state::StateBatch>(data);
});

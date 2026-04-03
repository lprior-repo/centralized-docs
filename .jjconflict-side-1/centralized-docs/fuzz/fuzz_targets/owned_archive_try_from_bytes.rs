#![no_main]
use centralized_docs::persisted::PersistedAnalyzeResult;
use centralized_docs::state::bulk_load::OwnedArchive;
use libfuzzer_sys::fuzz_target;

fuzz_target!(|data: &[u8]| {
    // OwnedArchive::try_from_bytes must not panic on any input.
    // It should return either Ok or CorruptPayload.
    let key: [u8; 32] = [0x42; 32];
    let bytes: Box<[u8]> = data.to_vec().into_boxed_slice();
    let _ = OwnedArchive::<PersistedAnalyzeResult>::try_from_bytes("analysis_outputs", &key, bytes);
});

#![no_main]

use libfuzzer_sys::fuzz_target;

fuzz_target!(|data: &[u8]| {
    // We only need to ensure the fuzz target compiles.
    // The actual stdio parser will be fuzzed when implemented.
    if let Ok(s) = std::str::from_utf8(data) {
        let _ = doc_transformer::mcp::SearchDocsParams {
            query: s.to_string(),
            limit: 10,
        };
    }
});

#![no_main]

use libfuzzer_sys::fuzz_target;

fuzz_target!(|data: &[u8]| {
    // FUZZ-02: record_file_hash input validation must not panic
    // Split arbitrary bytes into two strings at the first NUL byte.
    // If no NUL byte, use entire input as relative_path with empty hash.
    let (relative_path, content_hash) = match data.iter().position(|&b| b == 0) {
        Some(pos) => {
            let path = String::from_utf8_lossy(&data[..pos]).to_string();
            let hash = String::from_utf8_lossy(&data[pos + 1..]).to_string();
            (path, hash)
        }
        None => (String::from_utf8_lossy(data).to_string(), String::new()),
    };

    let dir = match tempfile::TempDir::new() {
        Ok(d) => d,
        Err(_) => return,
    };

    if let Ok(mut db) = doc_transformer::state::StateDb::new(dir.path()) {
        // Must not panic on any input
        let _ = db.record_file_hash(&relative_path, &content_hash);
    }
});

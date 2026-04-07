//! Error variant construction and display tests.

use super::*;

// =======================================================================
// BulkLoadError::StorageError variant (construction test)
// =======================================================================

#[test]
fn bulk_load_error_storage_error_variant_carries_table_and_message() {
    let err = BulkLoadError::StorageError {
        table: "analysis_outputs",
        message: "I/O error reading page 42".to_string(),
    };
    assert!(
        matches!(
            err,
            BulkLoadError::StorageError {
                table: "analysis_outputs",
                message: _,
            }
        ),
        "StorageError must match with exact table name"
    );
    let display = err.to_string();
    assert!(display.contains("analysis_outputs"), "got: {display}");
    assert!(
        display.contains("I/O error reading page 42"),
        "got: {display}"
    );
}

#[test]
fn storage_error_variant_carries_table_and_message() {
    let err = BulkLoadError::StorageError {
        table: "<begin_read>",
        message: "simulated storage failure".to_string(),
    };
    assert!(
        matches!(
            &err,
            BulkLoadError::StorageError {
                table: "<begin_read>",
                message,
            } if message == "simulated storage failure"
        ),
        "expected StorageError with exact table and message, got {err:?}"
    );
    let display = err.to_string();
    assert!(display.contains("<begin_read>"), "display: {display}");
    assert!(
        display.contains("simulated storage failure"),
        "display: {display}"
    );
}

// =======================================================================
// BulkLoadError::TableOpen variant
// =======================================================================

#[test]
fn table_open_error_variant_carries_table_and_message() {
    let err = BulkLoadError::TableOpen {
        table: "analysis_outputs",
        message: "table does not exist".to_string(),
    };
    assert!(
        matches!(
            &err,
            BulkLoadError::TableOpen {
                table: "analysis_outputs",
                message,
            } if message == "table does not exist"
        ),
        "expected TableOpen with exact fields, got {err:?}"
    );
    let display = err.to_string();
    assert!(display.contains("analysis_outputs"), "display: {display}");
}

// =======================================================================
// BulkLoadError::CorruptPayload variant
// =======================================================================

#[test]
fn corrupt_payload_error_variant_carries_table_key_and_message() {
    let key: [u8; 32] = [0xAB; 32];
    let err = BulkLoadError::CorruptPayload {
        table: "transform_outputs",
        key_hex: hex_encode(&key),
        message: "bytecheck failed".to_string(),
    };
    assert!(
        matches!(
            &err,
            BulkLoadError::CorruptPayload {
                table: "transform_outputs",
                key_hex,
                message,
            } if key_hex == "abababababababababababababababababababababababababababababababab"
                && message == "bytecheck failed"
        ),
        "expected CorruptPayload with exact fields, got {err:?}"
    );
    let display = err.to_string();
    assert!(display.contains("transform_outputs"), "display: {display}");
    assert!(display.contains("ababab"), "display: {display}");
}

// =======================================================================
// StateLoadError variants
// =======================================================================

#[test]
fn utf8_key_error_variant_carries_lossy_bytes() {
    let lossy_value = "\u{FFFD}\u{FFFD} invalid key".to_string();
    let err = StateLoadError::Utf8KeyError {
        bytes_lossy: lossy_value.clone(),
    };
    assert!(
        matches!(
            &err,
            StateLoadError::Utf8KeyError { bytes_lossy }
            if bytes_lossy == "\u{FFFD}\u{FFFD} invalid key"
        ),
        "expected Utf8KeyError with exact bytes_lossy, got {err:?}"
    );
    let display = err.to_string();
    assert!(display.contains("invalid key"), "display: {display}");
}

#[test]
fn malformed_row_error_variant_fields_are_accessible() {
    let err = StateLoadError::MalformedRow {
        key: "test_key.md".to_string(),
        actual: 50,
        expected: 200,
    };
    assert!(
        matches!(
            &err,
            StateLoadError::MalformedRow {
                key,
                actual: 50,
                expected: 200,
            } if key == "test_key.md"
        ),
        "expected MalformedRow with exact fields, got {err:?}"
    );
    let display = err.to_string();
    assert!(display.contains("test_key.md"), "display: {display}");
    assert!(display.contains("50"), "display: {display}");
    assert!(display.contains("200"), "display: {display}");
}

#[test]
fn backend_error_variant_fields_are_accessible() {
    let err = StateLoadError::BackendError {
        operation: "table_iter",
        message: "io error during iteration".to_string(),
    };
    assert!(
        matches!(
            &err,
            StateLoadError::BackendError {
                operation: "table_iter",
                message,
            } if message == "io error during iteration"
        ),
        "expected BackendError with exact fields, got {err:?}"
    );
    let display = err.to_string();
    assert!(display.contains("table_iter"), "display: {display}");
    assert!(
        display.contains("io error during iteration"),
        "display: {display}"
    );
}

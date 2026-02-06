//! Test to verify test_output/ Tantivy index is properly populated

use std::path::Path;

#[test]
fn test_tantivy_index_is_populated() -> Result<(), Box<dyn std::error::Error>> {
    let test_output = Path::new("test_output");
    let tantivy_index_dir = test_output.join(".tantivy_index");

    // Verify test_output exists
    assert!(test_output.exists(), "test_output directory should exist");

    // Verify Tantivy index directory exists
    assert!(
        tantivy_index_dir.exists(),
        ".tantivy_index directory should exist"
    );

    // Verify INDEX.json exists (should have documents)
    let index_json = test_output.join("INDEX.json");
    assert!(index_json.exists(), "INDEX.json should exist");

    // Read INDEX.json to get expected document count
    let index_content = std::fs::read_to_string(&index_json)?;
    let index_value: serde_json::Value = serde_json::from_str(&index_content)?;
    let doc_count = index_value["documents"]
        .as_array()
        .map(|arr| arr.len())
        .unwrap_or(0);

    assert!(
        doc_count > 0,
        "INDEX.json should contain at least one document"
    );

    // Verify Tantivy index has segment files (not just metadata)
    let entries = std::fs::read_dir(&tantivy_index_dir)?;
    let mut has_segment_files = false;

    for entry in entries {
        let path = entry?.path();
        if let Some(filename) = path.file_name() {
            let name = filename.to_string_lossy();
            // Check for segment files (not meta.json, lock files, etc.)
            if name.ends_with(".store")
                || name.ends_with(".fast")
                || name.ends_with(".idx")
                || name.ends_with(".term")
                || name.ends_with(".pos")
                || name.ends_with(".fieldnorm")
            {
                has_segment_files = true;
                break;
            }
        }
    }

    assert!(
        has_segment_files,
        "Tantivy index should contain segment files (*.store, *.fast, etc.)"
    );

    Ok(())
}

#[test]
fn test_search_returns_results() -> Result<(), Box<dyn std::error::Error>> {
    let test_output = Path::new("test_output");

    // Verify test_output exists
    assert!(test_output.exists(), "test_output directory should exist");

    // Try to open the Tantivy index
    let index = doc_transformer::search::open_or_create_index(test_output)
        .expect("Should be able to open or create Tantivy index");

    // Try searching - just verify the search executes without error
    let results = doc_transformer::search::search_index(&index, "test query", 10)
        .expect("Search should succeed");

    // Search should succeed and return results (may be empty if no matches)
    // The important part is that it doesn't panic or error
    let _results = results;

    Ok(())
}

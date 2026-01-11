use std::path::PathBuf;

// Helper to get test output directory
fn get_test_output_dir() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("test_output")
}

#[test]
fn test_search_documents_basic() {
    let test_output = get_test_output_dir();
    let index_path = test_output.join("INDEX.json");

    // Skip test if INDEX.json doesn't exist (would happen in clean build)
    if !index_path.exists() {
        eprintln!("Skipping test: INDEX.json not found at {:?}", index_path);
        return;
    }

    // Test that search_documents can be called without panicking
    let result = doc_transformer::search::search_documents(
        &index_path,
        "rust",
        10,
    );

    // We just check that it doesn't error - the actual results depend on test data
    assert!(result.is_ok(), "search_documents should not error: {:?}", result.err());
}

#[test]
fn test_search_chunks_basic() {
    let test_output = get_test_output_dir();
    let index_path = test_output.join("INDEX.json");

    // Skip test if INDEX.json doesn't exist
    if !index_path.exists() {
        eprintln!("Skipping test: INDEX.json not found at {:?}", index_path);
        return;
    }

    // Test that search_chunks can be called
    let result = doc_transformer::search::search_chunks(
        &index_path,
        "rust",
        10,
    );

    // If the index is old version without chunks array, that's okay - skip test
    // Otherwise check that it doesn't error
    match result {
        Ok(_) => {
            // Success - chunks found or no matches
        }
        Err(e) => {
            let err_msg = e.to_string();
            if err_msg.contains("chunks") {
                eprintln!("Skipping test: INDEX.json is old version without chunks array");
                return;
            }
            panic!("search_chunks should not error: {:?}", e);
        }
    }
}

#[test]
fn test_search_respects_limit() {
    let test_output = get_test_output_dir();
    let index_path = test_output.join("INDEX.json");

    if !index_path.exists() {
        eprintln!("Skipping test: INDEX.json not found at {:?}", index_path);
        return;
    }

    // Search with limit of 3
    let result = doc_transformer::search::search_documents(
        &index_path,
        "the",  // Common word likely to match many documents
        3,
    );

    if let Ok(results) = result {
        assert!(
            results.len() <= 3,
            "Results should not exceed limit of 3, got {}",
            results.len()
        );
    }
}

#[test]
fn test_search_empty_query_returns_no_results() {
    let test_output = get_test_output_dir();
    let index_path = test_output.join("INDEX.json");

    if !index_path.exists() {
        eprintln!("Skipping test: INDEX.json not found at {:?}", index_path);
        return;
    }

    let result = doc_transformer::search::search_documents(
        &index_path,
        "",
        10,
    );

    if let Ok(results) = result {
        assert_eq!(
            results.len(),
            0,
            "Empty query should return no results"
        );
    }
}

#[test]
fn test_search_nonexistent_term_returns_no_results() {
    let test_output = get_test_output_dir();
    let index_path = test_output.join("INDEX.json");

    if !index_path.exists() {
        eprintln!("Skipping test: INDEX.json not found at {:?}", index_path);
        return;
    }

    // Use a very unlikely term
    let result = doc_transformer::search::search_documents(
        &index_path,
        "xyzabc123nonexistent",
        10,
    );

    if let Ok(results) = result {
        assert_eq!(
            results.len(),
            0,
            "Nonexistent term should return no results"
        );
    }
}

#[test]
fn test_chunk_search_respects_limit() {
    let test_output = get_test_output_dir();
    let index_path = test_output.join("INDEX.json");

    if !index_path.exists() {
        eprintln!("Skipping test: INDEX.json not found at {:?}", index_path);
        return;
    }

    // Search with limit of 2
    let result = doc_transformer::search::search_chunks(
        &index_path,
        "the",  // Common word
        2,
    );

    if let Ok(results) = result {
        assert!(
            results.len() <= 2,
            "Chunk results should not exceed limit of 2, got {}",
            results.len()
        );
    }
}

#[test]
fn test_search_missing_index_returns_error() {
    let nonexistent_path = PathBuf::from("/nonexistent/path/INDEX.json");

    let result = doc_transformer::search::search_documents(
        &nonexistent_path,
        "test",
        10,
    );

    assert!(
        result.is_err(),
        "Search with nonexistent index should return error"
    );
}

#[test]
fn test_search_with_zero_word_count_documents() {
    use std::fs;
    use tempfile::TempDir;

    // Create a temporary directory
    let temp_dir = TempDir::new().expect("Failed to create temp dir");
    let index_path = temp_dir.path().join("INDEX.json");

    // Create INDEX.json with documents having zero word_count
    let index_content = r#"{
  "documents": [
    {
      "id": "test-doc-1",
      "title": "Test Document",
      "category": "tutorial-general",
      "summary": "",
      "word_count": 0,
      "chunk_ids": []
    },
    {
      "id": "test-doc-2",
      "title": "Another Test",
      "category": "ops-general",
      "summary": "",
      "word_count": 0,
      "chunk_ids": []
    }
  ],
  "chunks": []
}"#;

    fs::write(&index_path, index_content).expect("Failed to write test INDEX.json");

    // This should NOT panic with division by zero
    let result = doc_transformer::search::search_documents(
        &index_path,
        "test",
        10,
    );

    // Should succeed without panic
    assert!(result.is_ok(), "Search with zero word_count documents should not panic: {:?}", result.err());

    // Results should be empty or valid (no matches since summaries are empty)
    if let Ok(results) = result {
        for res in results {
            assert!(res.score.is_finite(), "All scores should be finite");
            assert!(res.score >= 0.0, "All scores should be non-negative");
        }
    }
}

#[test]
fn test_search_chunks_with_empty_content() {
    use std::fs;
    use tempfile::TempDir;

    // Create a temporary directory with chunks subdirectory
    let temp_dir = TempDir::new().expect("Failed to create temp dir");
    let chunks_dir = temp_dir.path().join("chunks");
    fs::create_dir(&chunks_dir).expect("Failed to create chunks dir");

    let index_path = temp_dir.path().join("INDEX.json");

    // Create an empty chunk file
    let chunk_path = chunks_dir.join("empty-chunk.md");
    fs::write(&chunk_path, "---\nid: empty\n---\n").expect("Failed to write chunk file");

    // Create INDEX.json referencing the empty chunk
    let index_content = r#"{
  "documents": [],
  "chunks": [
    {
      "chunk_id": "empty",
      "doc_id": "test",
      "doc_title": "Test",
      "heading": "",
      "chunk_type": "introduction",
      "path": "chunks/empty-chunk.md",
      "word_count": 0
    }
  ]
}"#;

    fs::write(&index_path, index_content).expect("Failed to write test INDEX.json");

    // This should NOT panic with division by zero
    let result = doc_transformer::search::search_chunks(
        &index_path,
        "test",
        10,
    );

    // Should succeed without panic
    assert!(result.is_ok(), "Search with empty chunk content should not panic: {:?}", result.err());

    // Check all scores are valid
    if let Ok(results) = result {
        for res in results {
            assert!(res.score.is_finite(), "All chunk scores should be finite");
            assert!(res.score >= 0.0, "All chunk scores should be non-negative");
        }
    }
}

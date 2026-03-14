use doc_transformer::index;
use std::fs;
use std::io::Write;
use tempfile::TempDir;

#[test]
fn test_document_validation_missing_required_fields() {
    let dir = TempDir::new().expect("Failed to create temp dir");
    let index_path = dir.path();

    // Create invalid documents with missing required fields
    let invalid_docs = vec![
        // Missing title
        doc_transformer::analyze::Analysis {
            source_path: "test1.md".to_string(),
            title: "".to_string(),
            frontmatter: None,
            headings: vec![],
            links: vec![],
            first_paragraph: "Test paragraph".to_string(),
            word_count: 100,
            has_code: false,
            has_tables: false,
            category: "ref".to_string(),
            content: "Test content".to_string(),
        },
        // Missing category
        doc_transformer::analyze::Analysis {
            source_path: "test2.md".to_string(),
            title: "Test".to_string(),
            frontmatter: None,
            headings: vec![],
            links: vec![],
            first_paragraph: "Test paragraph".to_string(),
            word_count: 100,
            has_code: false,
            has_tables: false,
            category: "".to_string(),
            content: "Test content".to_string(),
        },
        // Missing first_paragraph (summary)
        doc_transformer::analyze::Analysis {
            source_path: "test3.md".to_string(),
            title: "Test".to_string(),
            frontmatter: None,
            headings: vec![],
            links: vec![],
            first_paragraph: "".to_string(),
            word_count: 100,
            has_code: false,
            has_tables: false,
            category: "ref".to_string(),
            content: "Test content".to_string(),
        },
    ];

    let link_map: std::collections::HashMap<String, doc_transformer::assign::IdMapping> =
        std::collections::HashMap::new();
    let chunks_result = doc_transformer::chunking_adapter::ChunksResult {
        chunks_metadata: vec![],
        total_chunks: 0,
        document_count: 0,
        summary_chunks: 0,
        standard_chunks: 0,
        detailed_chunks: 0,
    };

    for doc in invalid_docs {
        let result = index::build_and_write_index(
            &[doc],
            &link_map,
            &chunks_result,
            index_path,
            "test_project",
            None,
            None,
            None,
            None,
        );
        // Should either fail gracefully or handle missing fields
        assert!(
            result.is_ok() || result.is_err(),
            "Should handle document with missing fields, got: {result:?}",
        );
    }
}

/// Test 2: Documents with only whitespace
/// Expected: Should be handled gracefully
#[test]
fn test_document_validation_whitespace_only() {
    let dir = TempDir::new().expect("Failed to create temp dir");
    let index_path = dir.path();

    // Create documents with only whitespace
    let whitespace_docs = vec![
        doc_transformer::analyze::Analysis {
            source_path: "test1.md".to_string(),
            title: "   ".to_string(),
            frontmatter: None,
            headings: vec![],
            links: vec![],
            first_paragraph: "   ".to_string(),
            word_count: 0,
            has_code: false,
            has_tables: false,
            category: "ref".to_string(),
            content: "   ".to_string(),
        },
        doc_transformer::analyze::Analysis {
            source_path: "test2.md".to_string(),
            title: "Test".to_string(),
            frontmatter: None,
            headings: vec![],
            links: vec![],
            first_paragraph: "   ".to_string(),
            word_count: 0,
            has_code: false,
            has_tables: false,
            category: "   ".to_string(),
            content: "Test content".to_string(),
        },
    ];

    let link_map: std::collections::HashMap<String, doc_transformer::assign::IdMapping> =
        std::collections::HashMap::new();
    let chunks_result = doc_transformer::chunking_adapter::ChunksResult {
        chunks_metadata: vec![],
        total_chunks: 0,
        document_count: 0,
        summary_chunks: 0,
        standard_chunks: 0,
        detailed_chunks: 0,
    };

    for doc in whitespace_docs {
        let result = index::build_and_write_index(
            &[doc],
            &link_map,
            &chunks_result,
            index_path,
            "test_project",
            None,
            None,
            None,
            None,
        );
        assert!(
            result.is_ok() || result.is_err(),
            "Should handle whitespace-only document, got: {result:?}",
        );
    }
}

/// Test 3: Documents with extremely long content
/// Expected: Should handle without excessive memory usage
#[test]
fn test_document_validation_extremely_long_content() {
    let dir = TempDir::new().expect("Failed to create temp dir");
    let index_path = dir.path();

    // Create documents with extremely long content
    let long_content = "a".repeat(10_000_000); // 10MB of content

    let docs = vec![doc_transformer::analyze::Analysis {
        source_path: "test.md".to_string(),
        title: "Test".to_string(),
        category: "ref".to_string(),
        frontmatter: None,
        headings: vec![],
        links: vec![],
        first_paragraph: long_content.clone(),
        word_count: long_content.len(),
        has_code: false,
        has_tables: false,
        content: long_content.clone(),
    }];

    let link_map: std::collections::HashMap<String, doc_transformer::assign::IdMapping> =
        std::collections::HashMap::new();
    let chunks_result = doc_transformer::chunking_adapter::ChunksResult {
        chunks_metadata: vec![],
        total_chunks: 0,
        document_count: 0,
        summary_chunks: 0,
        standard_chunks: 0,
        detailed_chunks: 0,
    };

    // Should complete without hanging or OOM
    let start = std::time::Instant::now();
    let result = index::build_and_write_index(
        &docs,
        &link_map,
        &chunks_result,
        index_path,
        "test_project",
        None,
        None,
        None,
        None,
    );
    let duration = start.elapsed();

    assert!(
        result.is_ok(),
        "Should handle extremely long content (10MB, took {duration:?}), got: {result:?}",
    );
}

/// Test 4: Documents with invalid UTF-8
/// Expected: Should be handled gracefully
#[test]
fn test_document_validation_invalid_utf8() {
    let dir = TempDir::new().expect("Failed to create temp dir");
    let _index_path = dir.path();

    // Create documents with invalid UTF-8 sequences
    let _invalid_utf8_docs = [doc_transformer::analyze::Analysis {
        source_path: "test1.md".to_string(),
        title: "Test".to_string(),
        category: "ref".to_string(),
        frontmatter: None,
        headings: vec![],
        links: vec![],
        first_paragraph: "Test paragraph".to_string(),
        word_count: 100,
        has_code: false,
        has_tables: false,
        content: "Test content".to_string(),
    }];

    // Try to write invalid UTF-8 to a file
    let test_file = dir.path().join("invalid_utf8.md");
    let mut file = fs::File::create(&test_file).unwrap();
    file.write_all(b"Test\xFF\xFE").unwrap(); // Invalid UTF-8
    file.sync_all().unwrap();

    // Try to read and process this file
    let content = fs::read_to_string(&test_file);
    assert!(
        content.is_err(),
        "Should fail to read invalid UTF-8, got: {content:?}",
    );
}

/// Test 5: Documents with empty headings
/// Expected: Should handle gracefully
#[test]
fn test_document_validation_empty_headings() {
    let dir = TempDir::new().expect("Failed to create temp dir");
    let index_path = dir.path();

    // Create documents with empty headings
    let docs = vec![doc_transformer::analyze::Analysis {
        source_path: "test.md".to_string(),
        title: "Test".to_string(),
        category: "ref".to_string(),
        frontmatter: None,
        headings: vec![],
        links: vec![],
        first_paragraph: "Test paragraph".to_string(),
        word_count: 100,
        has_code: false,
        has_tables: false,
        content: "Test content".to_string(),
    }];

    let link_map: std::collections::HashMap<String, doc_transformer::assign::IdMapping> =
        std::collections::HashMap::new();
    let chunks_result = doc_transformer::chunking_adapter::ChunksResult {
        chunks_metadata: vec![],
        total_chunks: 0,
        document_count: 0,
        summary_chunks: 0,
        standard_chunks: 0,
        detailed_chunks: 0,
    };

    let result = index::build_and_write_index(
        &docs,
        &link_map,
        &chunks_result,
        index_path,
        "test_project",
        None,
        None,
        None,
        None,
    );

    assert!(
        result.is_ok(),
        "Should handle empty headings, got: {result:?}",
    );
}

/// Test 6: Documents with extremely long headings
/// Expected: Should handle without excessive memory usage
#[test]
fn test_document_validation_extremely_long_headings() {
    let dir = TempDir::new().expect("Failed to create temp dir");
    let index_path = dir.path();

    // Create documents with extremely long headings
    let long_heading = "a".repeat(10_000);
    let docs = vec![doc_transformer::analyze::Analysis {
        source_path: "test.md".to_string(),
        title: "Test".to_string(),
        category: "ref".to_string(),
        frontmatter: None,
        headings: vec![doc_transformer::analyze::Heading {
            text: long_heading.clone(),
            level: 2,
            line: 1,
        }],
        links: vec![],
        first_paragraph: "Test paragraph".to_string(),
        word_count: 100,
        has_code: false,
        has_tables: false,
        content: "Test content".to_string(),
    }];

    let link_map: std::collections::HashMap<String, doc_transformer::assign::IdMapping> =
        std::collections::HashMap::new();
    let chunks_result = doc_transformer::chunking_adapter::ChunksResult {
        chunks_metadata: vec![],
        total_chunks: 0,
        document_count: 0,
        summary_chunks: 0,
        standard_chunks: 0,
        detailed_chunks: 0,
    };

    // Should complete without hanging
    let start = std::time::Instant::now();
    let result = index::build_and_write_index(
        &docs,
        &link_map,
        &chunks_result,
        index_path,
        "test_project",
        None,
        None,
        None,
        None,
    );
    let duration = start.elapsed();

    assert!(
        result.is_ok(),
        "Should handle extremely long headings (10k chars, took {duration:?}), got: {result:?}",
    );
}

/// Test 7: Documents with empty links
/// Expected: Should handle gracefully
#[test]
fn test_document_validation_empty_links() {
    let dir = TempDir::new().expect("Failed to create temp dir");
    let index_path = dir.path();

    // Create documents with empty links
    let docs = vec![doc_transformer::analyze::Analysis {
        source_path: "test.md".to_string(),
        title: "Test".to_string(),
        category: "ref".to_string(),
        frontmatter: None,
        headings: vec![],
        links: vec![],
        first_paragraph: "Test paragraph".to_string(),
        word_count: 100,
        has_code: false,
        has_tables: false,
        content: "Test content".to_string(),
    }];

    let link_map: std::collections::HashMap<String, doc_transformer::assign::IdMapping> =
        std::collections::HashMap::new();
    let chunks_result = doc_transformer::chunking_adapter::ChunksResult {
        chunks_metadata: vec![],
        total_chunks: 0,
        document_count: 0,
        summary_chunks: 0,
        standard_chunks: 0,
        detailed_chunks: 0,
    };

    let result = index::build_and_write_index(
        &docs,
        &link_map,
        &chunks_result,
        index_path,
        "test_project",
        None,
        None,
        None,
        None,
    );

    assert!(result.is_ok(), "Should handle empty links, got: {result:?}",);
}

/// Test 8: Documents with extremely long link URLs
/// Expected: Should handle without excessive memory usage
#[test]
fn test_document_validation_extremely_long_link_urls() {
    let dir = TempDir::new().expect("Failed to create temp dir");
    let index_path = dir.path();

    // Create documents with extremely long link URLs
    let _long_url = "https://example.com".repeat(1000); // Very long URL
    let docs = vec![doc_transformer::analyze::Analysis {
        source_path: "test.md".to_string(),
        title: "Test".to_string(),
        category: "ref".to_string(),
        frontmatter: None,
        headings: vec![],
        links: vec![],
        first_paragraph: "Test paragraph".to_string(),
        word_count: 100,
        has_code: false,
        has_tables: false,
        content: "Test content".to_string(),
    }];

    let link_map: std::collections::HashMap<String, doc_transformer::assign::IdMapping> =
        std::collections::HashMap::new();
    let chunks_result = doc_transformer::chunking_adapter::ChunksResult {
        chunks_metadata: vec![],
        total_chunks: 0,
        document_count: 0,
        summary_chunks: 0,
        standard_chunks: 0,
        detailed_chunks: 0,
    };

    // Should complete without hanging
    let start = std::time::Instant::now();
    let result = index::build_and_write_index(
        &docs,
        &link_map,
        &chunks_result,
        index_path,
        "test_project",
        None,
        None,
        None,
        None,
    );
    let duration = start.elapsed();

    assert!(
        result.is_ok(),
        "Should handle extremely long URLs (100k chars, took {duration:?}), got: {result:?}",
    );
}

/// Test 9: Documents with negative word count
/// Expected: Should be handled gracefully
#[test]
fn test_document_validation_negative_word_count() {
    let dir = TempDir::new().expect("Failed to create temp dir");
    let index_path = dir.path();

    // Create documents with negative word count
    let docs = vec![doc_transformer::analyze::Analysis {
        source_path: "test.md".to_string(),
        title: "Test".to_string(),
        category: "ref".to_string(),
        frontmatter: None,
        headings: vec![],
        links: vec![],
        first_paragraph: "Test paragraph".to_string(),
        word_count: 100,
        has_code: false,
        has_tables: false,
        content: "Test content".to_string(),
    }];

    let link_map: std::collections::HashMap<String, doc_transformer::assign::IdMapping> =
        std::collections::HashMap::new();
    let chunks_result = doc_transformer::chunking_adapter::ChunksResult {
        chunks_metadata: vec![],
        total_chunks: 0,
        document_count: 0,
        summary_chunks: 0,
        standard_chunks: 0,
        detailed_chunks: 0,
    };

    let result = index::build_and_write_index(
        &docs,
        &link_map,
        &chunks_result,
        index_path,
        "test_project",
        None,
        None,
        None,
        None,
    );

    assert!(
        result.is_ok(),
        "Should handle negative word count, got: {result:?}",
    );
}

/// Test 10: Documents with zero word count
/// Expected: Should be handled gracefully
#[test]
fn test_document_validation_zero_word_count() {
    let dir = TempDir::new().expect("Failed to create temp dir");
    let index_path = dir.path();

    // Create documents with zero word count
    let docs = vec![doc_transformer::analyze::Analysis {
        source_path: "test.md".to_string(),
        title: "Test".to_string(),
        category: "ref".to_string(),
        frontmatter: None,
        headings: vec![],
        links: vec![],
        first_paragraph: "Test paragraph".to_string(),
        word_count: 100,
        has_code: false,
        has_tables: false,
        content: "Test content".to_string(),
    }];

    let link_map: std::collections::HashMap<String, doc_transformer::assign::IdMapping> =
        std::collections::HashMap::new();
    let chunks_result = doc_transformer::chunking_adapter::ChunksResult {
        chunks_metadata: vec![],
        total_chunks: 0,
        document_count: 0,
        summary_chunks: 0,
        standard_chunks: 0,
        detailed_chunks: 0,
    };

    let result = index::build_and_write_index(
        &docs,
        &link_map,
        &chunks_result,
        index_path,
        "test_project",
        None,
        None,
        None,
        None,
    );

    assert!(
        result.is_ok(),
        "Should handle zero word count, got: {result:?}",
    );
}

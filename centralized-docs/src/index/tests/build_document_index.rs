//! Tests for `build_document_index`.

use std::collections::HashMap;

use super::*;

#[test]
fn test_build_document_index_basic() {
    let analyses = vec![make_analysis(
        "docs/tutorial/rust-guide.md",
        "Rust Guide",
        "tutorial",
        vec![
            make_heading(1, "Introduction"),
            make_heading(2, "Getting Started"),
        ],
        "Rust is a systems language.",
        100,
    )];
    let link_map = make_link_map(vec![(
        "docs/tutorial/rust-guide.md",
        "tutorial/rust-guide",
        "tutorial-rust-guide.md",
        "tutorial",
    )]);
    let result = build_document_index(&analyses, &link_map, &make_empty_chunks_result());
    assert_eq!(result.documents.len(), 1);
    assert_eq!(result.documents[0].id, "tutorial/rust-guide");
    assert_eq!(result.documents[0].title, "Rust Guide");
    assert_eq!(result.documents[0].category, "tutorial");
    assert_eq!(result.documents[0].path, "docs/tutorial-rust-guide.md");
}

#[test]
fn test_build_document_index_with_chunk_ids() {
    let analyses = vec![make_analysis(
        "docs/concept/design.md",
        "Design Patterns",
        "concept",
        vec![make_heading(1, "Patterns")],
        "Design patterns overview.",
        50,
    )];
    let link_map = make_link_map(vec![(
        "docs/concept/design.md",
        "concept/design",
        "concept-design.md",
        "concept",
    )]);
    let chunks_result = ChunksResult {
        total_chunks: 2,
        document_count: 1,
        chunks_metadata: vec![
            make_chunk(
                "concept/design#0-standard",
                "concept/design",
                "Design Patterns",
                "content 1",
                None,
                ChunkLevel::Standard,
            ),
            make_chunk(
                "concept/design#1-standard",
                "concept/design",
                "Design Patterns",
                "content 2",
                None,
                ChunkLevel::Standard,
            ),
        ],
        summary_chunks: 0,
        standard_chunks: 2,
        detailed_chunks: 0,
    };
    let result = build_document_index(&analyses, &link_map, &chunks_result);
    assert_eq!(result.documents[0].chunk_ids.len(), 2);
    assert_eq!(
        result.documents[0].chunk_ids[0],
        "concept/design#0-standard"
    );
    assert_eq!(
        result.documents[0].chunk_ids[1],
        "concept/design#1-standard"
    );
}

#[test]
fn test_build_document_index_empty_analyses() {
    let link_map: HashMap<String, IdMapping> = HashMap::new();
    let result = build_document_index(&[], &link_map, &make_empty_chunks_result());
    assert!(result.documents.is_empty());
    assert!(result.keywords.is_empty());
    assert!(result.document_tags.is_empty());
}

#[test]
fn test_build_document_index_no_matching_link_map() {
    let analyses = vec![make_analysis(
        "docs/orphan.md",
        "Orphan Doc",
        "concept",
        vec![],
        "No mapping exists.",
        10,
    )];
    let link_map: HashMap<String, IdMapping> = HashMap::new();
    let result = build_document_index(&analyses, &link_map, &make_empty_chunks_result());
    assert!(result.documents.is_empty());
}

#[test]
fn test_build_document_index_keywords_extraction() {
    let analyses = vec![make_analysis(
        "docs/tutorial/rust-guide.md",
        "Rust Guide",
        "tutorial",
        vec![
            make_heading(1, "Introduction"),
            make_heading(2, "Advanced Programming"),
            make_heading(3, "Error Handling"),
        ],
        "Rust guide intro.",
        200,
    )];
    let link_map = make_link_map(vec![(
        "docs/tutorial/rust-guide.md",
        "tutorial/rust-guide",
        "tutorial-rust-guide.md",
        "tutorial",
    )]);
    let result = build_document_index(&analyses, &link_map, &make_empty_chunks_result());
    assert!(!result.keywords.is_empty());
    let all_keywords: Vec<_> = result.keywords.keys().collect();
    assert!(all_keywords
        .iter()
        .any(|k| **k == "introduction" || **k == "advanced"));
}

#[test]
fn test_build_document_index_document_tags() {
    let analyses = vec![make_analysis(
        "docs/ref/api.md",
        "API Reference",
        "ref",
        vec![
            make_heading(1, "HTTP Endpoints"),
            make_heading(2, "Functions"),
        ],
        "API docs.",
        300,
    )];
    let link_map = make_link_map(vec![("docs/ref/api.md", "ref/api", "ref-api.md", "ref")]);
    let result = build_document_index(&analyses, &link_map, &make_empty_chunks_result());
    assert_eq!(result.document_tags.len(), 1);
    assert_eq!(result.document_tags[0].0, "ref/api");
    assert!(!result.document_tags[0].1.is_empty());
    assert_eq!(result.document_tags[0].2, "ref");
}

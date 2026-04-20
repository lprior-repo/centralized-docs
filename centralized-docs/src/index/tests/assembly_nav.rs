//! Tests for index assembly, navigation, tags, and slugify.

use std::collections::{HashMap, HashSet};
use std::fs;

use super::*;
use crate::graph::KnowledgeDAG;
use crate::index::index_assembly::{assemble_index_json, GraphAnalytics};

#[test]
fn test_assemble_index_json_structure() {
    let documents = vec![IndexDocument {
        id: "doc1".to_string(),
        title: "Test".to_string(),
        path: "docs/test.md".to_string(),
        category: "concept".to_string(),
        tags: vec![],
        summary: "Sum".to_string(),
        word_count: 100,
        chunk_ids: vec![],
        headings: vec![],
        content: "Content".into(),
    }];
    let dag = KnowledgeDAG::new();
    let analytics = GraphAnalytics {
        topo_order: vec![],
        reachability: HashMap::new(),
        node_importance: HashMap::new(),
    };
    let json = assemble_index_json(
        &documents,
        &[],
        &HashMap::new(),
        &dag,
        &analytics,
        0,
        "test-project",
    );
    assert_eq!(json["version"], "5.0");
    assert_eq!(json["project"], "test-project");
    assert!(json["metadata"].is_object());
    assert!(json["navigation"].is_object());
    assert_eq!(json["stats"]["doc_count"], 1);
}

#[test]
fn test_assemble_index_json_with_chunks() {
    let documents = vec![IndexDocument {
        id: "d1".to_string(),
        title: "D".to_string(),
        path: "p".to_string(),
        category: "c".to_string(),
        tags: vec![],
        summary: "s".to_string(),
        word_count: 100,
        chunk_ids: vec![],
        headings: vec![],
        content: "c".into(),
    }];
    let chunks_metadata = vec![ChunkMetadata {
        chunk_id: "d1#0".to_string(),
        doc_id: "d1".to_string(),
        doc_title: "D".to_string(),
        heading: Some("Intro".to_string()),
        heading_path: vec![],
        heading_anchor: None,
        chunk_type: contextual_chunker::ChunkType::Prose,
        token_count: 200,
        summary: "Chunk sum".to_string(),
        previous_chunk_id: None,
        next_chunk_id: None,
        section_index: 0,
        path: "chunks/d1--0-standard.md".to_string(),
        related_chunks: vec![],
        chunk_level: ChunkLevel::Standard,
        parent_chunk_id: None,
        child_chunk_ids: vec![],
        sibling_chunk_ids: vec![],
    }];
    let dag = KnowledgeDAG::new();
    let analytics = GraphAnalytics {
        topo_order: vec![],
        reachability: HashMap::new(),
        node_importance: HashMap::new(),
    };
    let json = assemble_index_json(
        &documents,
        &chunks_metadata,
        &HashMap::new(),
        &dag,
        &analytics,
        1,
        "p",
    );
    assert_eq!(json["stats"]["doc_count"], 1);
    assert_eq!(json["stats"]["chunk_count"], 1);
    assert_eq!(json["stats"]["avg_chunk_size_tokens"], 200);
}

#[test]
fn test_build_and_write_navigation() {
    let analyses = vec![
        make_analysis(
            "docs/tutorial/rust.md",
            "Rust Tutorial",
            "tutorial",
            vec![],
            "Learn Rust.",
            100,
        ),
        make_analysis(
            "docs/ref/api.md",
            "API Reference",
            "ref",
            vec![],
            "HTTP API docs.",
            200,
        ),
    ];
    let link_map = make_link_map(vec![
        (
            "docs/tutorial/rust.md",
            "tutorial/rust",
            "tutorial-rust.md",
            "tutorial",
        ),
        ("docs/ref/api.md", "ref/api", "ref-api.md", "ref"),
    ]);
    let dir = tempfile::TempDir::new().unwrap();
    build_and_write_navigation(&analyses, &link_map, dir.path()).unwrap();
    let content = fs::read_to_string(dir.path().join("NAVIGATION.md")).unwrap();
    assert!(content.contains("Documentation Navigation"));
    assert!(content.contains("2 documents"));
    assert!(content.contains("Rust Tutorial"));
}

#[test]
fn test_build_and_write_navigation_empty() {
    let dir = tempfile::TempDir::new().unwrap();
    build_and_write_navigation(&[], &HashMap::new(), dir.path()).unwrap();
    let content = fs::read_to_string(dir.path().join("NAVIGATION.md")).unwrap();
    assert!(content.contains("0 documents"));
}

#[test]
fn test_extract_tags_basic() {
    let analysis = make_analysis(
        "test.md",
        "Test",
        "tutorial",
        vec![make_heading(1, "Introduction")],
        "First paragraph.",
        50,
    );
    let tags = extract_tags(&analysis);
    assert!(!tags.is_empty());
    assert!(tags.contains(&"tutorial".to_string()));
}

#[test]
fn test_extract_tags_sorted_and_deduped() {
    let analysis = make_analysis(
        "test.md",
        "Test",
        "tutorial",
        vec![
            make_heading(1, "Programming"),
            make_heading(2, "Programming"),
        ],
        "Text.",
        10,
    );
    let tags = extract_tags(&analysis);
    let mut sorted = tags.clone();
    sorted.sort();
    assert_eq!(tags, sorted);
    let unique: HashSet<_> = tags.iter().collect();
    assert_eq!(unique.len(), tags.len());
}

#[test]
fn test_slugify_heading_basic() {
    assert_eq!(build_index::slugify_heading("Hello World"), "hello-world");
}

#[test]
fn test_slugify_heading_special_chars() {
    assert_eq!(
        build_index::slugify_heading("API Reference (v2)"),
        "api-reference-v2"
    );
}

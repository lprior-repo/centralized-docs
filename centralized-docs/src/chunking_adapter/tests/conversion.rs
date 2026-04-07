//! Basic conversion and utility tests.

use std::collections::HashMap;

use super::*;
use crate::analyze::Analysis;
use crate::assign::IdMapping;
use crate::chunking_adapter::types::{
    analysis_to_document, convert_chunk, convert_chunking_result, escape_frontmatter, slugify,
};

#[test]
fn test_analysis_to_document_with_link_map() {
    let analysis = Analysis {
        source_path: "concept/general/test.md".to_string(),
        title: "Test Document".to_string(),
        content: "## Section\nContent here".into(),
        frontmatter: None,
        headings: vec![],
        links: vec![],
        first_paragraph: "Content here".to_string(),
        word_count: 2,
        has_code: false,
        has_tables: false,
        category: "concept".to_string(),
    };

    let mut link_map = HashMap::new();
    link_map.insert(
        "concept/general/test.md".to_string(),
        IdMapping {
            id: "concept/general/test".to_string(),
            filename: "concept-general-test.md".to_string(),
            subcategory: "general".to_string(),
            slug: "test".to_string(),
        },
    );

    let doc = analysis_to_document(&analysis, &link_map);
    assert_eq!(doc.id, "concept/general/test");
    assert_eq!(doc.title, "Test Document");
    assert_eq!(doc.content, "## Section\nContent here");
}

#[test]
fn test_analysis_to_document_missing_link_map_fallbacks() {
    let analysis = Analysis {
        source_path: "concept/general/test.md".to_string(),
        title: "Test Document".to_string(),
        content: "## Section\nContent here".into(),
        frontmatter: None,
        headings: vec![],
        links: vec![],
        first_paragraph: "Content here".to_string(),
        word_count: 2,
        has_code: false,
        has_tables: false,
        category: "concept".to_string(),
    };

    let link_map = HashMap::new();
    let doc = analysis_to_document(&analysis, &link_map);
    assert_eq!(doc.id, "concept/general/test");
}

#[test]
fn test_chunk_conversion() {
    let cc_chunk = contextual_chunker::Chunk {
        chunk_id: "test#0".to_string(),
        doc_id: "test".to_string(),
        doc_title: "Test".to_string(),
        chunk_index: 0,
        content: "Content".to_string(),
        context_prefix: Some("Context from previous".to_string()),
        token_count: 10,
        heading: Some("Section".to_string()),
        heading_path: vec!["Test".to_string(), "Section".to_string()],
        chunk_type: contextual_chunker::ChunkType::Prose,
        previous_chunk_id: None,
        next_chunk_id: None,
        summary: "Summary".to_string(),
        chunk_level: contextual_chunker::ChunkLevel::Standard,
        parent_chunk_id: None,
        child_chunk_ids: vec![],
    };

    let chunk = convert_chunk(cc_chunk);
    assert_eq!(chunk.chunk_id, "test#0");
    assert_eq!(chunk.chunk_level, contextual_chunker::ChunkLevel::Standard);
    assert!(chunk.related_chunk_ids.is_empty());
}

#[test]
fn test_convert_chunk_with_navigation() {
    let cc_chunk = contextual_chunker::Chunk {
        chunk_id: "nav#0".to_string(),
        doc_id: "nav-doc".to_string(),
        doc_title: "Nav Doc".to_string(),
        chunk_index: 0,
        content: "Nav content".to_string(),
        context_prefix: None,
        token_count: 5,
        heading: Some("Table of Contents".to_string()),
        heading_path: vec!["Nav Doc".to_string(), "Table of Contents".to_string()],
        chunk_type: contextual_chunker::ChunkType::Code,
        previous_chunk_id: None,
        next_chunk_id: Some("nav#1".to_string()),
        summary: "TOC summary".to_string(),
        chunk_level: contextual_chunker::ChunkLevel::Summary,
        parent_chunk_id: Some("nav-doc".to_string()),
        child_chunk_ids: vec!["nav#1".to_string()],
    };

    let chunk = convert_chunk(cc_chunk);
    assert_eq!(chunk.chunk_type, contextual_chunker::ChunkType::Code);
    assert_eq!(chunk.previous_chunk_id, None);
    assert_eq!(chunk.next_chunk_id.as_deref(), Some("nav#1"));
    assert_eq!(chunk.parent_chunk_id.as_deref(), Some("nav-doc"));
    assert_eq!(chunk.child_chunk_ids.len(), 1);
    assert_eq!(chunk.context_prefix, None);
}

#[test]
fn test_slugify() {
    assert_eq!(slugify("Hello World"), "hello-world");
    assert_eq!(slugify("Test Document"), "test-document");
    assert_eq!(slugify("hello-world"), "hello-world");
}

#[test]
fn test_slugify_special_chars() {
    let result = slugify("test@#$%^&*()doc");
    assert!(result.contains("test"));
    assert!(result.contains("doc"));
    assert!(!result.contains('@'));
}

#[test]
fn test_fallback_doc_id() {
    let analysis = Analysis {
        source_path: "concept/general/my-doc.md".to_string(),
        title: "My Doc".to_string(),
        content: "content".into(),
        frontmatter: None,
        headings: vec![],
        links: vec![],
        first_paragraph: "content".to_string(),
        word_count: 1,
        has_code: false,
        has_tables: false,
        category: "concept".to_string(),
    };
    let link_map = HashMap::new();
    let doc = analysis_to_document(&analysis, &link_map);
    assert_eq!(doc.id, "concept/general/my-doc");
}

#[test]
fn test_fallback_doc_id_shallow_path() {
    let analysis = Analysis {
        source_path: "file.md".to_string(),
        title: "File".to_string(),
        content: "content".into(),
        frontmatter: None,
        headings: vec![],
        links: vec![],
        first_paragraph: "content".to_string(),
        word_count: 1,
        has_code: false,
        has_tables: false,
        category: "concept".to_string(),
    };
    let link_map = HashMap::new();
    let doc = analysis_to_document(&analysis, &link_map);
    assert!(doc.id.starts_with("concept/"));
}

#[test]
fn test_analysis_to_document_empty_title() {
    let analysis = Analysis {
        source_path: "concept/general/test.md".to_string(),
        title: String::new(),
        content: "content".into(),
        frontmatter: None,
        headings: vec![],
        links: vec![],
        first_paragraph: "content".to_string(),
        word_count: 1,
        has_code: false,
        has_tables: false,
        category: "concept".to_string(),
    };
    let link_map = HashMap::new();
    let doc = analysis_to_document(&analysis, &link_map);
    assert_eq!(doc.title, "Untitled");
}

#[test]
fn test_escape_frontmatter() {
    assert_eq!(escape_frontmatter("hello"), "hello");
    assert_eq!(escape_frontmatter("line1\nline2"), "line1 line2");
    assert_eq!(escape_frontmatter("say \"hi\""), "say \\\"hi\\\"");
}

#[test]
fn test_convert_chunking_result() {
    let cc_result = contextual_chunker::ChunkingResult {
        chunks: vec![contextual_chunker::Chunk {
            chunk_id: "doc#0".to_string(),
            doc_id: "doc".to_string(),
            doc_title: "Doc".to_string(),
            chunk_index: 0,
            content: "Summary content".to_string(),
            context_prefix: None,
            token_count: 20,
            heading: Some("Intro".to_string()),
            heading_path: vec!["Doc".to_string()],
            chunk_type: contextual_chunker::ChunkType::Prose,
            previous_chunk_id: None,
            next_chunk_id: None,
            summary: "A summary".to_string(),
            chunk_level: contextual_chunker::ChunkLevel::Summary,
            parent_chunk_id: None,
            child_chunk_ids: vec![],
        }],
        summary_count: 1,
        standard_count: 0,
        detailed_count: 0,
    };

    let result = convert_chunking_result(cc_result, 1);
    assert_eq!(result.total_chunks, 1);
    assert_eq!(result.document_count, 1);
    assert_eq!(result.summary_chunks, 1);
    assert_eq!(result.standard_chunks, 0);
    assert_eq!(result.detailed_chunks, 0);
    assert_eq!(result.chunks_metadata.len(), 1);
}

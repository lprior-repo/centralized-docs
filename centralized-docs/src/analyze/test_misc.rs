use std::collections::HashMap;
use std::sync::Arc;

use super::types::*;
use crate::discover::DiscoveryFile;

#[test]
fn test_link_kind_enum() {
    assert_eq!(LinkKind::Internal, LinkKind::Internal);
    assert_eq!(LinkKind::External, LinkKind::External);
    assert_ne!(LinkKind::Internal, LinkKind::External);
}

#[test]
fn test_heading_struct() {
    let h = Heading {
        level: 2,
        text: "Test".to_string(),
        line: 5,
    };
    let cloned = h.clone();
    assert_eq!(h.level, cloned.level);
    assert_eq!(h.text, cloned.text);
}

#[test]
fn test_failed_file_struct() {
    let f = FailedFile {
        source_path: "bad.md".to_string(),
        error: "file not found".to_string(),
    };
    let cloned = f.clone();
    assert_eq!(f.source_path, cloned.source_path);
}

#[test]
fn test_analyze_serialization() {
    let analysis = Analysis {
        source_path: "test.md".to_string(),
        title: "Test".to_string(),
        frontmatter: Some(HashMap::from([("key".to_string(), "val".to_string())])),
        headings: vec![Heading {
            level: 1,
            text: "H1".to_string(),
            line: 0,
        }],
        links: vec![Link {
            text: "link".to_string(),
            target: "url".to_string(),
            kind: LinkKind::External,
        }],
        first_paragraph: "Para".to_string(),
        word_count: 42,
        has_code: true,
        has_tables: false,
        category: "tutorial".to_string(),
        content: Arc::from("content"),
    };

    let json = serde_json::to_string(&analysis).unwrap();
    let deserialized: Analysis = serde_json::from_str(&json).unwrap();
    assert_eq!(deserialized.title, "Test");
    assert_eq!(deserialized.word_count, 42);
    assert!(deserialized.has_code);
    assert_eq!(deserialized.links[0].kind, LinkKind::External);
}

#[test]
fn test_analyze_files_discover_file_struct() {
    let df = DiscoveryFile {
        source_path: "path/to/file.md".to_string(),
        size_bytes: 1024,
    };
    let cloned = df.clone();
    assert_eq!(df.source_path, cloned.source_path);
    assert_eq!(df.size_bytes, cloned.size_bytes);
}

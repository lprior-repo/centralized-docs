use doc_transformer::analyze::{Analysis, Link};
use doc_transformer::validate::{validate_links, BrokenLinkReason};
use std::path::Path;

#[test]
fn test_validate_links_no_broken_links() {
    // Create analyses with valid internal links
    let analyses = vec![
        Analysis {
            source_path: "docs/intro.md".to_string(),
            title: "Introduction".to_string(),
            frontmatter: None,
            headings: vec![],
            links: vec![
                Link {
                    text: "See guide".to_string(),
                    target: "./guide.md".to_string(),
                    is_internal: true,
                },
            ],
            first_paragraph: "Intro".to_string(),
            word_count: 10,
            has_code: false,
            has_tables: false,
            category: "tutorial".to_string(),
            content: "Content".to_string(),
        },
        Analysis {
            source_path: "docs/guide.md".to_string(),
            title: "Guide".to_string(),
            frontmatter: None,
            headings: vec![],
            links: vec![],
            first_paragraph: "Guide".to_string(),
            word_count: 10,
            has_code: false,
            has_tables: false,
            category: "tutorial".to_string(),
            content: "Content".to_string(),
        },
    ];

    let source_dir = Path::new(".");
    let result = validate_links(&analyses, source_dir).unwrap();

    assert_eq!(result.total_links, 1);
    assert_eq!(result.internal_links, 1);
    assert_eq!(result.broken_links.len(), 0);
}

#[test]
fn test_validate_links_detects_broken_link() {
    // Create analysis with broken internal link
    let analyses = vec![
        Analysis {
            source_path: "docs/intro.md".to_string(),
            title: "Introduction".to_string(),
            frontmatter: None,
            headings: vec![],
            links: vec![
                Link {
                    text: "Broken link".to_string(),
                    target: "./nonexistent.md".to_string(),
                    is_internal: true,
                },
            ],
            first_paragraph: "Intro".to_string(),
            word_count: 10,
            has_code: false,
            has_tables: false,
            category: "tutorial".to_string(),
            content: "Content".to_string(),
        },
    ];

    let source_dir = Path::new(".");
    let result = validate_links(&analyses, source_dir).unwrap();

    assert_eq!(result.total_links, 1);
    assert_eq!(result.internal_links, 1);
    assert_eq!(result.broken_links.len(), 1);

    let broken = &result.broken_links[0];
    assert_eq!(broken.source_file, "docs/intro.md");
    assert_eq!(broken.target, "./nonexistent.md");
    assert!(matches!(broken.reason, BrokenLinkReason::FileNotFound));
}

#[test]
fn test_validate_links_ignores_external_links() {
    // External links should not be validated
    let analyses = vec![
        Analysis {
            source_path: "docs/intro.md".to_string(),
            title: "Introduction".to_string(),
            frontmatter: None,
            headings: vec![],
            links: vec![
                Link {
                    text: "External".to_string(),
                    target: "https://example.com".to_string(),
                    is_internal: false,
                },
                Link {
                    text: "Mailto".to_string(),
                    target: "mailto:test@example.com".to_string(),
                    is_internal: false,
                },
            ],
            first_paragraph: "Intro".to_string(),
            word_count: 10,
            has_code: false,
            has_tables: false,
            category: "tutorial".to_string(),
            content: "Content".to_string(),
        },
    ];

    let source_dir = Path::new(".");
    let result = validate_links(&analyses, source_dir).unwrap();

    assert_eq!(result.total_links, 2);
    assert_eq!(result.internal_links, 0);
    assert_eq!(result.broken_links.len(), 0);
}

#[test]
fn test_validate_links_strips_anchors() {
    // Link with anchor to existing file should be valid
    let analyses = vec![
        Analysis {
            source_path: "docs/intro.md".to_string(),
            title: "Introduction".to_string(),
            frontmatter: None,
            headings: vec![],
            links: vec![
                Link {
                    text: "See guide section".to_string(),
                    target: "./guide.md#section".to_string(),
                    is_internal: true,
                },
            ],
            first_paragraph: "Intro".to_string(),
            word_count: 10,
            has_code: false,
            has_tables: false,
            category: "tutorial".to_string(),
            content: "Content".to_string(),
        },
        Analysis {
            source_path: "docs/guide.md".to_string(),
            title: "Guide".to_string(),
            frontmatter: None,
            headings: vec![],
            links: vec![],
            first_paragraph: "Guide".to_string(),
            word_count: 10,
            has_code: false,
            has_tables: false,
            category: "tutorial".to_string(),
            content: "Content".to_string(),
        },
    ];

    let source_dir = Path::new(".");
    let result = validate_links(&analyses, source_dir).unwrap();

    assert_eq!(result.internal_links, 1);
    assert_eq!(result.broken_links.len(), 0);
}

#[test]
fn test_validate_links_detects_empty_target() {
    // Empty link target should be reported as broken
    let analyses = vec![
        Analysis {
            source_path: "docs/intro.md".to_string(),
            title: "Introduction".to_string(),
            frontmatter: None,
            headings: vec![],
            links: vec![
                Link {
                    text: "Empty link".to_string(),
                    target: "".to_string(),
                    is_internal: true,
                },
            ],
            first_paragraph: "Intro".to_string(),
            word_count: 10,
            has_code: false,
            has_tables: false,
            category: "tutorial".to_string(),
            content: "Content".to_string(),
        },
    ];

    let source_dir = Path::new(".");
    let result = validate_links(&analyses, source_dir).unwrap();

    assert_eq!(result.broken_links.len(), 1);
    assert!(matches!(result.broken_links[0].reason, BrokenLinkReason::EmptyTarget));
}

#[test]
fn test_validate_links_parent_directory_links() {
    // Test links with ../ parent directory traversal
    let analyses = vec![
        Analysis {
            source_path: "docs/subdir/page.md".to_string(),
            title: "Page".to_string(),
            frontmatter: None,
            headings: vec![],
            links: vec![
                Link {
                    text: "Parent".to_string(),
                    target: "../intro.md".to_string(),
                    is_internal: true,
                },
            ],
            first_paragraph: "Page".to_string(),
            word_count: 10,
            has_code: false,
            has_tables: false,
            category: "tutorial".to_string(),
            content: "Content".to_string(),
        },
        Analysis {
            source_path: "docs/intro.md".to_string(),
            title: "Introduction".to_string(),
            frontmatter: None,
            headings: vec![],
            links: vec![],
            first_paragraph: "Intro".to_string(),
            word_count: 10,
            has_code: false,
            has_tables: false,
            category: "tutorial".to_string(),
            content: "Content".to_string(),
        },
    ];

    let source_dir = Path::new(".");
    let result = validate_links(&analyses, source_dir).unwrap();

    assert_eq!(result.internal_links, 1);
    assert_eq!(result.broken_links.len(), 0);
}

#[test]
fn test_validate_links_multiple_broken_links() {
    // Test multiple broken links across files
    let analyses = vec![
        Analysis {
            source_path: "docs/intro.md".to_string(),
            title: "Introduction".to_string(),
            frontmatter: None,
            headings: vec![],
            links: vec![
                Link {
                    text: "Broken 1".to_string(),
                    target: "./missing1.md".to_string(),
                    is_internal: true,
                },
                Link {
                    text: "Broken 2".to_string(),
                    target: "./missing2.md".to_string(),
                    is_internal: true,
                },
            ],
            first_paragraph: "Intro".to_string(),
            word_count: 10,
            has_code: false,
            has_tables: false,
            category: "tutorial".to_string(),
            content: "Content".to_string(),
        },
    ];

    let source_dir = Path::new(".");
    let result = validate_links(&analyses, source_dir).unwrap();

    assert_eq!(result.broken_links.len(), 2);
    assert_eq!(result.broken_links[0].source_file, "docs/intro.md");
    assert_eq!(result.broken_links[1].source_file, "docs/intro.md");
}

#[test]
fn test_validate_links_mixed_internal_external() {
    // Mix of internal (some broken) and external links
    let analyses = vec![
        Analysis {
            source_path: "docs/intro.md".to_string(),
            title: "Introduction".to_string(),
            frontmatter: None,
            headings: vec![],
            links: vec![
                Link {
                    text: "External".to_string(),
                    target: "https://example.com".to_string(),
                    is_internal: false,
                },
                Link {
                    text: "Valid internal".to_string(),
                    target: "./guide.md".to_string(),
                    is_internal: true,
                },
                Link {
                    text: "Broken internal".to_string(),
                    target: "./missing.md".to_string(),
                    is_internal: true,
                },
            ],
            first_paragraph: "Intro".to_string(),
            word_count: 10,
            has_code: false,
            has_tables: false,
            category: "tutorial".to_string(),
            content: "Content".to_string(),
        },
        Analysis {
            source_path: "docs/guide.md".to_string(),
            title: "Guide".to_string(),
            frontmatter: None,
            headings: vec![],
            links: vec![],
            first_paragraph: "Guide".to_string(),
            word_count: 10,
            has_code: false,
            has_tables: false,
            category: "tutorial".to_string(),
            content: "Content".to_string(),
        },
    ];

    let source_dir = Path::new(".");
    let result = validate_links(&analyses, source_dir).unwrap();

    assert_eq!(result.total_links, 3);
    assert_eq!(result.internal_links, 2);
    assert_eq!(result.broken_links.len(), 1);
    assert_eq!(result.broken_links[0].target, "./missing.md");
}

#[test]
fn test_validate_links_query_string_ignored() {
    // Links with query strings should have query stripped before validation
    let analyses = vec![
        Analysis {
            source_path: "docs/intro.md".to_string(),
            title: "Introduction".to_string(),
            frontmatter: None,
            headings: vec![],
            links: vec![
                Link {
                    text: "With query".to_string(),
                    target: "./guide.md?version=1.0".to_string(),
                    is_internal: true,
                },
            ],
            first_paragraph: "Intro".to_string(),
            word_count: 10,
            has_code: false,
            has_tables: false,
            category: "tutorial".to_string(),
            content: "Content".to_string(),
        },
        Analysis {
            source_path: "docs/guide.md".to_string(),
            title: "Guide".to_string(),
            frontmatter: None,
            headings: vec![],
            links: vec![],
            first_paragraph: "Guide".to_string(),
            word_count: 10,
            has_code: false,
            has_tables: false,
            category: "tutorial".to_string(),
            content: "Content".to_string(),
        },
    ];

    let source_dir = Path::new(".");
    let result = validate_links(&analyses, source_dir).unwrap();

    assert_eq!(result.internal_links, 1);
    assert_eq!(result.broken_links.len(), 0);
}

#[test]
fn test_validate_links_empty_analyses() {
    // Empty analyses should return all zeros
    let analyses = vec![];
    let source_dir = Path::new(".");
    let result = validate_links(&analyses, source_dir).unwrap();

    assert_eq!(result.total_links, 0);
    assert_eq!(result.internal_links, 0);
    assert_eq!(result.broken_links.len(), 0);
}

#[test]
fn test_validate_links_no_links() {
    // Analysis with no links
    let analyses = vec![
        Analysis {
            source_path: "docs/intro.md".to_string(),
            title: "Introduction".to_string(),
            frontmatter: None,
            headings: vec![],
            links: vec![],
            first_paragraph: "Intro".to_string(),
            word_count: 10,
            has_code: false,
            has_tables: false,
            category: "tutorial".to_string(),
            content: "Content".to_string(),
        },
    ];

    let source_dir = Path::new(".");
    let result = validate_links(&analyses, source_dir).unwrap();

    assert_eq!(result.total_links, 0);
    assert_eq!(result.internal_links, 0);
    assert_eq!(result.broken_links.len(), 0);
}

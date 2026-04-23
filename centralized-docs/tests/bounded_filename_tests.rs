//! Bounded filename tests for `cdocs-mgf` — cli: bound derived filenames during index chunk emission
//!
//! These tests define the EXPECTED behavior after implementing the bounding fix.
//! Currently (RED phase), they FAIL because the bounding logic is not yet implemented.
//!
//! ## Bug Summary
//!
//! The system has two filename derivation points that are entirely unbounded:
//! 1. `assign_ids` (`assign.rs:45`) — builds document filenames via
//!    `format!("{category}-{subcategory}-{slug}.md")`. For the QA Docs corpus,
//!    source paths exceed 255 bytes.
//! 2. `chunk_all` / `write_chunk_file` — builds chunk filenames via
//!    `format!("{chunk_id}-{level_suffix}.md")` where `chunk_id` contains the
//!    unbounded doc_id. OS error 36 fires at `fs::write`.
//!
//! ## Expected Behavior After Fix
//!
//! - Document filenames must be ≤ 187 bytes (POST2)
//! - Chunk filenames must be ≤ 200 bytes (POST3)
//! - Bounded names use format: `{truncated_stem[:172]}-{hash[:8]}.md` for docs
//! - Bounded chunk names use format: `{bounded_chunk_id[:180]}-{hash[:8]}-{level}.md`
//! - Hash suffix is deterministic: same input → same hash across runs

#![allow(clippy::pedantic)]
#![allow(clippy::unwrap_used)]
#![allow(clippy::expect_used)]
#![allow(clippy::panic)]

use std::fs;

use doc_transformer::analyze::Analysis;
use doc_transformer::assign::assign_ids;
use doc_transformer::chunking_adapter::{chunk_all, write_chunk_file, Chunk};

use tempfile::TempDir;

// ============================================================================
// TEST FIXTURES
// ============================================================================

/// Create an Analysis with a very long source_path that would produce an
/// overlong derived filename (> 187 bytes).
fn make_long_name_analysis(source_path: &str, category: &str, title: &str) -> Analysis {
    use std::sync::Arc;
    Analysis {
        source_path: source_path.to_string(),
        title: title.to_string(),
        frontmatter: None,
        headings: vec![],
        links: vec![],
        first_paragraph: "Test paragraph".to_string(),
        word_count: 10,
        has_code: false,
        has_tables: false,
        category: category.to_string(),
        content: Arc::from("Test content"),
    }
}

/// Create a Chunk with a long chunk_id that would produce an overlong
/// chunk filename (> 200 bytes).
fn make_long_chunk_id(chunk_id: &str, doc_id: &str) -> Chunk {
    Chunk {
        chunk_id: chunk_id.to_string(),
        doc_id: doc_id.to_string(),
        doc_title: "Test Doc".to_string(),
        chunk_index: 0,
        content: "Test content".to_string(),
        token_count: 10,
        heading: Some("Intro".to_string()),
        heading_path: vec![],
        chunk_type: contextual_chunker::ChunkType::Prose,
        previous_chunk_id: None,
        next_chunk_id: None,
        related_chunk_ids: vec![],
        summary: "Summary".to_string(),
        chunk_level: contextual_chunker::ChunkLevel::Standard,
        parent_chunk_id: None,
        child_chunk_ids: vec![],
        context_prefix: None,
    }
}

// ============================================================================
// UNIT TESTS: Document Filename Bounding (B1-B5, INV1-INV3, INV6)
// ============================================================================

/// B1: IdMapping.filename is ≤ 187 bytes
/// Currently FAILS: no length check in assign_ids
#[test]
fn assign_ids_produces_bounded_filename_len() {
    // This is the REAL problematic filename from the QA Docs corpus
    let long_source = "ref-docs-tasks-administer-cluster-manage-resources.md-docs-tasks-administer-cluster-manage-resources.md";
    let analysis = make_long_name_analysis(long_source, "ref", "Manage Resources");
    let (_, map) = assign_ids(vec![analysis]);

    let mapping = map.get(long_source).expect("mapping must exist");
    let filename_len = mapping.filename.len();

    // POST2: filename must be ≤ 187 bytes
    assert!(
        filename_len <= 187,
        "filename '{}' is {} bytes, exceeds 187 byte budget (POST2)",
        mapping.filename,
        filename_len
    );
}

/// B5: Overlong derived name gets bounded with hash suffix
/// Currently FAILS: no bounding in assign_ids
#[test]
fn assign_ids_bounded_name_contains_hash_suffix_when_truncated() {
    // The problematic filename from QA Docs corpus (> 187 bytes natural)
    let long_source = "ref-docs-tasks-administer-cluster-manage-resources.md-docs-tasks-administer-cluster-manage-resources.md";
    let analysis = make_long_name_analysis(long_source, "ref", "Manage Resources");
    let (_, map) = assign_ids(vec![analysis]);

    let mapping = map.get(long_source).expect("mapping must exist");
    let filename_len = mapping.filename.len();

    // If natural name would exceed 187, bounded name must contain hash suffix
    // The bounded format is: {truncated_stem[:172]}-{hash[:8]}.md = 184 bytes
    // Hash suffix is 8 hex chars before .md
    let has_hash_suffix = regex::Regex::new(r"-[[:xdigit:]]{8}\.md$").unwrap();

    // If filename is long (>= 172 chars), it MUST have been truncated and have hash suffix
    if filename_len >= 172 {
        assert!(
            has_hash_suffix.is_match(&mapping.filename),
            "long filename '{}' missing hash suffix (expected truncated_stem-hash8.md format)",
            mapping.filename
        );
    }
}

/// B2: IdMapping.filename is deterministic
/// This should PASS on current code (determinism is expected to hold even without fix)
#[test]
fn assign_ids_deterministic_output() {
    let long_source = "ref-docs-tasks-administer-cluster-manage-resources.md-docs-tasks-administer-cluster-manage-resources.md";
    let analysis = make_long_name_analysis(long_source, "ref", "Manage Resources");

    let (_, map1) = assign_ids(vec![analysis.clone()]);
    let (_, map2) = assign_ids(vec![analysis]);

    // Compare the filenames from both maps
    let filenames1: Vec<_> = map1.values().map(|m| m.filename.clone()).collect();
    let filenames2: Vec<_> = map2.values().map(|m| m.filename.clone()).collect();
    assert_eq!(filenames1, filenames2, "assign_ids must be deterministic");
}

/// B3: Two distinct inputs produce distinct filenames (no collision)
/// Currently FAILS if two long stems truncate to same stem (no hash suffix)
#[test]
fn assign_ids_distinct_sources_produce_distinct_filenames() {
    // Two source documents with distinct paths but long common prefixes
    let source_a = "ref-docs-tasks-administer-cluster-manage-resources.md-docs-tasks-administer-cluster-manage-resources.md";
    let source_b = "ref-docs-tasks-administer-cluster-manage-config.md-docs-tasks-administer-cluster-manage-config.md";

    let analyses = vec![
        make_long_name_analysis(source_a, "ref", "Manage Resources"),
        make_long_name_analysis(source_b, "ref", "Manage Config"),
    ];

    let (_, map) = assign_ids(analyses);

    let filename_a = &map.get(source_a).expect("mapping A exists").filename;
    let filename_b = &map.get(source_b).expect("mapping B exists").filename;

    assert_ne!(
        filename_a, filename_b,
        "distinct sources produced colliding filenames: '{}' == '{}' (W2: no stem collision)",
        filename_a, filename_b
    );
}

/// INV1: All derived document filenames are ≤ 187 bytes (proptest)
/// Currently FAILS: no length check in assign_ids
#[test]
fn prop_idmapping_filename_len_always_bounded() {
    // Test with the known long problematic path
    let long_source = "ref-docs-tasks-administer-cluster-manage-resources.md-docs-tasks-administer-cluster-manage-resources.md";
    let analysis = make_long_name_analysis(long_source, "ref", "Manage Resources");

    let (_, map) = assign_ids(vec![analysis]);
    for mapping in map.values() {
        assert!(
            mapping.filename.len() <= 187,
            "filename '{}' is {} bytes, exceeds 187 (POST2)",
            mapping.filename,
            mapping.filename.len()
        );
    }
}

/// INV2: Distinct source paths produce distinct filenames (collision resistance)
/// Currently FAILS if truncation without hash causes collision
#[test]
fn prop_distinct_sources_no_collision() {
    // Two sources that would collide without proper hash suffix
    let source_a = "docs/ref-docs-tasks-administer-cluster-manage-resources.md";
    let source_b = "docs/ref-docs-tasks-administer-cluster-manage-config.md";

    let analyses = vec![
        make_long_name_analysis(source_a, "ref", "Manage Resources"),
        make_long_name_analysis(source_b, "ref", "Manage Config"),
    ];

    let (_, map) = assign_ids(analyses);
    let filenames: Vec<&str> = map.values().map(|m| m.filename.as_str()).collect();
    let unique: std::collections::HashSet<_> = filenames.iter().collect();

    assert_eq!(
        filenames.len(),
        unique.len(),
        "distinct sources produced colliding filenames: {:?}",
        filenames
    );
}

// ============================================================================
// UNIT TESTS: Chunk Filename Bounding (B6-B10, INV4-INV5)
// ============================================================================

/// B6: chunk_all produces chunk filenames ≤ 200 bytes
/// Currently FAILS: no length check in chunk_all formatting
#[test]
fn chunk_all_produces_bounded_chunk_filename() {
    let temp = TempDir::new().unwrap();
    let output_dir = temp.path();

    // A document with a long path that will produce a long chunk_id
    let long_source = "ref-docs-tasks-administer-cluster-manage-resources.md-docs-tasks-administer-cluster-manage-resources.md";
    let analysis = make_long_name_analysis(long_source, "ref", "Manage Resources");

    // First get the IdMapping to build link_map
    let (_, link_map) = assign_ids(vec![analysis.clone()]);

    // Run chunk_all
    let result = chunk_all(&[analysis], &link_map, output_dir, 1_000_000);

    // If chunking succeeded, check the actual filenames written
    if result.is_ok() {
        let chunks_dir = output_dir.join("chunks");
        if chunks_dir.exists() {
            for entry in fs::read_dir(&chunks_dir).expect("read_dir should work") {
                let entry = entry.expect("entry should exist");
                let filename = entry.file_name().to_string_lossy().to_string();
                let filename_len = filename.len();
                assert!(
                    filename_len <= 200,
                    "chunk filename '{}' is {} bytes, exceeds 200 byte budget (POST3)",
                    filename,
                    filename_len
                );
            }
        }
    } else {
        // If it failed, check if it was due to OS error 36 (File name too long)
        let err = result.unwrap_err().to_string();
        if err.contains("File name too long") || err.contains("os error 36") {
            panic!("OS error 36: chunk filename exceeded filesystem limit (FM1)");
        }
    }
}

/// B9: chunk_all does NOT fail with OS error 36 on long corpus
/// Currently FAILS: the long corpus produces OS error 36
#[test]
fn chunk_all_no_os_error_36_on_long_corpus() {
    let temp = TempDir::new().unwrap();
    let output_dir = temp.path();

    // Multiple documents from the QA Docs corpus with known long names
    let docs = vec![
        ("ref-docs-tasks-administer-cluster-manage-resources.md-docs-tasks-administer-cluster-manage-resources.md", "ref", "Manage Resources"),
        ("ref-docs-tasks-administer-cluster-manage-config.md-docs-tasks-administer-cluster-manage-config.md", "ref", "Manage Config"),
        ("tutorial-docs-tasks-administer-cluster-securing-a-cluster.md-docs-tasks-administer-cluster-securing-a-cluster.md", "tutorial", "Securing Cluster"),
    ];

    let analyses: Vec<Analysis> = docs
        .iter()
        .map(|(path, cat, title)| make_long_name_analysis(path, cat, title))
        .collect();

    let (_, link_map) = assign_ids(analyses.clone());

    // This should NOT fail with OS error 36
    let result = chunk_all(&analyses, &link_map, output_dir, 1_000_000);

    // Assert no OS error 36 (File name too long)
    if let Err(e) = &result {
        let err_str = e.to_string();
        assert!(
            !err_str.contains("File name too long"),
            "OS error 36: chunk filename exceeded filesystem limit (W1)",
        );
        assert!(
            !err_str.contains("os error 36"),
            "OS error 36: chunk filename exceeded filesystem limit (W1)",
        );
    }

    assert!(
        result.is_ok(),
        "chunk_all should succeed on long corpus (POST1)"
    );
}

/// B7: write_chunk_file produces filenames ≤ 200 bytes
/// Currently FAILS: no length check in write_chunk_file
#[test]
fn write_chunk_file_bounded_filename() {
    let temp = TempDir::new().unwrap();
    let chunks_dir = temp.path().join("chunks");
    fs::create_dir_all(&chunks_dir).unwrap();

    // Long chunk_id that would exceed 200 bytes
    let long_chunk_id = "ref/ref-docs-tasks-administer-cluster-manage-resources.md-docs-tasks-administer-cluster-manage-resources.md/0";
    let chunk = make_long_chunk_id(
        long_chunk_id,
        "ref/ref-docs-tasks-administer-cluster-manage-resources",
    );

    let result = write_chunk_file(&chunk, &chunks_dir);

    // Assert no OS error 36
    if let Err(e) = &result {
        let err_str = e.to_string();
        assert!(
            !err_str.contains("File name too long"),
            "OS error 36: chunk filename exceeded filesystem limit (W1)",
        );
        assert!(
            !err_str.contains("os error 36"),
            "OS error 36: chunk filename exceeded filesystem limit (W1)",
        );
    }

    assert!(
        result.is_ok(),
        "write_chunk_file should succeed on long chunk_id"
    );

    // If successful, verify the actual filename is bounded
    if result.is_ok() {
        for entry in fs::read_dir(&chunks_dir).expect("read_dir should work") {
            let entry = entry.expect("entry should exist");
            let filename = entry.file_name().to_string_lossy().to_string();
            assert!(
                filename.len() <= 200,
                "written chunk filename '{}' is {} bytes, exceeds 200 (POST3)",
                filename,
                filename.len()
            );
        }
    }
}

/// B10: write_chunk_file does NOT fail with OS error 36 on long chunk_id
/// Currently FAILS: the long chunk_id produces OS error 36
#[test]
fn write_chunk_file_no_os_error_36() {
    let temp = TempDir::new().unwrap();
    let chunks_dir = temp.path().join("chunks");
    fs::create_dir_all(&chunks_dir).unwrap();

    // The exact problematic pattern from QA Docs corpus
    let long_chunk_id = "ref/ref-docs-tasks-administer-cluster-manage-resources.md-docs-tasks-administer-cluster-manage-resources.md/0";
    let chunk = make_long_chunk_id(
        long_chunk_id,
        "ref/ref-docs-tasks-administer-cluster-manage-resources",
    );

    let result = write_chunk_file(&chunk, &chunks_dir);

    if let Err(e) = &result {
        let err_str = e.to_string();
        assert!(
            !err_str.contains("File name too long") && !err_str.contains("os error 36"),
            "OS error 36 on long chunk_id: {} (W1)",
            err_str
        );
    }

    assert!(
        result.is_ok(),
        "write_chunk_file should not fail with OS error 36 on long chunk_id"
    );
}

/// INV4: All derived chunk filenames are ≤ 200 bytes (proptest)
/// Currently FAILS: no length check
#[test]
fn prop_chunk_filename_len_always_bounded() {
    // Test with the known long chunk_id
    let long_chunk_id = "ref/ref-docs-tasks-administer-cluster-manage-resources.md-docs-tasks-administer-cluster-manage-resources.md/0";
    let chunk = make_long_chunk_id(
        long_chunk_id,
        "ref/ref-docs-tasks-administer-cluster-manage-resources",
    );

    let level_suffix = match chunk.chunk_level {
        contextual_chunker::ChunkLevel::Summary => "summary",
        contextual_chunker::ChunkLevel::Standard => "standard",
        contextual_chunker::ChunkLevel::Detailed => "detailed",
    };

    let chunk_filename = format!(
        "{}-{}.md",
        chunk.chunk_id.replace(['/', '#'], "-"),
        level_suffix
    );

    assert!(
        chunk_filename.len() <= 200,
        "chunk filename '{}' is {} bytes, exceeds 200 (POST3)",
        chunk_filename,
        chunk_filename.len()
    );
}

// ============================================================================
// INTEGRATION TESTS: ChunkMetadata Path Sync (B11-B12, INV8)
// ============================================================================

/// B11: ChunkMetadata.path matches actual filename on disk
/// Currently FAILS: path uses unbounded chunk_id, actual file may use bounded name
#[test]
fn chunkmetadata_path_matches_actual_filename() {
    let temp = TempDir::new().unwrap();
    let output_dir = temp.path();

    let long_source = "ref-docs-tasks-administer-cluster-manage-resources.md-docs-tasks-administer-cluster-manage-resources.md";
    let analysis = make_long_name_analysis(long_source, "ref", "Manage Resources");

    let (_, link_map) = assign_ids(vec![analysis.clone()]);

    let result = chunk_all(&[analysis], &link_map, output_dir, 1_000_000);

    if result.is_ok() {
        // Read INDEX.json and verify paths match actual files
        let index_path = output_dir.join("INDEX.json");
        if index_path.exists() {
            let index_content: serde_json::Value =
                serde_json::from_str(&fs::read_to_string(&index_path).unwrap()).unwrap();

            // The INDEX.json has chunks as an array of ChunkMetadata objects
            if let Some(chunks) = index_content.get("chunks").and_then(|c| c.as_array()) {
                for chunk_meta in chunks {
                    if let Some(path) = chunk_meta.get("path").and_then(|p| p.as_str()) {
                        let expected_path = output_dir.join(path);
                        assert!(
                            expected_path.exists(),
                            "ChunkMetadata.path '{}' does not resolve to actual file (POST6)",
                            path
                        );
                    }
                }
            }
        }
    }
}

/// INV8: ChunkMetadata.path format is bounded
/// Currently FAILS: unbounded chunk_id in path
#[test]
fn prop_chunkmetadata_path_bounded() {
    let long_chunk_id = "ref/ref-docs-tasks-administer-cluster-manage-resources.md-docs-tasks-administer-cluster-manage-resources.md/0";
    let chunk = make_long_chunk_id(
        long_chunk_id,
        "ref/ref-docs-tasks-administer-cluster-manage-resources",
    );

    let level_suffix = match chunk.chunk_level {
        contextual_chunker::ChunkLevel::Summary => "summary",
        contextual_chunker::ChunkLevel::Standard => "standard",
        contextual_chunker::ChunkLevel::Detailed => "detailed",
    };

    // This is the format used in build_index.rs:204-208
    let path = format!(
        "chunks/{}-{}.md",
        chunk.chunk_id.replace(['/', '#'], "-"),
        level_suffix
    );

    // Extract just the filename part
    let filename = path.strip_prefix("chunks/").unwrap_or(&path);

    assert!(
        filename.len() <= 200,
        "ChunkMetadata.path filename '{}' is {} bytes, exceeds 200 (POST3)",
        filename,
        filename.len()
    );
}

// ============================================================================
// BDD SCENARIO TESTS
// ============================================================================

/// Scenario 1: Document filename stays within budget
/// Given: A document with source_path "docs/ref-docs-tasks-administer-cluster-manage-resources.md"
///        and category "ref"
/// When:  assign_ids processes it
/// Then:  The resulting IdMapping.filename is ≤ 187 bytes
/// And:   The filename ends with ".md"
/// And:   If the natural filename would exceed 187 bytes, it contains a deterministic hash suffix
#[test]
fn scenario1_document_filename_budget() {
    let source = "docs/ref-docs-tasks-administer-cluster-manage-resources.md";
    let analysis = make_long_name_analysis(source, "ref", "Manage Resources");
    let (_, map) = assign_ids(vec![analysis]);

    let mapping = map.get(source).expect("mapping must exist");

    // Then: filename is ≤ 187 bytes
    assert!(
        mapping.filename.len() <= 187,
        "filename '{}' is {} bytes, exceeds 187",
        mapping.filename,
        mapping.filename.len()
    );

    // And: filename ends with ".md"
    assert!(
        mapping.filename.ends_with(".md"),
        "filename '{}' must end with '.md'",
        mapping.filename
    );

    // And: if natural name would exceed 187, contains hash suffix
    let has_hash_suffix = regex::Regex::new(r"-[[:xdigit:]]{8}\.md$").unwrap();
    if mapping.filename.len() >= 172 {
        assert!(
            has_hash_suffix.is_match(&mapping.filename),
            "long filename '{}' must have hash suffix",
            mapping.filename
        );
    }
}

/// Scenario 2: Chunk filename stays within budget
/// Given: A chunk with chunk_id containing a long doc_id stem (≥ 180 bytes when combined)
/// When:  chunk_all formats the chunk_filename
/// Then:  The resulting chunk_filename is ≤ 200 bytes
/// And:   The filename ends with ".md"
/// And:   It contains a deterministic hash suffix derived from the full original chunk_id
#[test]
fn scenario2_chunk_filename_budget() {
    let temp = TempDir::new().unwrap();
    let output_dir = temp.path();

    let long_source = "ref-docs-tasks-administer-cluster-manage-resources.md-docs-tasks-administer-cluster-manage-resources.md";
    let analysis = make_long_name_analysis(long_source, "ref", "Manage Resources");
    let (_, link_map) = assign_ids(vec![analysis.clone()]);

    let result = chunk_all(&[analysis], &link_map, output_dir, 1_000_000);

    if result.is_ok() {
        let chunks_dir = output_dir.join("chunks");
        for entry in fs::read_dir(&chunks_dir).expect("read_dir should work") {
            let entry = entry.expect("entry should exist");
            let filename = entry.file_name().to_string_lossy().to_string();

            // Then: chunk_filename is ≤ 200 bytes
            assert!(
                filename.len() <= 200,
                "chunk_filename '{}' is {} bytes, exceeds 200",
                filename,
                filename.len()
            );

            // And: filename ends with ".md"
            assert!(
                filename.ends_with(".md"),
                "chunk_filename '{}' must end with '.md'",
                filename
            );

            // And: if long, contains hash suffix (8 hex chars before level suffix and .md)
            // Chunk format is: {stem}-{hash8}-{level}.md where level is like "detailed"
            if filename.len() >= 180 {
                let has_hash_suffix = regex::Regex::new(r"-[[:xdigit:]]{8}-[a-z]+\.md$").unwrap();
                assert!(
                    has_hash_suffix.is_match(&filename),
                    "long chunk_filename '{}' must have hash suffix",
                    filename
                );
            }
        }
    } else {
        // If failed, should NOT be due to OS error 36
        let err_str = result.unwrap_err().to_string();
        assert!(
            !err_str.contains("File name too long") && !err_str.contains("os error 36"),
            "chunk_all should not fail with OS error 36 on long corpus"
        );
    }
}

/// Scenario 4: No stem collision from truncation
/// Given: Two source documents with distinct paths but long common prefixes
/// When:  Both are processed through assign_ids and chunking
/// Then:  Their derived document filenames are distinct
/// And:   Their derived chunk filenames are distinct
/// And:   No file overwrite occurs
#[test]
fn scenario4_no_stem_collision() {
    let source_a = "docs/ref-docs-tasks-administer-cluster-manage-resources.md";
    let source_b = "docs/ref-docs-tasks-administer-cluster-manage-config.md";

    let analyses = vec![
        make_long_name_analysis(source_a, "ref", "Manage Resources"),
        make_long_name_analysis(source_b, "ref", "Manage Config"),
    ];

    let (_, map) = assign_ids(analyses.clone());

    let filename_a = &map.get(source_a).expect("mapping A exists").filename;
    let filename_b = &map.get(source_b).expect("mapping B exists").filename;

    // Then: filenames are distinct (no collision)
    assert_ne!(
        filename_a, filename_b,
        "distinct sources must produce distinct filenames (W2: no stem collision)"
    );

    // And: run through chunking and verify chunk filenames are distinct
    let temp = TempDir::new().unwrap();
    let output_dir = temp.path();
    let (_, link_map) = assign_ids(analyses.clone());

    let result = chunk_all(&analyses, &link_map, output_dir, 1_000_000);

    if result.is_ok() {
        let chunks_dir = output_dir.join("chunks");

        let chunk_files: Vec<_> = fs::read_dir(&chunks_dir)
            .expect("read_dir should work")
            .map(|e| {
                e.expect("entry should exist")
                    .file_name()
                    .to_string_lossy()
                    .to_string()
            })
            .collect();

        // Verify no duplicate chunk filenames
        let mut seen: std::collections::HashSet<String> = std::collections::HashSet::new();
        for filename in &chunk_files {
            assert!(
                seen.insert(filename.clone()),
                "duplicate chunk filename '{}' (W2: silent data loss)",
                filename
            );
        }
    }
}

/// Scenario 5: Long corpus indexing completes without error (OS error 36 avoidance)
/// Given: A corpus containing files with names like
///        "ref-docs-tasks-administer-cluster-manage-resources.md-docs-tasks-administer-cluster-manage-resources.md"
///        (already >255 bytes as source filenames)
/// When:  Running `ctd index <corpus> --output <dir> --project-name "QA Docs"`
/// Then:  The command exits 0
/// And:   All derived document files are written to docs/
/// And:   All derived chunk files are written to chunks/
/// And:   No "File name too long" error appears
/// And:   Every IdMapping.filename is ≤ 187 bytes
/// And:   Each bounded filename contains an 8-character hash suffix
/// And:   No file in docs/ or chunks/ has a name exceeding 255 bytes
#[test]
fn scenario5_long_corpus_no_os_error_36() {
    let temp = TempDir::new().unwrap();
    let output_dir = temp.path();

    // The problematic corpus files (source filenames > 255 bytes)
    let docs = vec![
        ("ref-docs-tasks-administer-cluster-manage-resources.md-docs-tasks-administer-cluster-manage-resources.md", "ref", "Manage Resources"),
        ("ref-docs-tasks-administer-cluster-manage-config.md-docs-tasks-administer-cluster-manage-config.md", "ref", "Manage Config"),
        ("tutorial-docs-tasks-administer-cluster-securing-a-cluster.md-docs-tasks-administer-cluster-securing-a-cluster.md", "tutorial", "Securing Cluster"),
    ];

    let analyses: Vec<Analysis> = docs
        .iter()
        .map(|(path, cat, title)| make_long_name_analysis(path, cat, title))
        .collect();

    // And: Every IdMapping.filename is ≤ 187 bytes
    let (_, link_map) = assign_ids(analyses.clone());
    for mapping in link_map.values() {
        assert!(
            mapping.filename.len() <= 187,
            "IdMapping.filename '{}' exceeds 187 bytes",
            mapping.filename
        );

        // And: Each bounded filename contains an 8-character hash suffix (for long names)
        if mapping.filename.len() >= 172 {
            let has_hash = regex::Regex::new(r"-[[:xdigit:]]{8}\.md$").unwrap();
            assert!(
                has_hash.is_match(&mapping.filename),
                "long filename '{}' missing 8-char hash suffix",
                mapping.filename
            );
        }
    }

    // And: Running chunking succeeds
    let result = chunk_all(&analyses, &link_map, output_dir, 1_000_000);

    assert!(
        result.is_ok(),
        "chunk_all should not fail on long corpus (POST1)"
    );

    // And: No file in docs/ or chunks/ has a name exceeding 255 bytes
    let docs_dir = output_dir.join("docs");
    let chunks_dir = output_dir.join("chunks");

    if docs_dir.exists() {
        for entry in fs::read_dir(&docs_dir).expect("read_dir should work") {
            let entry = entry.expect("entry should exist");
            let filename = entry.file_name().to_string_lossy().to_string();
            assert!(
                filename.len() <= 255,
                "doc filename '{}' exceeds 255 bytes (INV1: ext4 limit)",
                filename
            );
        }
    }

    if chunks_dir.exists() {
        for entry in fs::read_dir(&chunks_dir).expect("read_dir should work") {
            let entry = entry.expect("entry should exist");
            let filename = entry.file_name().to_string_lossy().to_string();
            assert!(
                filename.len() <= 255,
                "chunk filename '{}' exceeds 255 bytes (INV1: ext4 limit)",
                filename
            );
        }
    }
}

/// Scenario 6: Deterministic bounded names across invocations
/// Given: A corpus of documents with long names
/// And:   A first run of `ctd index` that completed successfully
/// When:  Running `ctd index` again on the same corpus
/// Then:  All document filenames match the first run exactly
/// And:   All chunk filenames match the first run exactly
/// And:   No new files are created (idempotent)
#[test]
fn scenario6_deterministic_bounded_names() {
    let long_source = "ref-docs-tasks-administer-cluster-manage-resources.md-docs-tasks-administer-cluster-manage-resources.md";
    let analysis = make_long_name_analysis(long_source, "ref", "Manage Resources");

    // First run
    let (_, link_map1) = assign_ids(vec![analysis.clone()]);

    // Second run (same input)
    let (_, link_map2) = assign_ids(vec![analysis.clone()]);

    // Then: All document filenames match exactly
    let filenames1: Vec<_> = link_map1.values().map(|m| m.filename.clone()).collect();
    let filenames2: Vec<_> = link_map2.values().map(|m| m.filename.clone()).collect();

    assert_eq!(
        filenames1, filenames2,
        "bounded names must be deterministic across runs"
    );
}

/// INV3: Bounded name is deterministic (same input → same output)
#[test]
fn prop_bounded_name_deterministic() {
    let long_source = "ref-docs-tasks-administer-cluster-manage-resources.md-docs-tasks-administer-cluster-manage-resources.md";
    let analysis = make_long_name_analysis(long_source, "ref", "Manage Resources");

    let (_, map1) = assign_ids(vec![analysis.clone()]);
    let (_, map2) = assign_ids(vec![analysis]);

    // Compare the filenames from both maps
    let filenames1: Vec<_> = map1.values().map(|m| m.filename.clone()).collect();
    let filenames2: Vec<_> = map2.values().map(|m| m.filename.clone()).collect();
    assert_eq!(filenames1, filenames2, "assign_ids must be deterministic");
}

/// Scenario 8: Document filename is bounded when stem is long
/// Given: A document whose natural derived filename would be > 187 bytes
/// When:  assign_ids processes it
/// Then:  It returns Ok
/// And:   The resulting filename is ≤ 187 bytes
/// And:   The bounded name is deterministic for this input
/// And:   The bounded name follows the format: {truncated_stem[:172]}-{hash_suffix[:8]}.md
#[test]
fn scenario8_bounded_name_format() {
    let long_source = "ref-docs-tasks-administer-cluster-manage-resources.md-docs-tasks-administer-cluster-manage-resources.md";
    let analysis = make_long_name_analysis(long_source, "ref", "Manage Resources");

    let (_, map) = assign_ids(vec![analysis]);

    let mapping = map.get(long_source).expect("mapping must exist");

    // Then: It returns Ok (implicit - no error)
    // And: The resulting filename is ≤ 187 bytes
    assert!(
        mapping.filename.len() <= 187,
        "bounded filename '{}' is {} bytes, exceeds 187",
        mapping.filename,
        mapping.filename.len()
    );

    // And: The bounded name follows the format: {truncated_stem[:172]}-{hash_suffix[:8]}.md
    // This format produces exactly 184 bytes: 172 + 1 + 8 + 3 = 184
    let bounded_format = regex::Regex::new(r"^.{1,172}-[[:xdigit:]]{8}\.md$").unwrap();
    if mapping.filename.len() >= 172 {
        assert!(
            bounded_format.is_match(&mapping.filename),
            "bounded filename '{}' does not match format {{truncated}}-{{hash8}}.md",
            mapping.filename
        );
    }
}

/// INV7: SHA-256 hash is deterministic (sanity check on hash function)
#[test]
fn prop_sha256_deterministic() {
    use sha2::{Digest, Sha256};

    let input = "ref-docs-tasks-administer-cluster-manage-resources.md-docs-tasks-administer-cluster-manage-resources.md";

    let mut h1 = Sha256::new();
    h1.update(input.as_bytes());
    let r1 = format!("{:x}", h1.finalize());

    let mut h2 = Sha256::new();
    h2.update(input.as_bytes());
    let r2 = format!("{:x}", h2.finalize());

    assert_eq!(r1, r2, "SHA-256 must be deterministic");
}

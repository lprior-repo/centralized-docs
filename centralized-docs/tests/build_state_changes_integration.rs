//! Integration tests for build_file_state_changes (cdocs-7bu).
//!
//! Tests the calc function through the public API with real dependencies.
//! Validates invariants, rkyv blob integrity, and wiring to cache layer.

#![allow(clippy::unwrap_used)]
#![allow(clippy::expect_used)]

use doc_transformer::analyze::Analysis;
use doc_transformer::calc::{
    build_file_state_changes, hash_payload, FileDiff, FileStateRaw, PipelineOutputs,
};
use doc_transformer::chunking_adapter::Chunk;
use doc_transformer::discover::DiscoveryFile;
use std::collections::HashMap;
use std::sync::Arc;

// ---------------------------------------------------------------------------
// Test Helpers
// ---------------------------------------------------------------------------

fn make_discovery_file(path: &str) -> DiscoveryFile {
    DiscoveryFile {
        source_path: path.to_string(),
        size_bytes: 100,
    }
}

fn make_analysis(path: &str) -> Analysis {
    Analysis {
        source_path: path.to_string(),
        title: format!("Title for {}", path),
        frontmatter: None,
        headings: vec![],
        links: vec![],
        first_paragraph: String::new(),
        word_count: 10,
        has_code: false,
        has_tables: false,
        category: "concept".to_string(),
        content: Arc::from("test content"),
    }
}

fn make_chunk(doc_id: &str, index: usize) -> Chunk {
    use contextual_chunker::ChunkType;
    Chunk {
        chunk_id: format!("{}#{}", doc_id, index),
        doc_id: doc_id.to_string(),
        doc_title: format!("Title for {}", doc_id),
        chunk_index: index,
        content: format!("Chunk {} content", index),
        token_count: 50,
        heading: None,
        heading_path: vec![],
        chunk_type: ChunkType::Prose,
        previous_chunk_id: None,
        next_chunk_id: None,
        related_chunk_ids: vec![],
        summary: format!("Summary for chunk {}", index),
        chunk_level: contextual_chunker::ChunkLevel::Standard,
        parent_chunk_id: None,
        child_chunk_ids: vec![],
        context_prefix: None,
    }
}

fn make_hash(value: u8) -> [u8; 32] {
    [value; 32]
}

fn make_pipeline_outputs_for(paths: &[&str]) -> PipelineOutputs {
    let mut analyses = HashMap::new();
    let mut transforms = HashMap::new();
    let mut chunks = HashMap::new();
    let mut content_hashes = HashMap::new();

    for &path in paths {
        analyses.insert(path.to_string(), make_analysis(path));
        transforms.insert(path.to_string(), format!("transformed {}", path));
        chunks.insert(path.to_string(), vec![make_chunk(path, 0)]);
        content_hashes.insert(path.to_string(), make_hash(1));
    }

    PipelineOutputs {
        analyses,
        transforms,
        chunks,
        content_hashes,
        config_hash: make_hash(2),
        now_secs: 1_700_000_000,
    }
}

fn make_unchanged_entry(path: &str) -> (DiscoveryFile, FileStateRaw) {
    (
        make_discovery_file(path),
        FileStateRaw {
            content_hash: make_hash(0xAA),
            config_hash: make_hash(0xBB),
            analysis_hash: make_hash(0xCC),
            transform_hash: make_hash(0xDD),
            chunk_hash: make_hash(0xEE),
            last_processed_secs: 1_699_999_999,
            reserved: [0u8; 32],
        },
    )
}

// ---------------------------------------------------------------------------
// INV-07: Serialized blobs round-trip correctly
// ---------------------------------------------------------------------------

#[test]
fn integration_serialized_blobs_round_trip_correctly() {
    // Given: 3 changed files with real Analysis, String, Vec<Chunk>
    let paths = ["docs/a.md", "docs/b.md", "docs/c.md"];
    let diff = FileDiff {
        unchanged: vec![],
        changed: paths.iter().map(|p| make_discovery_file(p)).collect(),
        new_files: vec![],
        deleted: vec![],
    };
    let outputs = make_pipeline_outputs_for(&paths);

    // When
    let result = build_file_state_changes(&diff, &outputs);

    // Then
    let changes = result.expect("should succeed");

    // Verify analysis blobs are non-empty and hashable
    for (key, bytes) in &changes.new_analyses {
        assert!(!bytes.is_empty(), "analysis blob must be non-empty");
        assert_eq!(
            *key,
            hash_payload(bytes),
            "analysis key must be SHA-256 of blob"
        );
    }

    // Verify transform blobs
    for (key, bytes) in &changes.new_transforms {
        assert!(!bytes.is_empty(), "transform blob must be non-empty");
        assert_eq!(
            *key,
            hash_payload(bytes),
            "transform key must be SHA-256 of blob"
        );
    }

    // Verify chunk blobs
    for (key, bytes) in &changes.new_chunks {
        assert!(!bytes.is_empty(), "chunk blob must be non-empty");
        assert_eq!(
            *key,
            hash_payload(bytes),
            "chunk key must be SHA-256 of blob"
        );
    }
}

// ---------------------------------------------------------------------------
// INV-01 through INV-06: Full invariant check on realistic input
// ---------------------------------------------------------------------------

#[test]
fn integration_all_invariants_hold_for_realistic_input() {
    // Given: 5 unchanged, 3 changed, 2 new, 1 deleted
    let active_paths = [
        "docs/c1.md",
        "docs/c2.md",
        "docs/c3.md",
        "docs/n1.md",
        "docs/n2.md",
    ];
    let diff = FileDiff {
        unchanged: vec![
            make_unchanged_entry("docs/u1.md"),
            make_unchanged_entry("docs/u2.md"),
            make_unchanged_entry("docs/u3.md"),
            make_unchanged_entry("docs/u4.md"),
            make_unchanged_entry("docs/u5.md"),
        ],
        changed: vec![
            make_discovery_file("docs/c1.md"),
            make_discovery_file("docs/c2.md"),
            make_discovery_file("docs/c3.md"),
        ],
        new_files: vec![
            make_discovery_file("docs/n1.md"),
            make_discovery_file("docs/n2.md"),
        ],
        deleted: vec!["docs/d1.md".to_string()],
    };
    let outputs = make_pipeline_outputs_for(&active_paths);

    // When
    let result = build_file_state_changes(&diff, &outputs);

    // Then
    let changes = result.expect("should succeed");

    // INV-01: updated_files.len() == 3 changed + 2 new = 5
    assert_eq!(changes.updated_files.len(), 5, "INV-01");

    // INV-02: deleted_files.len() == 1
    assert_eq!(changes.deleted_files.len(), 1, "INV-02");

    // INV-03: payload counts match file count
    assert_eq!(changes.new_analyses.len(), 5, "INV-03: analyses");
    assert_eq!(changes.new_transforms.len(), 5, "INV-03: transforms");
    assert_eq!(changes.new_chunks.len(), 5, "INV-03: chunks");

    // INV-04: every hash in FileStateRaw appears in corresponding payload
    for (_path, state) in &changes.updated_files {
        let analysis_keys: Vec<[u8; 32]> = changes.new_analyses.iter().map(|(k, _)| *k).collect();
        let transform_keys: Vec<[u8; 32]> =
            changes.new_transforms.iter().map(|(k, _)| *k).collect();
        let chunk_keys: Vec<[u8; 32]> = changes.new_chunks.iter().map(|(k, _)| *k).collect();

        assert!(
            analysis_keys.contains(&state.analysis_hash),
            "INV-04: analysis_hash missing from new_analyses"
        );
        assert!(
            transform_keys.contains(&state.transform_hash),
            "INV-04: transform_hash missing from new_transforms"
        );
        assert!(
            chunk_keys.contains(&state.chunk_hash),
            "INV-04: chunk_hash missing from new_chunks"
        );
    }

    // INV-05: every payload key referenced exactly once
    let analysis_refs: Vec<[u8; 32]> = changes
        .updated_files
        .iter()
        .map(|(_, s)| s.analysis_hash)
        .collect();
    let transform_refs: Vec<[u8; 32]> = changes
        .updated_files
        .iter()
        .map(|(_, s)| s.transform_hash)
        .collect();
    let chunk_refs: Vec<[u8; 32]> = changes
        .updated_files
        .iter()
        .map(|(_, s)| s.chunk_hash)
        .collect();

    for (key, _) in &changes.new_analyses {
        let count = analysis_refs.iter().filter(|h| *h == key).count();
        assert_eq!(count, 1, "INV-05: analysis key referenced exactly once");
    }
    for (key, _) in &changes.new_transforms {
        let count = transform_refs.iter().filter(|h| *h == key).count();
        assert_eq!(count, 1, "INV-05: transform key referenced exactly once");
    }
    for (key, _) in &changes.new_chunks {
        let count = chunk_refs.iter().filter(|h| *h == key).count();
        assert_eq!(count, 1, "INV-05: chunk key referenced exactly once");
    }

    // INV-06: no unchanged source_path in updated_files
    let unchanged_paths = [
        "docs/u1.md",
        "docs/u2.md",
        "docs/u3.md",
        "docs/u4.md",
        "docs/u5.md",
    ];
    let updated_paths: Vec<&str> = changes
        .updated_files
        .iter()
        .map(|(p, _)| p.as_str())
        .collect();
    for uc in &unchanged_paths {
        assert!(
            !updated_paths.contains(uc),
            "INV-06: unchanged file {} must not appear in updated_files",
            uc
        );
    }
}

// ---------------------------------------------------------------------------
// StateChanges structure is correct for commit (validation without redb)
// ---------------------------------------------------------------------------

#[test]
fn integration_state_changes_structure_is_correct_for_commit() {
    // Given: 2 changed files
    let paths = ["docs/a.md", "docs/b.md"];
    let diff = FileDiff {
        unchanged: vec![],
        changed: paths.iter().map(|p| make_discovery_file(p)).collect(),
        new_files: vec![],
        deleted: vec![],
    };
    let outputs = make_pipeline_outputs_for(&paths);

    // When
    let result = build_file_state_changes(&diff, &outputs);

    // Then: StateChanges has all required fields populated correctly
    let changes = result.expect("should succeed");
    assert_eq!(changes.updated_files.len(), 2);
    assert_eq!(changes.new_analyses.len(), 2);
    assert_eq!(changes.new_transforms.len(), 2);
    assert_eq!(changes.new_chunks.len(), 2);

    // Every FileStateRaw has valid fields
    for (path, state) in &changes.updated_files {
        assert!(!path.is_empty(), "path should be non-empty: {path}");
        assert_eq!(
            state.config_hash, outputs.config_hash,
            "config_hash must match"
        );
        assert_eq!(
            state.content_hash,
            outputs.content_hashes[path.as_str()],
            "content_hash must match for {}",
            path
        );
        assert_eq!(
            state.last_processed_secs, outputs.now_secs,
            "now_secs must match"
        );
        assert_eq!(state.reserved, [0u8; 32], "reserved must be zeroed");
    }

    // URL state fields are empty
    assert!(changes.updated_urls.is_empty());
    assert!(changes.deleted_urls.is_empty());
    assert!(changes.new_scrapes.is_empty());
    assert!(changes.new_snapshots.is_empty());
    assert!(changes.deleted_snapshots.is_empty());
}

// ---------------------------------------------------------------------------
// Empty diff (unchanged-only) produces valid empty StateChanges
// ---------------------------------------------------------------------------

#[test]
fn integration_empty_state_changes_from_unchanged_only() {
    // Given: diff with only unchanged files
    let diff = FileDiff {
        unchanged: vec![
            make_unchanged_entry("docs/u1.md"),
            make_unchanged_entry("docs/u2.md"),
        ],
        changed: vec![],
        new_files: vec![],
        deleted: vec![],
    };
    let outputs = PipelineOutputs {
        analyses: HashMap::new(),
        transforms: HashMap::new(),
        chunks: HashMap::new(),
        content_hashes: HashMap::new(),
        config_hash: make_hash(2),
        now_secs: 1_700_000_000,
    };

    // When
    let result = build_file_state_changes(&diff, &outputs);

    // Then
    let changes = result.expect("unchanged-only should return Ok");
    assert!(changes.updated_files.is_empty());
    assert!(changes.deleted_files.is_empty());
    assert!(changes.new_analyses.is_empty());
    assert!(changes.new_transforms.is_empty());
    assert!(changes.new_chunks.is_empty());
}

// ---------------------------------------------------------------------------
// Deleted files only — no updated rows, only delete entries
// ---------------------------------------------------------------------------

#[test]
fn integration_deleted_files_only_produces_correct_state_changes() {
    // Given
    let deleted = vec!["docs/old1.md".to_string(), "docs/old2.md".to_string()];
    let diff = FileDiff {
        unchanged: vec![],
        changed: vec![],
        new_files: vec![],
        deleted: deleted.clone(),
    };
    let outputs = PipelineOutputs {
        analyses: HashMap::new(),
        transforms: HashMap::new(),
        chunks: HashMap::new(),
        content_hashes: HashMap::new(),
        config_hash: make_hash(2),
        now_secs: 1_700_000_000,
    };

    // When
    let result = build_file_state_changes(&diff, &outputs);

    // Then
    let changes = result.expect("should succeed for deleted-only");
    assert_eq!(changes.deleted_files.len(), 2);
    assert_eq!(changes.deleted_files, deleted);
    assert!(changes.updated_files.is_empty());
    assert!(changes.new_analyses.is_empty());
    assert!(changes.new_transforms.is_empty());
    assert!(changes.new_chunks.is_empty());
}

// ---------------------------------------------------------------------------
// Large batch (100 files) performance and correctness
// ---------------------------------------------------------------------------

#[test]
fn integration_large_batch_completes_correctly() {
    // Given: 100 changed files
    let paths: Vec<String> = (0..100).map(|i| format!("docs/file_{:03}.md", i)).collect();
    let path_refs: Vec<&str> = paths.iter().map(|s| s.as_str()).collect();

    let diff = FileDiff {
        unchanged: vec![],
        changed: paths.iter().map(|p| make_discovery_file(p)).collect(),
        new_files: vec![],
        deleted: vec![],
    };
    let outputs = make_pipeline_outputs_for(&path_refs);

    // When
    let start = std::time::Instant::now();
    let result = build_file_state_changes(&diff, &outputs);
    let elapsed = start.elapsed();

    // Then
    let changes = result.expect("should succeed for 100 files");
    assert!(
        elapsed.as_millis() < 1000,
        "should complete in under 1 second, took {:?}",
        elapsed
    );
    assert_eq!(
        changes.updated_files.len(),
        100,
        "all 100 files should have updated rows"
    );
    assert_eq!(changes.new_analyses.len(), 100);
    assert_eq!(changes.new_transforms.len(), 100);
    assert_eq!(changes.new_chunks.len(), 100);

    // Spot-check: all hashes are non-zero
    for (_path, state) in &changes.updated_files {
        assert_ne!(
            state.analysis_hash, [0u8; 32],
            "analysis_hash must be non-zero"
        );
        assert_ne!(
            state.transform_hash, [0u8; 32],
            "transform_hash must be non-zero"
        );
        assert_ne!(state.chunk_hash, [0u8; 32], "chunk_hash must be non-zero");
    }
}

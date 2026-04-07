use super::*;
use serde::{Deserialize, Serialize};
use tempfile::TempDir;

#[test]
fn test_snapshot_roundtrip() -> Result<()> {
    let config = CacheConfig::in_memory();
    let cache = DocCache::open(config)?;

    #[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
    struct Snapshot {
        url: String,
        count: usize,
    }

    let snap = Snapshot {
        url: "https://example.com".to_string(),
        count: 42,
    };
    let key = b"snapshot_key";
    cache.put_snapshot(key, &snap)?;
    let retrieved: Option<Snapshot> = cache.get_snapshot(key)?;
    assert_eq!(retrieved, Some(snap));
    Ok(())
}

#[test]
fn test_composite_hash_order_matters() {
    let hash1 = composite_hash(&[b"hello", b"world"]);
    let hash2 = composite_hash(&[b"world", b"hello"]);
    assert_ne!(
        hash1, hash2,
        "Different order should produce different hashes"
    );
}

#[test]
fn test_composite_hash_deterministic() {
    let hash1 = composite_hash(&[b"path/to/file.md", b"file content here"]);
    let hash2 = composite_hash(&[b"path/to/file.md", b"file content here"]);
    assert_eq!(hash1, hash2);
}

#[test]
fn test_analysis_roundtrip() -> Result<()> {
    let config = CacheConfig::in_memory();
    let cache = DocCache::open(config)?;

    #[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
    struct AnalysisData {
        source_path: String,
        title: String,
        category: String,
        word_count: usize,
    }

    let analysis = AnalysisData {
        source_path: "concept/general/test.md".to_string(),
        title: "Test Doc".to_string(),
        category: "concept".to_string(),
        word_count: 42,
    };
    let key = composite_hash(&[
        b"concept/general/test.md",
        b"file content bytes",
        b"config_hash",
    ]);
    cache.put(CacheType::Analysis, key.as_bytes(), &analysis)?;
    let retrieved: Option<AnalysisData> = cache.get(CacheType::Analysis, key.as_bytes())?;
    assert_eq!(retrieved, Some(analysis));
    Ok(())
}

#[test]
fn test_chunk_roundtrip() -> Result<()> {
    let config = CacheConfig::in_memory();
    let cache = DocCache::open(config)?;

    #[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
    struct ChunkData {
        chunk_id: String,
        doc_id: String,
        content: String,
        token_count: usize,
    }

    let chunks = vec![
        ChunkData {
            chunk_id: "doc#0".into(),
            doc_id: "doc".into(),
            content: "Summary".into(),
            token_count: 20,
        },
        ChunkData {
            chunk_id: "doc#1".into(),
            doc_id: "doc".into(),
            content: "Detailed".into(),
            token_count: 50,
        },
    ];
    let key = composite_hash(&[b"doc/path.md", b"file content bytes"]);
    cache.put(CacheType::Chunk, key.as_bytes(), &chunks)?;
    let retrieved: Option<Vec<ChunkData>> = cache.get(CacheType::Chunk, key.as_bytes())?;
    assert_eq!(retrieved, Some(chunks));
    Ok(())
}

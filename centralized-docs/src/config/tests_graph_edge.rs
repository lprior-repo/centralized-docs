//! Graph config boundary tests, edge cases, and serialization round-trips.

use std::fs;

use super::*;
use tempfile::TempDir;

// === BOUNDARY TESTS ===

#[test]
fn test_boundary_max_related_chunks_low() -> anyhow::Result<()> {
    for value in 1..=5 {
        let config = GraphConfig::with_params(value, 16, 200)?;
        assert_eq!(config.get_max_related_chunks(), value);
    }
    Ok(())
}

#[test]
fn test_boundary_max_related_chunks_high() -> anyhow::Result<()> {
    for value in 996..=1000 {
        let config = GraphConfig::with_params(value, 16, 200)?;
        assert_eq!(config.get_max_related_chunks(), value);
    }
    Ok(())
}

#[test]
fn test_boundary_hnsw_m_low() -> anyhow::Result<()> {
    for value in 4..=8 {
        let config = GraphConfig::with_params(20, value, 200)?;
        assert_eq!(config.get_hnsw_m(), value);
    }
    Ok(())
}

#[test]
fn test_boundary_hnsw_m_high() -> anyhow::Result<()> {
    for value in 60..=64 {
        let config = GraphConfig::with_params(20, value, 200)?;
        assert_eq!(config.get_hnsw_m(), value);
    }
    Ok(())
}

#[test]
fn test_boundary_hnsw_ef_construction_low() -> anyhow::Result<()> {
    for value in 50..=55 {
        let config = GraphConfig::with_params(20, 16, value)?;
        assert_eq!(config.get_hnsw_ef_construction(), value);
    }
    Ok(())
}

#[test]
fn test_boundary_hnsw_ef_construction_high() -> anyhow::Result<()> {
    for value in 995..=1000 {
        let config = GraphConfig::with_params(20, 16, value)?;
        assert_eq!(config.get_hnsw_ef_construction(), value);
    }
    Ok(())
}

// === LOAD FROM FILE EDGE CASES ===

#[test]
fn test_load_yaml_missing_file() -> anyhow::Result<()> {
    let temp = TempDir::new()?;
    let config_path = temp.path().join("nonexistent.yaml");
    let result = GraphConfig::load_from_file(&config_path);
    assert!(result.is_err());
    Ok(())
}

#[test]
fn test_load_yaml_malformed() -> anyhow::Result<()> {
    let yaml_content = r#"
max_related_chunks: "not_a_number"
hnsw_m: 16
hnsw_ef_construction: 200
"#;
    let temp = TempDir::new()?;
    let config_path = temp.path().join("graph_config.yaml");
    fs::write(&config_path, yaml_content)?;

    let result = GraphConfig::load_from_file(&config_path);
    assert!(result.is_err());
    Ok(())
}

#[test]
fn test_load_yaml_all_defaults_work() {
    let config = GraphConfig::default();
    assert!(config.get_max_related_chunks() >= 1 && config.get_max_related_chunks() <= 1000);
    assert!(config.get_hnsw_m() >= 4 && config.get_hnsw_m() <= 64);
    assert!(config.get_hnsw_ef_construction() >= 50 && config.get_hnsw_ef_construction() <= 1000);
}

#[test]
fn test_clone_preserves_validation() -> anyhow::Result<()> {
    let config = GraphConfig::with_params(100, 32, 500)?;
    let cloned = config.clone();
    assert_eq!(cloned.max_related_chunks.get(), 100);
    assert_eq!(cloned.hnsw_m.get(), 32);
    assert_eq!(cloned.hnsw_ef_construction.get(), 500);
    Ok(())
}

#[test]
fn test_serialization_round_trip() -> anyhow::Result<()> {
    let config = GraphConfig::with_params(50, 24, 350)?;
    let yaml = serde_yaml::to_string(&config)?;
    let deserialized: GraphConfig = serde_yaml::from_str(&yaml)?;
    assert_eq!(
        config.get_max_related_chunks(),
        deserialized.max_related_chunks
    );
    assert_eq!(config.get_hnsw_m(), deserialized.hnsw_m);
    assert_eq!(
        config.get_hnsw_ef_construction(),
        deserialized.hnsw_ef_construction
    );
    Ok(())
}

//! Tests for graph configuration and HNSW parameter validation.

use std::fs;

use super::*;
use tempfile::TempDir;

// === VALID CONFIGURATION TESTS ===

#[test]
fn test_graph_config_default() {
    let config = GraphConfig::new();
    assert_eq!(config.get_max_related_chunks(), 20);
    assert_eq!(config.get_hnsw_m(), 16);
    assert_eq!(config.get_hnsw_ef_construction(), 200);
}

#[test]
fn test_graph_config_default_trait() {
    let config = GraphConfig::default();
    assert_eq!(config.get_max_related_chunks(), 20);
    assert_eq!(config.get_hnsw_m(), 16);
    assert_eq!(config.get_hnsw_ef_construction(), 200);
}

#[test]
fn test_graph_config_load_valid_yaml() -> anyhow::Result<()> {
    let yaml_content = r"
max_related_chunks: 25
hnsw_m: 20
hnsw_ef_construction: 300
";
    let temp = TempDir::new()?;
    let config_path = temp.path().join("graph_config.yaml");
    fs::write(&config_path, yaml_content)?;

    let config = GraphConfig::load_from_file(&config_path)?;
    assert_eq!(config.get_max_related_chunks(), 25);
    assert_eq!(config.get_hnsw_m(), 20);
    assert_eq!(config.get_hnsw_ef_construction(), 300);
    Ok(())
}

#[test]
fn test_graph_config_with_params_valid() -> anyhow::Result<()> {
    let config = GraphConfig::with_params(50, 32, 400)?;
    assert_eq!(config.get_max_related_chunks(), 50);
    assert_eq!(config.get_hnsw_m(), 32);
    assert_eq!(config.get_hnsw_ef_construction(), 400);
    Ok(())
}

#[test]
fn test_graph_config_with_params_min_values() -> anyhow::Result<()> {
    let config = GraphConfig::with_params(1, 4, 50)?;
    assert_eq!(config.get_max_related_chunks(), 1);
    assert_eq!(config.get_hnsw_m(), 4);
    assert_eq!(config.get_hnsw_ef_construction(), 50);
    Ok(())
}

#[test]
fn test_graph_config_with_params_max_values() -> anyhow::Result<()> {
    let config = GraphConfig::with_params(1000, 64, 1000)?;
    assert_eq!(config.get_max_related_chunks(), 1000);
    assert_eq!(config.get_hnsw_m(), 64);
    assert_eq!(config.get_hnsw_ef_construction(), 1000);
    Ok(())
}

// === max_related_chunks VALIDATION TESTS ===

#[test]
fn test_reject_max_related_chunks_zero() {
    let result = GraphConfig::with_params(0, 16, 200);
    assert!(result.is_err());
    if let Err(e) = result {
        let err_msg = e.to_string();
        assert!(err_msg.contains("max_related_chunks"));
        assert!(err_msg.contains("at least 1"));
    }
}

#[test]
fn test_reject_max_related_chunks_too_large() {
    let result = GraphConfig::with_params(1001, 16, 200);
    assert!(result.is_err());
    if let Err(e) = result {
        let err_msg = e.to_string();
        assert!(err_msg.contains("max_related_chunks"));
        assert!(err_msg.contains("at most 1000"));
    }
}

#[test]
fn test_reject_max_related_chunks_way_too_large() {
    let result = GraphConfig::with_params(1_000_000, 16, 200);
    assert!(result.is_err());
    if let Err(e) = result {
        let err_msg = e.to_string();
        assert!(err_msg.contains("max_related_chunks"));
    }
}

#[test]
fn test_load_yaml_max_related_chunks_zero() -> anyhow::Result<()> {
    let yaml_content = r"
max_related_chunks: 0
hnsw_m: 16
hnsw_ef_construction: 200
";
    let temp = TempDir::new()?;
    let config_path = temp.path().join("graph_config.yaml");
    fs::write(&config_path, yaml_content)?;

    let result = GraphConfig::load_from_file(&config_path);
    assert!(result.is_err());
    if let Err(e) = result {
        let err_msg = e.to_string();
        assert!(err_msg.contains("max_related_chunks"));
    }
    Ok(())
}

// === hnsw_m VALIDATION TESTS ===

#[test]
fn test_reject_hnsw_m_too_small() {
    let result = GraphConfig::with_params(20, 3, 200);
    assert!(result.is_err());
    if let Err(e) = result {
        let err_msg = e.to_string();
        assert!(err_msg.contains("hnsw_m"));
        assert!(err_msg.contains("at least 4"));
    }
}

#[test]
fn test_reject_hnsw_m_zero() {
    let result = GraphConfig::with_params(20, 0, 200);
    assert!(result.is_err());
    if let Err(e) = result {
        let err_msg = e.to_string();
        assert!(err_msg.contains("hnsw_m"));
    }
}

#[test]
fn test_reject_hnsw_m_too_large() {
    let result = GraphConfig::with_params(20, 65, 200);
    assert!(result.is_err());
    if let Err(e) = result {
        let err_msg = e.to_string();
        assert!(err_msg.contains("hnsw_m"));
        assert!(err_msg.contains("at most 64"));
    }
}

#[test]
fn test_reject_hnsw_m_way_too_large() {
    let result = GraphConfig::with_params(20, 256, 200);
    assert!(result.is_err());
    if let Err(e) = result {
        let err_msg = e.to_string();
        assert!(err_msg.contains("hnsw_m"));
    }
}

#[test]
fn test_load_yaml_hnsw_m_too_small() -> anyhow::Result<()> {
    let yaml_content = r"
max_related_chunks: 20
hnsw_m: 2
hnsw_ef_construction: 200
";
    let temp = TempDir::new()?;
    let config_path = temp.path().join("graph_config.yaml");
    fs::write(&config_path, yaml_content)?;

    let result = GraphConfig::load_from_file(&config_path);
    assert!(result.is_err());
    if let Err(e) = result {
        let err_msg = e.to_string();
        assert!(err_msg.contains("hnsw_m"));
    }
    Ok(())
}

// === hnsw_ef_construction VALIDATION TESTS ===

#[test]
fn test_reject_hnsw_ef_construction_too_small() {
    let result = GraphConfig::with_params(20, 16, 49);
    assert!(result.is_err());
    if let Err(e) = result {
        let err_msg = e.to_string();
        assert!(err_msg.contains("hnsw_ef_construction"));
        assert!(err_msg.contains("at least 50"));
    }
}

#[test]
fn test_reject_hnsw_ef_construction_zero() {
    let result = GraphConfig::with_params(20, 16, 0);
    assert!(result.is_err());
    if let Err(e) = result {
        let err_msg = e.to_string();
        assert!(err_msg.contains("hnsw_ef_construction"));
    }
}

#[test]
fn test_reject_hnsw_ef_construction_too_large() {
    let result = GraphConfig::with_params(20, 16, 1001);
    assert!(result.is_err());
    if let Err(e) = result {
        let err_msg = e.to_string();
        assert!(err_msg.contains("hnsw_ef_construction"));
        assert!(err_msg.contains("at most 1000"));
    }
}

#[test]
fn test_reject_hnsw_ef_construction_way_too_large() {
    let result = GraphConfig::with_params(20, 16, 10000);
    assert!(result.is_err());
    if let Err(e) = result {
        let err_msg = e.to_string();
        assert!(err_msg.contains("hnsw_ef_construction"));
    }
}

#[test]
fn test_load_yaml_hnsw_ef_construction_too_small() -> anyhow::Result<()> {
    let yaml_content = r"
max_related_chunks: 20
hnsw_m: 16
hnsw_ef_construction: 25
";
    let temp = TempDir::new()?;
    let config_path = temp.path().join("graph_config.yaml");
    fs::write(&config_path, yaml_content)?;

    let result = GraphConfig::load_from_file(&config_path);
    assert!(result.is_err());
    if let Err(e) = result {
        let err_msg = e.to_string();
        assert!(err_msg.contains("hnsw_ef_construction"));
    }
    Ok(())
}

// === MULTIPLE PARAMETER FAILURES ===

#[test]
fn test_reject_all_parameters_invalid() {
    let result = GraphConfig::with_params(0, 2, 25);
    assert!(result.is_err());
    // Should report the first validation failure (max_related_chunks)
    if let Err(e) = result {
        let err_msg = e.to_string();
        assert!(err_msg.contains("max_related_chunks"));
    }
}

#[test]
fn test_load_yaml_multiple_invalid_parameters() -> anyhow::Result<()> {
    let yaml_content = r"
max_related_chunks: 2000
hnsw_m: 100
hnsw_ef_construction: 10000
";
    let temp = TempDir::new()?;
    let config_path = temp.path().join("graph_config.yaml");
    fs::write(&config_path, yaml_content)?;

    let result = GraphConfig::load_from_file(&config_path);
    assert!(result.is_err());
    // Should report the first validation failure
    if let Err(e) = result {
        let err_msg = e.to_string();
        // ConfigError provides specific error messages
        assert!(err_msg.contains("max_related_chunks") || err_msg.contains("too large"));
    }
    Ok(())
}

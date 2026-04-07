//! Tests for category configuration and validation.

use std::fs;

use super::category_config::is_valid_category_name;
use super::*;
use tempfile::TempDir;

#[test]
fn test_valid_category_names() {
    assert!(is_valid_category_name("meta"));
    assert!(is_valid_category_name("tutorial"));
    assert!(is_valid_category_name("api-reference"));
    assert!(is_valid_category_name("type_system"));
    assert!(is_valid_category_name("ref123"));
}

#[test]
fn test_detect_category_by_filename() {
    let config = CategoryConfig {
        default_category: "concept".to_string(),
        rules: vec![CategoryRule {
            category: "meta".to_string(),
            criteria: MatchCriteria {
                filename: Some(vec!["readme".to_string()]),
                ..Default::default()
            },
        }],
    };
    assert_eq!(config.detect_category("readme.md", "", ""), "meta");
}

#[test]
fn test_reject_rule_with_all_none_criteria() -> anyhow::Result<()> {
    let config_yaml = r#"
default_category: "concept"
rules:
  - category: "api"
"#;
    let temp = TempDir::new()?;
    let config_path = temp.path().join("config.yaml");
    fs::write(&config_path, config_yaml)?;
    let result = CategoryConfig::load_from_file(&config_path);
    assert!(result.is_err());
    if let Err(e) = result {
        let err_msg = e.to_string();
        assert!(err_msg.contains("no criteria"));
        assert!(err_msg.contains("api"));
    }
    Ok(())
}

#[test]
fn test_reject_rule_with_all_empty_arrays() -> anyhow::Result<()> {
    let config_yaml = r#"
default_category: "concept"
rules:
  - category: "api"
    filename: []
    content: []
    path: []
"#;
    let temp = TempDir::new()?;
    let config_path = temp.path().join("config.yaml");
    fs::write(&config_path, config_yaml)?;
    let result = CategoryConfig::load_from_file(&config_path);
    assert!(result.is_err());
    if let Err(e) = result {
        assert!(e.to_string().contains("no criteria"));
    }
    Ok(())
}

#[test]
fn test_reject_rule_with_empty_strings_only() -> anyhow::Result<()> {
    let config_yaml = r#"
default_category: "concept"
rules:
  - category: "tutorial"
    filename: [""]
    content: [""]
    path: [""]
"#;
    let temp = TempDir::new()?;
    let config_path = temp.path().join("config.yaml");
    fs::write(&config_path, config_yaml)?;
    let result = CategoryConfig::load_from_file(&config_path);
    assert!(result.is_err());
    Ok(())
}

#[test]
fn test_reject_rule_with_whitespace_only_strings() -> anyhow::Result<()> {
    let config_yaml = r#"
default_category: "concept"
rules:
  - category: "reference"
    filename: ["   "]
    content: [" "]
"#;
    let temp = TempDir::new()?;
    let config_path = temp.path().join("config.yaml");
    fs::write(&config_path, config_yaml)?;
    let result = CategoryConfig::load_from_file(&config_path);
    assert!(result.is_err());
    Ok(())
}

#[test]
fn test_accept_rule_with_single_filename_criterion() -> anyhow::Result<()> {
    let config_yaml = r#"
default_category: "concept"
rules:
  - category: "api"
    filename: ["reference"]
"#;
    let temp = TempDir::new()?;
    let config_path = temp.path().join("config.yaml");
    fs::write(&config_path, config_yaml)?;
    let config = CategoryConfig::load_from_file(&config_path)?;
    assert_eq!(config.rules.len(), 1);
    assert_eq!(config.rules[0].category, "api");
    Ok(())
}

#[test]
fn test_accept_rule_with_single_content_criterion() -> anyhow::Result<()> {
    let config_yaml = r#"
default_category: "concept"
rules:
  - category: "tutorial"
    content: ["example"]
"#;
    let temp = TempDir::new()?;
    let config_path = temp.path().join("config.yaml");
    fs::write(&config_path, config_yaml)?;
    assert!(CategoryConfig::load_from_file(&config_path).is_ok());
    Ok(())
}

#[test]
fn test_accept_rule_with_single_path_criterion() -> anyhow::Result<()> {
    let config_yaml = r#"
default_category: "concept"
rules:
  - category: "guide"
    path: ["/docs/guides/"]
"#;
    let temp = TempDir::new()?;
    let config_path = temp.path().join("config.yaml");
    fs::write(&config_path, config_yaml)?;
    assert!(CategoryConfig::load_from_file(&config_path).is_ok());
    Ok(())
}

#[test]
fn test_accept_rule_with_multiple_criteria() -> anyhow::Result<()> {
    let config_yaml = r#"
default_category: "concept"
rules:
  - category: "api"
    filename: ["reference", "swagger"]
    content: ["endpoint", "method"]
    path: ["/api/"]
"#;
    let temp = TempDir::new()?;
    let config_path = temp.path().join("config.yaml");
    fs::write(&config_path, config_yaml)?;
    assert!(CategoryConfig::load_from_file(&config_path).is_ok());
    Ok(())
}

#[test]
fn test_accept_rule_with_empty_criteria_but_some_populated() -> anyhow::Result<()> {
    let config_yaml = r#"
default_category: "concept"
rules:
  - category: "meta"
    filename: []
    content: ["metadata", "header"]
"#;
    let temp = TempDir::new()?;
    let config_path = temp.path().join("config.yaml");
    fs::write(&config_path, config_yaml)?;
    let config = CategoryConfig::load_from_file(&config_path)?;
    assert_eq!(config.rules[0].category, "meta");
    Ok(())
}

#[test]
fn test_multiple_rules_all_valid() -> anyhow::Result<()> {
    let config_yaml = r#"
default_category: "concept"
rules:
  - category: "api"
    filename: ["reference"]
  - category: "tutorial"
    content: ["example"]
  - category: "guide"
    path: ["/guides/"]
"#;
    let temp = TempDir::new()?;
    let config_path = temp.path().join("config.yaml");
    fs::write(&config_path, config_yaml)?;
    assert!(CategoryConfig::load_from_file(&config_path).is_ok());
    Ok(())
}

#[test]
fn test_multiple_rules_one_invalid() -> anyhow::Result<()> {
    let config_yaml = r#"
default_category: "concept"
rules:
  - category: "api"
    filename: ["reference"]
  - category: "broken"
  - category: "guide"
    path: ["/guides/"]
"#;
    let temp = TempDir::new()?;
    let config_path = temp.path().join("config.yaml");
    fs::write(&config_path, config_yaml)?;
    let result = CategoryConfig::load_from_file(&config_path);
    assert!(result.is_err());
    if let Err(e) = result {
        assert!(e.to_string().contains("broken"));
    }
    Ok(())
}

#[test]
fn test_has_valid_criteria_with_all_none() {
    assert!(!CategoryConfig::has_valid_criteria(
        &MatchCriteria::default()
    ));
}

#[test]
fn test_has_valid_criteria_with_empty_vecs() {
    let criteria = MatchCriteria {
        filename: Some(vec![]),
        content: Some(vec![]),
        path: Some(vec![]),
    };
    assert!(!CategoryConfig::has_valid_criteria(&criteria));
}

#[test]
fn test_has_valid_criteria_with_one_valid_filename() {
    let criteria = MatchCriteria {
        filename: Some(vec!["api".to_string()]),
        content: None,
        path: None,
    };
    assert!(CategoryConfig::has_valid_criteria(&criteria));
}

#[test]
fn test_has_valid_criteria_with_whitespace_strings() {
    let criteria = MatchCriteria {
        filename: Some(vec!["  ".to_string()]),
        content: Some(vec![" ".to_string()]),
        path: None,
    };
    assert!(!CategoryConfig::has_valid_criteria(&criteria));
}

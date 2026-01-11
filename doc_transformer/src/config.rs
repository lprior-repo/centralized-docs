use anyhow::Result;
use serde::{Deserialize, Serialize};
use std::fs;
use std::path::Path;

/// Configuration for category detection rules
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CategoryConfig {
    /// Default category to use if no rules match
    pub default_category: String,
    /// List of category detection rules (evaluated in order)
    pub rules: Vec<CategoryRule>,
}

/// A single category detection rule
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CategoryRule {
    /// The category name to assign if this rule matches
    pub category: String,
    /// Criteria for matching documents
    #[serde(flatten)]
    pub criteria: MatchCriteria,
}

/// Matching criteria for a rule (filename, content, or path patterns)
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct MatchCriteria {
    /// Case-insensitive patterns to match against filename (without extension)
    pub filename: Option<Vec<String>>,
    /// Case-insensitive patterns to match against content (substring match)
    pub content: Option<Vec<String>>,
    /// Patterns to match against file path (substring match)
    pub path: Option<Vec<String>>,
}

impl CategoryConfig {
    /// Load configuration from a YAML file
    pub fn load_from_file(path: &Path) -> Result<Self> {
        let content = fs::read_to_string(path)?;
        let config: CategoryConfig = serde_saphyr::from_str(&content)?;

        // Validate that default_category is set
        if config.default_category.is_empty() {
            anyhow::bail!("Config error: default_category is required and must not be empty");
        }

        // Validate that all categories are lowercase alphanumeric
        if !is_valid_category_name(&config.default_category) {
            anyhow::bail!(
                "Config error: default_category '{}' is not lowercase alphanumeric",
                config.default_category
            );
        }

        for rule in &config.rules {
            if !is_valid_category_name(&rule.category) {
                anyhow::bail!(
                    "Config error: category '{}' is not lowercase alphanumeric",
                    rule.category
                );
            }

            // Validate that rule has at least one non-empty criterion
            if !Self::has_valid_criteria(&rule.criteria) {
                anyhow::bail!(
                    "Config error: rule for category '{}' has no criteria (all are None or empty)",
                    rule.category
                );
            }
        }

        Ok(config)
    }

    /// Check if criteria has at least one non-empty, non-None criterion
    fn has_valid_criteria(criteria: &MatchCriteria) -> bool {
        // Check filename: must be Some with non-empty vec containing non-empty strings
        let has_filename = criteria.filename.as_ref()
            .map_or(false, |v| !v.is_empty() && v.iter().any(|s| !s.trim().is_empty()));

        // Check content: must be Some with non-empty vec containing non-empty strings
        let has_content = criteria.content.as_ref()
            .map_or(false, |v| !v.is_empty() && v.iter().any(|s| !s.trim().is_empty()));

        // Check path: must be Some with non-empty vec containing non-empty strings
        let has_path = criteria.path.as_ref()
            .map_or(false, |v| !v.is_empty() && v.iter().any(|s| !s.trim().is_empty()));

        // At least one criterion must be valid
        has_filename || has_content || has_path
    }

    /// Detect category for a document using these rules
    pub fn detect_category(
        &self,
        filename: &str,
        content: &str,
        path: &str,
    ) -> String {
        // Evaluate rules in order
        for rule in &self.rules {
            if self.matches_rule(filename, content, path, &rule.criteria) {
                return rule.category.clone();
            }
        }
        // Return default if no rules match
        self.default_category.clone()
    }

    /// Check if a document matches a rule's criteria
    fn matches_rule(
        &self,
        filename: &str,
        content: &str,
        path: &str,
        criteria: &MatchCriteria,
    ) -> bool {
        // Check filename patterns (case-insensitive)
        if let Some(patterns) = &criteria.filename {
            let fname_lower = Path::new(filename)
                .file_stem()
                .unwrap_or_default()
                .to_string_lossy()
                .to_lowercase();

            if patterns.iter().any(|p| {
                fname_lower.contains(&p.to_lowercase())
            }) {
                return true;
            }
        }

        // Check content patterns (case-insensitive, substring)
        if let Some(patterns) = &criteria.content {
            let content_lower = content.to_lowercase();
            if patterns.iter().any(|p| {
                content_lower.contains(&p.to_lowercase())
            }) {
                return true;
            }
        }

        // Check path patterns (case-sensitive substring)
        if let Some(patterns) = &criteria.path {
            if patterns.iter().any(|p| {
                path.contains(p)
            }) {
                return true;
            }
        }

        false
    }
}

/// Validate that a category name is lowercase alphanumeric
fn is_valid_category_name(name: &str) -> bool {
    !name.is_empty() && name.chars().all(|c| c.is_ascii_lowercase() || c.is_ascii_digit() || c == '_' || c == '-')
}

#[cfg(test)]
mod tests {
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
            rules: vec![
                CategoryRule {
                    category: "meta".to_string(),
                    criteria: MatchCriteria {
                        filename: Some(vec!["readme".to_string()]),
                        ..Default::default()
                    },
                }
            ],
        };
        assert_eq!(
            config.detect_category("readme.md", "", ""),
            "meta"
        );
    }

    // === VALIDATION TESTS ===

    #[test]
    fn test_reject_rule_with_all_none_criteria() {
        let config_yaml = r#"
default_category: "concept"
rules:
  - category: "api"
"#;

        let temp = TempDir::new().unwrap();
        let config_path = temp.path().join("config.yaml");
        fs::write(&config_path, config_yaml).unwrap();

        let result = CategoryConfig::load_from_file(&config_path);
        assert!(result.is_err());
        let err_msg = result.unwrap_err().to_string();
        assert!(err_msg.contains("no criteria"));
        assert!(err_msg.contains("api"));
    }

    #[test]
    fn test_reject_rule_with_all_empty_arrays() {
        let config_yaml = r#"
default_category: "concept"
rules:
  - category: "api"
    filename: []
    content: []
    path: []
"#;

        let temp = TempDir::new().unwrap();
        let config_path = temp.path().join("config.yaml");
        fs::write(&config_path, config_yaml).unwrap();

        let result = CategoryConfig::load_from_file(&config_path);
        assert!(result.is_err());
        let err_msg = result.unwrap_err().to_string();
        assert!(err_msg.contains("no criteria"));
        assert!(err_msg.contains("api"));
    }

    #[test]
    fn test_reject_rule_with_empty_strings_only() {
        let config_yaml = r#"
default_category: "concept"
rules:
  - category: "tutorial"
    filename: [""]
    content: [""]
    path: [""]
"#;

        let temp = TempDir::new().unwrap();
        let config_path = temp.path().join("config.yaml");
        fs::write(&config_path, config_yaml).unwrap();

        let result = CategoryConfig::load_from_file(&config_path);
        assert!(result.is_err());
        let err_msg = result.unwrap_err().to_string();
        assert!(err_msg.contains("no criteria"));
    }

    #[test]
    fn test_reject_rule_with_whitespace_only_strings() {
        let config_yaml = r#"
default_category: "concept"
rules:
  - category: "reference"
    filename: ["   "]
    content: [" "]
"#;

        let temp = TempDir::new().unwrap();
        let config_path = temp.path().join("config.yaml");
        fs::write(&config_path, config_yaml).unwrap();

        let result = CategoryConfig::load_from_file(&config_path);
        assert!(result.is_err());
        let err_msg = result.unwrap_err().to_string();
        assert!(err_msg.contains("no criteria"));
    }

    #[test]
    fn test_accept_rule_with_single_filename_criterion() {
        let config_yaml = r#"
default_category: "concept"
rules:
  - category: "api"
    filename: ["reference"]
"#;

        let temp = TempDir::new().unwrap();
        let config_path = temp.path().join("config.yaml");
        fs::write(&config_path, config_yaml).unwrap();

        let result = CategoryConfig::load_from_file(&config_path);
        assert!(result.is_ok());
        let config = result.unwrap();
        assert_eq!(config.rules.len(), 1);
        assert_eq!(config.rules[0].category, "api");
    }

    #[test]
    fn test_accept_rule_with_single_content_criterion() {
        let config_yaml = r#"
default_category: "concept"
rules:
  - category: "tutorial"
    content: ["example"]
"#;

        let temp = TempDir::new().unwrap();
        let config_path = temp.path().join("config.yaml");
        fs::write(&config_path, config_yaml).unwrap();

        let result = CategoryConfig::load_from_file(&config_path);
        assert!(result.is_ok());
    }

    #[test]
    fn test_accept_rule_with_single_path_criterion() {
        let config_yaml = r#"
default_category: "concept"
rules:
  - category: "guide"
    path: ["/docs/guides/"]
"#;

        let temp = TempDir::new().unwrap();
        let config_path = temp.path().join("config.yaml");
        fs::write(&config_path, config_yaml).unwrap();

        let result = CategoryConfig::load_from_file(&config_path);
        assert!(result.is_ok());
    }

    #[test]
    fn test_accept_rule_with_multiple_criteria() {
        let config_yaml = r#"
default_category: "concept"
rules:
  - category: "api"
    filename: ["reference", "swagger"]
    content: ["endpoint", "method"]
    path: ["/api/"]
"#;

        let temp = TempDir::new().unwrap();
        let config_path = temp.path().join("config.yaml");
        fs::write(&config_path, config_yaml).unwrap();

        let result = CategoryConfig::load_from_file(&config_path);
        assert!(result.is_ok());
    }

    #[test]
    fn test_accept_rule_with_empty_criteria_but_some_populated() {
        let config_yaml = r#"
default_category: "concept"
rules:
  - category: "meta"
    filename: []
    content: ["metadata", "header"]
"#;

        let temp = TempDir::new().unwrap();
        let config_path = temp.path().join("config.yaml");
        fs::write(&config_path, config_yaml).unwrap();

        let result = CategoryConfig::load_from_file(&config_path);
        assert!(result.is_ok());
        let config = result.unwrap();
        assert_eq!(config.rules[0].category, "meta");
    }

    #[test]
    fn test_multiple_rules_all_valid() {
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

        let temp = TempDir::new().unwrap();
        let config_path = temp.path().join("config.yaml");
        fs::write(&config_path, config_yaml).unwrap();

        let result = CategoryConfig::load_from_file(&config_path);
        assert!(result.is_ok());
        let config = result.unwrap();
        assert_eq!(config.rules.len(), 3);
    }

    #[test]
    fn test_multiple_rules_one_invalid() {
        let config_yaml = r#"
default_category: "concept"
rules:
  - category: "api"
    filename: ["reference"]
  - category: "broken"
  - category: "guide"
    path: ["/guides/"]
"#;

        let temp = TempDir::new().unwrap();
        let config_path = temp.path().join("config.yaml");
        fs::write(&config_path, config_yaml).unwrap();

        let result = CategoryConfig::load_from_file(&config_path);
        assert!(result.is_err());
        let err_msg = result.unwrap_err().to_string();
        assert!(err_msg.contains("broken"));
    }

    #[test]
    fn test_has_valid_criteria_with_all_none() {
        let criteria = MatchCriteria::default();
        assert!(!CategoryConfig::has_valid_criteria(&criteria));
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
}

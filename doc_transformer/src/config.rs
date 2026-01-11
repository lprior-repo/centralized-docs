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
        }

        Ok(config)
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
}

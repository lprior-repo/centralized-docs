//! Category detection rules and configuration.

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
///
/// `filename` and `content` patterns are pre-lowered at config load time for
/// zero-allocation matching. `path` patterns remain case-sensitive.
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct MatchCriteria {
    /// Pre-lowered patterns to match against lowercase filename stem (substring)
    pub filename: Option<Vec<String>>,
    /// Pre-lowered patterns to match against lowercase content (substring)
    pub content: Option<Vec<String>>,
    /// Patterns to match against file path (case-sensitive substring)
    pub path: Option<Vec<String>>,
}

impl CategoryConfig {
    /// Load configuration from a YAML file
    ///
    /// # Errors
    ///
    /// Returns an error if:
    /// - The file cannot be read
    /// - The YAML is invalid
    /// - Category names are not lowercase alphanumeric
    /// - Rules have no valid criteria
    pub fn load_from_file(path: &Path) -> Result<Self> {
        let content = fs::read_to_string(path)?;

        // Parse YAML with error sanitization to prevent content leak
        let config: CategoryConfig = serde_yaml::from_str(&content).map_err(|_e| {
            anyhow::anyhow!(
                "invalid config: failed to parse YAML at '{}'",
                path.display()
            )
        })?;

        // Validate that default_category is set
        if config.default_category.is_empty() {
            anyhow::bail!("invalid config: default_category is required and must not be empty");
        }

        // Validate that all categories are lowercase alphanumeric
        if !is_valid_category_name(&config.default_category) {
            anyhow::bail!(
                "invalid config: default_category '{}' is not lowercase alphanumeric",
                config.default_category
            );
        }

        if let Some(rule) = config
            .rules
            .iter()
            .find(|rule| !is_valid_category_name(&rule.category))
        {
            anyhow::bail!(
                "invalid config: category '{}' is not lowercase alphanumeric",
                rule.category
            );
        }

        if let Some(rule) = config
            .rules
            .iter()
            .find(|rule| !Self::has_valid_criteria(&rule.criteria))
        {
            anyhow::bail!(
                "invalid config: rule for category '{}' has no criteria (all are None or empty)",
                rule.category
            );
        }

        let config = CategoryConfig {
            default_category: config.default_category,
            rules: config
                .rules
                .into_iter()
                .map(|rule| CategoryRule {
                    category: rule.category,
                    criteria: MatchCriteria {
                        filename: rule
                            .criteria
                            .filename
                            .map(|v| v.into_iter().map(|s| s.to_lowercase()).collect()),
                        content: rule
                            .criteria
                            .content
                            .map(|v| v.into_iter().map(|s| s.to_lowercase()).collect()),
                        path: rule.criteria.path,
                    },
                })
                .collect(),
        };

        Ok(config)
    }

    /// Check if criteria has at least one non-empty, non-None criterion
    pub(crate) fn has_valid_criteria(criteria: &MatchCriteria) -> bool {
        let has_filename = criteria
            .filename
            .as_ref()
            .is_some_and(|v| !v.is_empty() && v.iter().any(|s| !s.trim().is_empty()));

        let has_content = criteria
            .content
            .as_ref()
            .is_some_and(|v| !v.is_empty() && v.iter().any(|s| !s.trim().is_empty()));

        let has_path = criteria
            .path
            .as_ref()
            .is_some_and(|v| !v.is_empty() && v.iter().any(|s| !s.trim().is_empty()));

        has_filename || has_content || has_path
    }

    /// Detect category for a document using these rules
    #[must_use]
    pub fn detect_category(&self, filename: &str, content: &str, path: &str) -> String {
        self.rules
            .iter()
            .find_map(|rule| {
                self.matches_rule(filename, content, path, &rule.criteria)
                    .then(|| rule.category.clone())
            })
            .map_or_else(|| self.default_category.clone(), std::convert::identity)
    }

    /// Check if a document matches a rule's criteria
    #[allow(clippy::unused_self)]
    fn matches_rule(
        &self,
        filename: &str,
        content: &str,
        path: &str,
        criteria: &MatchCriteria,
    ) -> bool {
        if let Some(patterns) = &criteria.filename {
            let fname_lower = Path::new(filename)
                .file_stem()
                .filter(|s| !s.is_empty())
                .map(|s| s.to_string_lossy().to_lowercase())
                .map_or_else(String::new, |s| s.to_lowercase());

            if patterns.iter().any(|p| fname_lower.contains(p)) {
                return true;
            }
        }

        if let Some(patterns) = &criteria.content {
            let content_lower = content.to_lowercase();
            if patterns.iter().any(|p| content_lower.contains(p)) {
                return true;
            }
        }

        if let Some(patterns) = &criteria.path {
            if patterns.iter().any(|p| path.contains(p)) {
                return true;
            }
        }

        false
    }
}

/// Validate that a category name is lowercase alphanumeric
pub(crate) fn is_valid_category_name(name: &str) -> bool {
    !name.is_empty()
        && name
            .chars()
            .all(|c| c.is_ascii_lowercase() || c.is_ascii_digit() || c == '_' || c == '-')
}

//! Data types for llms.txt files.

use serde::{Deserialize, Serialize};
use std::collections::HashMap;

/// YAML frontmatter metadata.
// TODO(L6): serde_yaml is deprecated. Migrate to serde_yml or another YAML library.
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct Frontmatter {
    pub version: Option<String>,
    pub project: Option<String>,
    pub project_version: Option<String>,
    pub updated: Option<String>,
    pub documents: Option<usize>,
    pub index: Option<String>,
    #[serde(flatten)]
    pub extra: HashMap<String, serde_yaml::Value>,
}

/// A section in llms.txt (## heading).
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Section {
    pub title: String,
    pub content: String,
    pub links: Vec<Link>,
}

/// A link within a section.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Link {
    pub text: String,
    pub url: String,
    pub description: Option<String>,
}

/// Parsed llms.txt file.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LlmsTxt {
    pub frontmatter: Option<Frontmatter>,
    pub project_name: String,
    pub description: Option<String>,
    pub sections: Vec<Section>,
}

impl LlmsTxt {
    /// Get section by title.
    #[must_use]
    pub fn get_section(&self, title: &str) -> Option<&Section> {
        self.sections.iter().find(|s| s.title == title)
    }

    /// Check if required sections exist.
    #[must_use]
    pub fn has_required_sections(&self) -> bool {
        ["Getting Started", "Core Concepts", "API Reference"]
            .iter()
            .all(|&r| self.get_section(r).is_some())
    }

    /// Get index reference from frontmatter or Machine-Readable Index section.
    #[must_use]
    pub fn get_index_reference(&self) -> Option<String> {
        self.frontmatter
            .as_ref()
            .and_then(|fm| fm.index.as_ref())
            .cloned()
            .or_else(|| {
                self.get_section("Machine-Readable Index")
                    .and_then(|section| {
                        section
                            .links
                            .iter()
                            .find(|link| link.url.contains("INDEX.json"))
                            .map(|link| link.url.clone())
                    })
            })
    }
}

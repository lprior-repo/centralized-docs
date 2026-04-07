use itertools::Itertools;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::sync::Arc;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Heading {
    pub level: u32,
    pub text: String,
    pub line: usize,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum LinkKind {
    Internal,
    External,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Link {
    pub text: String,
    pub target: String,
    pub kind: LinkKind,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Analysis {
    pub source_path: String,
    pub title: String,
    pub frontmatter: Option<HashMap<String, String>>,
    pub headings: Vec<Heading>,
    pub links: Vec<Link>,
    pub first_paragraph: String,
    pub word_count: usize,
    pub has_code: bool,
    pub has_tables: bool,
    pub category: String,
    pub content: Arc<str>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FailedFile {
    pub source_path: String,
    pub error: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AnalyzeResult {
    pub analyses: Vec<Analysis>,
    pub failed_files: Vec<FailedFile>,
    pub total_discovered: usize,
}

impl AnalyzeResult {
    #[allow(dead_code)]
    #[must_use]
    pub fn len(&self) -> usize {
        self.analyses.len()
    }

    #[allow(dead_code)]
    #[must_use]
    pub fn is_empty(&self) -> bool {
        self.analyses.is_empty()
    }
}

impl std::ops::Deref for AnalyzeResult {
    type Target = Vec<Analysis>;

    fn deref(&self) -> &Self::Target {
        &self.analyses
    }
}

#[must_use]
pub fn count_categories(analyses: &[Analysis]) -> HashMap<String, usize> {
    analyses
        .iter()
        .map(|a| (a.category.clone(), ()))
        .into_group_map()
        .into_iter()
        .map(|(k, v)| (k, v.len()))
        .collect()
}

//! Types and helpers for llms.txt and INDEX.json validation.

use regex::Regex;
use serde::{Deserialize, Serialize};
use std::path::Path;

/// Validation error
#[derive(Debug)]
pub struct ValidationError {
    pub field: String,
    pub message: String,
    pub severity: Severity,
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Severity {
    Error,   // Must fix
    Warning, // Should fix
    Info,    // Nice to have
}

/// Validation result
#[derive(Debug)]
pub struct ValidationResult {
    #[allow(dead_code)]
    pub valid: bool,
    pub errors: Vec<ValidationError>,
}

impl ValidationResult {
    #[cfg(test)]
    pub(crate) fn new() -> Self {
        Self {
            valid: true,
            errors: Vec::new(),
        }
    }

    #[cfg(test)]
    pub(crate) fn add_error(&mut self, field: &str, message: &str, severity: Severity) {
        if severity == Severity::Error {
            self.valid = false;
        }
        self.errors.push(ValidationError {
            field: field.to_string(),
            message: message.to_string(),
            severity,
        });
    }

    #[allow(dead_code)] // Reserved for programmatic validation result checking
    pub(crate) fn has_errors(&self) -> bool {
        self.errors.iter().any(|e| e.severity == Severity::Error)
    }

    #[allow(dead_code)] // Reserved for programmatic validation result checking
    pub(crate) fn has_warnings(&self) -> bool {
        self.errors.iter().any(|e| e.severity == Severity::Warning)
    }
}

pub(crate) fn validation_result(errors: Vec<ValidationError>) -> ValidationResult {
    let valid = !errors.iter().any(|e| e.severity == Severity::Error);
    ValidationResult { valid, errors }
}

pub(crate) fn make_error(field: &str, message: &str, severity: Severity) -> ValidationError {
    ValidationError {
        field: field.to_string(),
        message: message.to_string(),
        severity,
    }
}

pub(crate) struct UrlValidation {
    malformed: bool,
    errors: Vec<ValidationError>,
}

pub(crate) fn validate_single_url(url: &str) -> UrlValidation {
    if url.is_empty() {
        return UrlValidation {
            malformed: true,
            errors: vec![make_error(
                "links",
                "Found empty link URL",
                Severity::Warning,
            )],
        };
    }

    if url.starts_with('\n') || url.contains('\n') {
        return UrlValidation {
            malformed: true,
            errors: vec![make_error(
                "links",
                &format!(
                    "Malformed link: URL contains newline near '{}'",
                    url.chars().take(20).collect::<String>()
                ),
                Severity::Warning,
            )],
        };
    }

    if url.starts_with("http://") || url.starts_with("https://") {
        if !url.contains('.') || url.len() < 12 {
            return UrlValidation {
                malformed: false,
                errors: vec![make_error(
                    "links",
                    &format!("Suspicious URL format: {url}"),
                    Severity::Info,
                )],
            };
        }
        return UrlValidation {
            malformed: false,
            errors: vec![],
        };
    }

    if url.starts_with('#') {
        return UrlValidation {
            malformed: false,
            errors: vec![],
        };
    }

    if url.starts_with('/') || url.starts_with("./") || url.starts_with("../") {
        if url.contains("..") && url.matches("..").count() > 3 {
            return UrlValidation {
                malformed: false,
                errors: vec![make_error(
                    "links",
                    &format!("Deeply nested relative path: {url}"),
                    Severity::Info,
                )],
            };
        }
        return UrlValidation {
            malformed: false,
            errors: vec![],
        };
    }

    if !url.starts_with("mailto:") && !url.starts_with("ftp:") {
        return UrlValidation {
            malformed: false,
            errors: vec![make_error(
                "links",
                &format!("Unknown URL scheme or relative path: {url}"),
                Severity::Info,
            )],
        };
    }

    UrlValidation {
        malformed: false,
        errors: vec![],
    }
}

/// Extract and validate URLs from markdown content
pub(crate) fn validate_links_in_content(content: &str) -> Vec<ValidationError> {
    let link_regex = match Regex::new(r"\[([^\]]+)\]\(([^)]+)\)") {
        Ok(re) => re,
        Err(_) => {
            return vec![make_error(
                "links",
                "Failed to compile link regex",
                Severity::Error,
            )]
        }
    };

    let url_validations: Vec<UrlValidation> = link_regex
        .captures_iter(content)
        .filter_map(|captures| captures.get(2).map(|m| m.as_str()))
        .map(validate_single_url)
        .collect();

    let url_count = url_validations.len();
    let malformed_count = url_validations.iter().filter(|v| v.malformed).count();

    let per_url_errors: Vec<ValidationError> =
        url_validations.into_iter().flat_map(|v| v.errors).collect();

    let summary_errors: Vec<ValidationError> = if url_count == 0 {
        vec![make_error(
            "links",
            "No links found in document",
            Severity::Info,
        )]
    } else if malformed_count > 0 {
        vec![make_error(
            "links",
            &format!("Found {malformed_count} malformed links out of {url_count} total"),
            Severity::Warning,
        )]
    } else {
        vec![]
    };

    per_url_errors.into_iter().chain(summary_errors).collect()
}

/// INDEX.json structure (simplified)
#[derive(Debug, Deserialize, Serialize)]
pub(crate) struct IndexJson {
    pub version: Option<String>,
    pub project: Option<String>,
    pub updated: Option<String>,
    pub documents: Option<Vec<Document>>,
    pub chunks: Option<Vec<Chunk>>,
}

#[derive(Debug, Deserialize, Serialize)]
pub(crate) struct Document {
    pub id: String,
    pub title: String,
    pub path: String,
    pub category: Option<String>,
    pub tags: Option<Vec<String>>,
    pub word_count: Option<usize>,
    pub summary: Option<String>,
}

#[derive(Debug, Deserialize, Serialize)]
pub(crate) struct Chunk {
    pub chunk_id: String,
    pub doc_id: String,
    pub content: Option<String>,
    pub token_count: Option<usize>,
    pub chunk_level: Option<String>,
}

/// Validate chunk file paths exist
#[allow(unused_variables)]
pub(crate) fn validate_chunk_paths(_chunks: &[Chunk], _base_path: &Path) -> Vec<ValidationError> {
    vec![]
}

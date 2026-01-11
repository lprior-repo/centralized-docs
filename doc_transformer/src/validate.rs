use crate::analyze::Analysis;
use anyhow::Result;
use regex::Regex;
use serde::{Deserialize, Serialize};
use std::collections::HashSet;
use std::fs;
use std::path::{Path, PathBuf};
use std::sync::OnceLock;
use urlencoding;

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum Severity {
    Error,
    Warning,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ValidationIssue {
    pub file: String,
    pub rule_id: String,
    pub severity: Severity,
    pub message: String,
    pub line: Option<usize>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ValidationResult {
    pub files_checked: usize,
    pub files_passed: usize,
    pub total_errors: usize,
    pub total_warnings: usize,
    pub issues: Vec<ValidationIssue>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BrokenLink {
    pub source_file: String,
    pub line_number: usize,
    pub link_text: String,
    pub target: String,
    pub reason: BrokenLinkReason,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum BrokenLinkReason {
    FileNotFound,
    EmptyTarget,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct LinkValidationResult {
    pub total_links: usize,
    pub internal_links: usize,
    pub broken_links: Vec<BrokenLink>,
}

pub fn validate_all(output_dir: &Path) -> Result<ValidationResult> {
    let docs_dir = output_dir.join("docs");

    let mut files_checked = 0;
    let mut files_passed = 0;
    let mut total_errors = 0;
    let mut total_warnings = 0;
    let mut all_issues = Vec::new();

    if !docs_dir.exists() {
        return Ok(ValidationResult {
            files_checked: 0,
            files_passed: 0,
            total_errors: 0,
            total_warnings: 0,
            issues: Vec::new(),
        });
    }

    for entry in fs::read_dir(docs_dir)? {
        let entry = entry?;
        let path = entry.path();

        if path.extension().is_some_and(|ext| ext == "md") {
            files_checked += 1;
            if let Ok(content) = fs::read_to_string(&path) {
                let file_name = path.file_name()
                    .and_then(|n| n.to_str())
                    .unwrap_or("unknown")
                    .to_string();

                let issues = validate_file_detailed(&file_name, &content);

                let errors = issues.iter().filter(|i| i.severity == Severity::Error).count();
                let warnings = issues.iter().filter(|i| i.severity == Severity::Warning).count();

                if errors == 0 {
                    files_passed += 1;
                }

                total_errors += errors;
                total_warnings += warnings;
                all_issues.extend(issues);
            }
        }
    }

    Ok(ValidationResult {
        files_checked,
        files_passed,
        total_errors,
        total_warnings,
        issues: all_issues,
    })
}

static H1_REGEX: OnceLock<Regex> = OnceLock::new();
static TAGS_REGEX: OnceLock<Regex> = OnceLock::new();

fn get_h1_regex() -> &'static Regex {
    H1_REGEX.get_or_init(|| {
        Regex::new(r"^# [^#]").expect("Failed to compile H1 regex pattern")
    })
}

fn get_tags_regex() -> &'static Regex {
    TAGS_REGEX.get_or_init(|| {
        Regex::new(r"tags:\s*\[[^\]]{10,}\]").expect("Failed to compile tags regex pattern")
    })
}

fn validate_file_detailed(file_name: &str, content: &str) -> Vec<ValidationIssue> {
    let mut issues = Vec::new();

    // V001: single_h1
    let h1_count = get_h1_regex()
        .find_iter(content)
        .count();
    if h1_count != 1 {
        issues.push(ValidationIssue {
            file: file_name.to_string(),
            rule_id: "V001".to_string(),
            severity: Severity::Error,
            message: format!("Expected exactly 1 H1 heading, found {}", h1_count),
            line: None,
        });
    }

    // V002: frontmatter_exists
    if !content.starts_with("---") {
        issues.push(ValidationIssue {
            file: file_name.to_string(),
            rule_id: "V002".to_string(),
            severity: Severity::Error,
            message: "Missing frontmatter (should start with ---)".to_string(),
            line: None,
        });
    }

    // V003: required_fields
    let required = ["id:", "title:", "category:", "tags:"];
    for field in &required {
        if !content[..std::cmp::min(500, content.len())].contains(field) {
            issues.push(ValidationIssue {
                file: file_name.to_string(),
                rule_id: "V003".to_string(),
                severity: Severity::Error,
                message: format!("Missing required frontmatter field: {}", field),
                line: None,
            });
        }
    }

    // V006: min_tags
    if !get_tags_regex().is_match(content) {
        issues.push(ValidationIssue {
            file: file_name.to_string(),
            rule_id: "V006".to_string(),
            severity: Severity::Warning,
            message: "Document should have at least 2 meaningful tags".to_string(),
            line: None,
        });
    }

    // V007: has_context
    if !content.contains("> **Context**:") {
        issues.push(ValidationIssue {
            file: file_name.to_string(),
            rule_id: "V007".to_string(),
            severity: Severity::Warning,
            message: "Missing context block".to_string(),
            line: None,
        });
    }

    // V008: has_see_also
    if !content.contains("## See Also") {
        issues.push(ValidationIssue {
            file: file_name.to_string(),
            rule_id: "V008".to_string(),
            severity: Severity::Warning,
            message: "Missing 'See Also' section".to_string(),
            line: None,
        });
    }

    issues
}

fn validate_file(content: &str) -> (usize, usize) {
    let issues = validate_file_detailed("", content);
    let errors = issues.iter().filter(|i| i.severity == Severity::Error).count();
    let warnings = issues.iter().filter(|i| i.severity == Severity::Warning).count();
    (errors, warnings)
}

fn validate_file_detailed(filename: &str, content: &str) -> Vec<ValidationIssue> {
    let mut issues = Vec::new();

    // V001: single_h1
    let h1_count = get_h1_regex()
        .find_iter(content)
        .count();
    if h1_count != 1 {
        issues.push(ValidationIssue {
            file: filename.to_string(),
            rule_id: "V001".to_string(),
            severity: Severity::Error,
            message: format!("Document has {} H1 headings (expected 1)", h1_count),
            line: None,
        });
    }

    // V002: frontmatter_exists
    if !content.starts_with("---") {
        issues.push(ValidationIssue {
            file: filename.to_string(),
            rule_id: "V002".to_string(),
            severity: Severity::Error,
            message: "Document does not start with frontmatter".to_string(),
            line: Some(1),
        });
    }

    // V003-V005: required_fields
    let frontmatter_section = &content[..std::cmp::min(500, content.len())];

    if !frontmatter_section.contains("id:") {
        issues.push(ValidationIssue {
            file: filename.to_string(),
            rule_id: "V003".to_string(),
            severity: Severity::Error,
            message: "Missing required frontmatter field 'id'".to_string(),
            line: None,
        });
    }

    if !frontmatter_section.contains("title:") {
        issues.push(ValidationIssue {
            file: filename.to_string(),
            rule_id: "V004".to_string(),
            severity: Severity::Error,
            message: "Missing required frontmatter field 'title'".to_string(),
            line: None,
        });
    }

    if !frontmatter_section.contains("category:") {
        issues.push(ValidationIssue {
            file: filename.to_string(),
            rule_id: "V005".to_string(),
            severity: Severity::Error,
            message: "Missing required frontmatter field 'category'".to_string(),
            line: None,
        });
    }

    if !frontmatter_section.contains("tags:") {
        issues.push(ValidationIssue {
            file: filename.to_string(),
            rule_id: "V005".to_string(),
            severity: Severity::Error,
            message: "Missing required frontmatter field 'tags'".to_string(),
            line: None,
        });
    }

    // V006: min_tags
    if !get_tags_regex().is_match(content) {
        issues.push(ValidationIssue {
            file: filename.to_string(),
            rule_id: "V006".to_string(),
            severity: Severity::Warning,
            message: "Tags field too short (< 10 chars)".to_string(),
            line: None,
        });
    }

    // V007: has_context
    if !content.contains("> **Context**:") {
        issues.push(ValidationIssue {
            file: filename.to_string(),
            rule_id: "V007".to_string(),
            severity: Severity::Warning,
            message: "Missing context block".to_string(),
            line: None,
        });
    }

    // V008: has_see_also
    if !content.contains("## See Also") {
        issues.push(ValidationIssue {
            file: filename.to_string(),
            rule_id: "V008".to_string(),
            severity: Severity::Warning,
            message: "Missing 'See Also' section".to_string(),
            line: None,
        });
    }

    issues
}

/// Validate internal links in analyzed documents
/// Returns LinkValidationResult with broken link details
pub fn validate_links(
    analyses: &[Analysis],
    source_dir: &Path,
) -> Result<LinkValidationResult> {
    let mut total_links = 0;
    let mut internal_links = 0;
    let mut broken_links = Vec::new();

    // Build a set of all available files for fast lookup
    let mut available_files = HashSet::new();
    for analysis in analyses {
        let source_path = PathBuf::from(&analysis.source_path);
        available_files.insert(source_path);
    }

    // Check each analysis for broken links
    for analysis in analyses {
        let source_path = PathBuf::from(&analysis.source_path);
        let source_dir_for_file = source_path.parent().unwrap_or(Path::new(""));

        for link in &analysis.links {
            total_links += 1;

            if !link.is_internal {
                continue;
            }

            internal_links += 1;

            // Check for empty target
            if link.target.trim().is_empty() {
                broken_links.push(BrokenLink {
                    source_file: analysis.source_path.clone(),
                    line_number: 0, // We don't track line numbers in Link struct yet
                    link_text: link.text.clone(),
                    target: link.target.clone(),
                    reason: BrokenLinkReason::EmptyTarget,
                });
                continue;
            }

            // Strip anchor and query from target
            let target_path = strip_anchor_and_query(&link.target);

            // Decode URL encoding
            let decoded_target = urlencoding::decode(&target_path)
                .unwrap_or_else(|_| std::borrow::Cow::Borrowed(&target_path));

            // Resolve relative path
            let resolved = resolve_link_path(source_dir_for_file, &decoded_target);

            // Check if file exists in available files
            if !available_files.contains(&resolved) {
                // Also check with source_dir prefix (handles both relative and absolute)
                let with_source_dir = source_dir.join(&resolved);
                let normalized = normalize_path(&with_source_dir);

                let mut found = false;
                for available in &available_files {
                    let available_full = source_dir.join(available);
                    let available_normalized = normalize_path(&available_full);
                    if normalized == available_normalized {
                        found = true;
                        break;
                    }
                }

                if !found {
                    broken_links.push(BrokenLink {
                        source_file: analysis.source_path.clone(),
                        line_number: 0,
                        link_text: link.text.clone(),
                        target: link.target.clone(),
                        reason: BrokenLinkReason::FileNotFound,
                    });
                }
            }
        }
    }

    Ok(LinkValidationResult {
        total_links,
        internal_links,
        broken_links,
    })
}

/// Strip anchor (#section) and query (?key=value) from link target
fn strip_anchor_and_query(target: &str) -> String {
    let without_anchor = target.split('#').next().unwrap_or(target);
    let without_query = without_anchor.split('?').next().unwrap_or(without_anchor);
    without_query.to_string()
}

/// Resolve a relative link path from a source directory
fn resolve_link_path(source_dir: &Path, target: &str) -> PathBuf {
    let target_path = PathBuf::from(target);

    // If target starts with ./ or ../, it's relative to source_dir
    if target.starts_with("./") || target.starts_with("../") {
        let mut result = source_dir.to_path_buf();
        for component in target_path.components() {
            match component {
                std::path::Component::CurDir => {}
                std::path::Component::ParentDir => {
                    result.pop();
                }
                std::path::Component::Normal(part) => {
                    result.push(part);
                }
                _ => {}
            }
        }
        result
    } else {
        // Absolute or simple filename - resolve from source_dir
        source_dir.join(target_path)
    }
}

/// Normalize path by resolving . and .. components
fn normalize_path(path: &Path) -> PathBuf {
    let mut normalized = PathBuf::new();
    for component in path.components() {
        match component {
            std::path::Component::CurDir => {}
            std::path::Component::ParentDir => {
                normalized.pop();
            }
            _ => {
                normalized.push(component);
            }
        }
    }
    normalized
}

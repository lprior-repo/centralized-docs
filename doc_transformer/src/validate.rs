use anyhow::Result;
use regex::Regex;
use serde::{Deserialize, Serialize};
use std::fs;
use std::path::Path;

#[derive(Debug, Serialize, Deserialize)]
pub struct ValidationResult {
    pub files_checked: usize,
    pub files_passed: usize,
    pub total_errors: usize,
    pub total_warnings: usize,
}

pub fn validate_all(output_dir: &Path) -> Result<ValidationResult> {
    let docs_dir = output_dir.join("docs");

    let mut files_checked = 0;
    let mut files_passed = 0;
    let mut total_errors = 0;
    let mut total_warnings = 0;

    if !docs_dir.exists() {
        return Ok(ValidationResult {
            files_checked: 0,
            files_passed: 0,
            total_errors: 0,
            total_warnings: 0,
        });
    }

    for entry in fs::read_dir(docs_dir)? {
        let entry = entry?;
        let path = entry.path();

        if path.extension().is_some_and(|ext| ext == "md") {
            files_checked += 1;
            if let Ok(content) = fs::read_to_string(&path) {
                let (errors, warnings) = validate_file(&content);

                if errors == 0 {
                    files_passed += 1;
                }

                total_errors += errors;
                total_warnings += warnings;
            }
        }
    }

    Ok(ValidationResult {
        files_checked,
        files_passed,
        total_errors,
        total_warnings,
    })
}

fn validate_file(content: &str) -> (usize, usize) {
    let mut errors = 0;
    let mut warnings = 0;

    // V001: single_h1
    let h1_count = Regex::new(r"^# [^#]")
        .unwrap()
        .find_iter(content)
        .count();
    if h1_count != 1 {
        errors += 1;
    }

    // V002: frontmatter_exists
    if !content.starts_with("---") {
        errors += 1;
    }

    // V003: required_fields
    let required = ["id:", "title:", "category:", "tags:"];
    for field in &required {
        if !content[..std::cmp::min(500, content.len())].contains(field) {
            errors += 1;
        }
    }

    // V006: min_tags
    if !Regex::new(r"tags:\s*\[[^\]]{10,}\]")
        .unwrap()
        .is_match(content)
    {
        warnings += 1;
    }

    // V007: has_context
    if !content.contains("> **Context**:") {
        warnings += 1;
    }

    // V008: has_see_also
    if !content.contains("## See Also") {
        warnings += 1;
    }

    (errors, warnings)
}

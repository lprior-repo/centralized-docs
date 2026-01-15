#![deny(clippy::unwrap_used)]
#![deny(clippy::expect_used)]
#![deny(clippy::panic)]

//! llms.txt Validator CLI
//!
//! Validates llms.txt files and INDEX.json against RFC specification.
//!
//! Usage:
//!   llms-txt-validator <path>           # Validate llms.txt file
//!   llms-txt-validator --index <path>   # Validate INDEX.json file
//!   llms-txt-validator --url <url>      # Validate remote llms.txt

use anyhow::{Context, Result};
use serde::{Deserialize, Serialize};
use std::collections::HashSet;
use std::fs;
use std::path::{Path, PathBuf};

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
    pub valid: bool,
    pub errors: Vec<ValidationError>,
}

impl ValidationResult {
    fn new() -> Self {
        Self {
            valid: true,
            errors: Vec::new(),
        }
    }

    fn add_error(&mut self, field: &str, message: &str, severity: Severity) {
        if severity == Severity::Error {
            self.valid = false;
        }
        self.errors.push(ValidationError {
            field: field.to_string(),
            message: message.to_string(),
            severity,
        });
    }

    fn has_errors(&self) -> bool {
        self.errors.iter().any(|e| e.severity == Severity::Error)
    }

    fn has_warnings(&self) -> bool {
        self.errors.iter().any(|e| e.severity == Severity::Warning)
    }
}

/// INDEX.json structure (simplified)
#[derive(Debug, Deserialize, Serialize)]
struct IndexJson {
    version: Option<String>,
    project: Option<String>,
    updated: Option<String>,
    documents: Option<Vec<Document>>,
    chunks: Option<Vec<Chunk>>,
}

#[derive(Debug, Deserialize, Serialize)]
struct Document {
    id: String,
    title: String,
    path: String,
    category: Option<String>,
    tags: Option<Vec<String>>,
    word_count: Option<usize>,
    summary: Option<String>,
}

#[derive(Debug, Deserialize, Serialize)]
struct Chunk {
    chunk_id: String,
    doc_id: String,
    content: Option<String>,
    token_count: Option<usize>,
    chunk_level: Option<String>,
}

/// Validate llms.txt file
fn validate_llms_txt(path: &Path) -> Result<ValidationResult> {
    let mut result = ValidationResult::new();

    // Check file exists
    if !path.exists() {
        result.add_error("file", "llms.txt does not exist", Severity::Error);
        return Ok(result);
    }

    // Read content
    let content = fs::read_to_string(path)
        .with_context(|| format!("Failed to read {}", path.display()))?;

    // Check file is not empty
    if content.trim().is_empty() {
        result.add_error("content", "File is empty", Severity::Error);
        return Ok(result);
    }

    // Check for required sections
    let required_sections = vec!["Getting Started", "Core Concepts", "API Reference"];
    for section in required_sections {
        if !content.contains(&format!("## {}", section)) {
            result.add_error(
                "sections",
                &format!("Missing required section: {}", section),
                Severity::Warning,
            );
        }
    }

    // Check for INDEX.json reference
    if !content.contains("INDEX.json") {
        result.add_error(
            "index_reference",
            "No reference to INDEX.json found",
            Severity::Warning,
        );
    }

    // Check structure (basic markdown validation)
    let lines: Vec<&str> = content.lines().collect();
    let mut has_h1 = false;
    let mut has_h2 = false;

    for line in &lines {
        if line.starts_with("# ") {
            has_h1 = true;
        }
        if line.starts_with("## ") {
            has_h2 = true;
        }
    }

    if !has_h1 {
        result.add_error("structure", "No H1 heading found", Severity::Warning);
    }

    if !has_h2 {
        result.add_error("structure", "No H2 headings found", Severity::Error);
    }

    // Check length (should be substantial)
    let word_count = content.split_whitespace().count();
    if word_count < 100 {
        result.add_error(
            "length",
            &format!("File seems too short ({} words)", word_count),
            Severity::Warning,
        );
    }

    Ok(result)
}

/// Validate INDEX.json file
fn validate_index_json(path: &Path) -> Result<ValidationResult> {
    let mut result = ValidationResult::new();

    // Check file exists
    if !path.exists() {
        result.add_error("file", "INDEX.json does not exist", Severity::Error);
        return Ok(result);
    }

    // Read and parse JSON
    let content = fs::read_to_string(path)
        .with_context(|| format!("Failed to read {}", path.display()))?;

    let index: IndexJson = match serde_json::from_str(&content) {
        Ok(idx) => idx,
        Err(e) => {
            result.add_error("json", &format!("Invalid JSON: {}", e), Severity::Error);
            return Ok(result);
        }
    };

    // Validate required fields
    if index.version.is_none() {
        result.add_error("version", "Missing required field: version", Severity::Error);
    }

    if index.project.is_none() {
        result.add_error("project", "Missing required field: project", Severity::Error);
    }

    if index.updated.is_none() {
        result.add_error("updated", "Missing required field: updated", Severity::Warning);
    }

    // Validate documents
    if let Some(docs) = &index.documents {
        if docs.is_empty() {
            result.add_error("documents", "Documents array is empty", Severity::Error);
        }

        // Check for duplicate document IDs
        let mut seen_ids = HashSet::new();
        for doc in docs {
            if !seen_ids.insert(&doc.id) {
                result.add_error(
                    "documents",
                    &format!("Duplicate document ID: {}", doc.id),
                    Severity::Error,
                );
            }

            // Validate document fields
            if doc.title.is_empty() {
                result.add_error(
                    "documents",
                    &format!("Document {} has empty title", doc.id),
                    Severity::Warning,
                );
            }

            if doc.path.is_empty() {
                result.add_error(
                    "documents",
                    &format!("Document {} has empty path", doc.id),
                    Severity::Error,
                );
            }
        }
    } else {
        result.add_error("documents", "Missing required field: documents", Severity::Error);
    }

    // Validate chunks
    if let Some(chunks) = &index.chunks {
        let mut seen_chunk_ids = HashSet::new();
        let doc_ids: HashSet<String> = index
            .documents
            .as_ref()
            .map(|docs| docs.iter().map(|d| d.id.clone()).collect())
            .unwrap_or_default();

        for chunk in chunks {
            // Check for duplicate chunk IDs
            if !seen_chunk_ids.insert(&chunk.chunk_id) {
                result.add_error(
                    "chunks",
                    &format!("Duplicate chunk ID: {}", chunk.chunk_id),
                    Severity::Error,
                );
            }

            // Validate doc_id references
            if !doc_ids.contains(&chunk.doc_id) {
                result.add_error(
                    "chunks",
                    &format!("Chunk {} references non-existent document: {}", chunk.chunk_id, chunk.doc_id),
                    Severity::Error,
                );
            }

            // Validate chunk_level values
            if let Some(level) = &chunk.chunk_level {
                if !["summary", "standard", "detailed"].contains(&level.as_str()) {
                    result.add_error(
                        "chunks",
                        &format!("Invalid chunk_level: {}", level),
                        Severity::Error,
                    );
                }
            }
        }

        if chunks.is_empty() {
            result.add_error("chunks", "Chunks array is empty", Severity::Warning);
        }
    }

    Ok(result)
}

/// Print validation results
fn print_results(result: &ValidationResult, path: &Path) {
    println!("\nValidating: {}", path.display());
    println!("{}", "=".repeat(60));

    if result.errors.is_empty() {
        println!("✅ No issues found!");
        return;
    }

    let error_count = result.errors.iter().filter(|e| e.severity == Severity::Error).count();
    let warning_count = result.errors.iter().filter(|e| e.severity == Severity::Warning).count();
    let info_count = result.errors.iter().filter(|e| e.severity == Severity::Info).count();

    println!(
        "\n📊 Found {} errors, {} warnings, {} info",
        error_count, warning_count, info_count
    );

    for error in &result.errors {
        let symbol = match error.severity {
            Severity::Error => "❌",
            Severity::Warning => "⚠️ ",
            Severity::Info => "ℹ️ ",
        };
        let severity_str = match error.severity {
            Severity::Error => "ERROR",
            Severity::Warning => "WARN",
            Severity::Info => "INFO",
        };
        println!("\n{} [{}] {}", symbol, severity_str, error.field);
        println!("   {}", error.message);
    }

    println!("\n{}", "=".repeat(60));
    if result.valid {
        println!("✅ Validation passed (with warnings)");
    } else {
        println!("❌ Validation failed");
    }
}

fn print_usage(program: &str) {
    eprintln!("llms-txt-validator v1.0 - Validate llms.txt and INDEX.json files");
    eprintln!();
    eprintln!("Usage:");
    eprintln!("  {} <llms.txt>              # Validate llms.txt file", program);
    eprintln!("  {} --index <INDEX.json>    # Validate INDEX.json file", program);
    eprintln!();
    eprintln!("Options:");
    eprintln!("  -h, --help      Show this help message");
    eprintln!("  -V, --version   Show version information");
}

fn main() -> Result<()> {
    let args: Vec<String> = std::env::args().collect();

    // Handle --help and --version flags first
    if args.iter().any(|a| a == "--help" || a == "-h") {
        print_usage(&args[0]);
        std::process::exit(0);
    }

    if args.iter().any(|a| a == "--version" || a == "-V") {
        eprintln!("llms-txt-validator v1.0");
        std::process::exit(0);
    }

    if args.len() < 2 {
        print_usage(&args[0]);
        std::process::exit(1);
    }

    let (is_index, path) = if args.get(1).map(|s| s.as_str()) == Some("--index") {
        if args.len() < 3 {
            eprintln!("Error: --index requires a path argument");
            std::process::exit(1);
        }
        (true, PathBuf::from(&args[2]))
    } else {
        (false, PathBuf::from(&args[1]))
    };

    let result = if is_index {
        validate_index_json(&path)?
    } else {
        validate_llms_txt(&path)?
    };

    print_results(&result, &path);

    // Exit with error code if validation failed
    if !result.valid {
        std::process::exit(1);
    }

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::io::Write;
    use tempfile::NamedTempFile;

    #[test]
    fn test_valid_llms_txt() {
        let mut file = NamedTempFile::new().unwrap();
        writeln!(
            file,
            "# Project\n\n## Getting Started\n\n## Core Concepts\n\n## API Reference\n\nSee INDEX.json"
        )
        .unwrap();

        let result = validate_llms_txt(file.path()).unwrap();
        assert!(result.valid);
    }

    #[test]
    fn test_empty_llms_txt() {
        let file = NamedTempFile::new().unwrap();
        let result = validate_llms_txt(file.path()).unwrap();
        assert!(!result.valid);
    }

    #[test]
    fn test_valid_index_json() {
        let mut file = NamedTempFile::new().unwrap();
        writeln!(
            file,
            r#"{{"version": "1.0", "project": "test", "documents": [{{"id": "1", "title": "Doc", "path": "doc.md"}}]}}"#
        )
        .unwrap();

        let result = validate_index_json(file.path()).unwrap();
        assert!(result.valid);
    }
}

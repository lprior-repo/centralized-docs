#![allow(clippy::pedantic)]
#![allow(clippy::nursery)]
#![allow(clippy::complexity)]
#![deny(clippy::unwrap_used)]
#![deny(clippy::expect_used)]
#![deny(clippy::panic)]

//! llms.txt Validator CLI
//!
//! Validates llms.txt files and INDEX.json against RFC specification.
//!
//! Usage:
//!   `llms_txt_validator` <path>           # Validate llms.txt file
//!   `llms_txt_validator` --index <path>   # Validate INDEX.json file
//!   `llms_txt_validator` --url <url>      # Validate remote llms.txt

use anyhow::Result;
use std::fs;
use std::path::{Path, PathBuf};

#[path = "llms_txt_validator/types.rs"]
mod types;

#[path = "llms_txt_validator/checks.rs"]
mod checks;

pub(crate) use checks::{validate_index_json, validate_llms_txt};
#[cfg(test)]
pub(crate) use types::{
    make_error, validate_links_in_content, validation_result, Severity, ValidationError,
    ValidationResult,
};
#[cfg(not(test))]
pub(crate) use types::{Severity, ValidationResult};

/// Print validation results
/// Returns the error count for exit code determination
fn print_results(result: &ValidationResult, path: &Path) -> usize {
    println!("\nValidating: {}", path.display());
    println!("{}", "=".repeat(60));

    if result.errors.is_empty() {
        println!("✅ No issues found!");
        return 0;
    }

    let error_count = result
        .errors
        .iter()
        .filter(|e| e.severity == Severity::Error)
        .count();
    let warning_count = result
        .errors
        .iter()
        .filter(|e| e.severity == Severity::Warning)
        .count();
    let info_count = result
        .errors
        .iter()
        .filter(|e| e.severity == Severity::Info)
        .count();

    println!("\n📊 Found {error_count} errors, {warning_count} warnings, {info_count} info");

    result.errors.iter().for_each(|error| {
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
    });

    println!("\n{}", "=".repeat(60));
    if error_count > 0 {
        println!("❌ Validation failed: {error_count} error(s)");
    } else if result.has_warnings() {
        println!("⚠️  Validation passed with warnings");
    } else {
        println!("✅ Validation passed");
    }

    error_count
}

fn print_usage(program: &str) {
    eprintln!(
        "llms_txt_validator v{} - Validate llms.txt and INDEX.json files",
        env!("CARGO_PKG_VERSION")
    );
    eprintln!();
    eprintln!("Usage:");
    eprintln!("  {program} <llms.txt>              # Validate llms.txt file");
    eprintln!("  {program} --index <INDEX.json>    # Validate INDEX.json file");
    eprintln!();
    eprintln!("Options:");
    eprintln!("  -h, --help      Show this help message");
    eprintln!("  -V, --version   Show version information");
}

fn main() -> Result<()> {
    let args: Vec<String> = std::env::args().collect();
    let program = args
        .first()
        .map(std::string::String::as_str)
        .map_or("llms_txt_validator", |s| s);

    if args.iter().any(|a| a == "--help" || a == "-h") {
        print_usage(program);
        std::process::exit(0);
    }

    if args.iter().any(|a| a == "--version" || a == "-V") {
        eprintln!("llms_txt_validator v{}", env!("CARGO_PKG_VERSION"));
        std::process::exit(0);
    }

    if args.len() < 2 {
        print_usage(program);
        std::process::exit(1);
    }

    let (is_index, path) = if args.get(1).map(std::string::String::as_str) == Some("--index") {
        if args.len() < 3 {
            eprintln!("Error: --index requires a path argument");
            std::process::exit(1);
        }
        (true, PathBuf::from(&args[2]))
    } else {
        (false, PathBuf::from(&args[1]))
    };

    if !path.exists() {
        eprintln!("Error: file not found: {}", path.display());
        std::process::exit(1);
    }

    let result = if is_index {
        let content = match fs::read_to_string(&path) {
            Ok(c) => c,
            Err(e) => {
                eprintln!("Error: failed to read file: {e}");
                std::process::exit(1);
            }
        };

        let parse_result: Result<types::IndexJson, _> = serde_json::from_str(&content);
        match parse_result {
            Ok(_) => validate_index_json(&path)?,
            Err(e) => {
                eprintln!("Error: Parse error (invalid JSON): {e}");
                std::process::exit(1);
            }
        }
    } else {
        validate_llms_txt(&path)?
    };

    let error_count = print_results(&result, &path);

    if error_count > 0 {
        std::process::exit(1);
    }

    Ok(())
}

#[cfg(test)]
#[path = "llms_txt_validator/tests.rs"]
mod tests;

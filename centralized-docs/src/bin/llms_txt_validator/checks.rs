//! File-level validation functions for llms.txt and INDEX.json.

use super::types::*;
use anyhow::{Context, Result};
use std::collections::HashSet;
use std::fs;
use std::path::Path;

/// Validate llms.txt file
pub(crate) fn validate_llms_txt(path: &Path) -> Result<ValidationResult> {
    if !path.exists() {
        return Ok(validation_result(vec![make_error(
            "file",
            "llms.txt does not exist",
            Severity::Error,
        )]));
    }

    let content =
        fs::read_to_string(path).with_context(|| format!("Failed to read {}", path.display()))?;

    if content.trim().is_empty() {
        return Ok(validation_result(vec![make_error(
            "content",
            "File is empty",
            Severity::Error,
        )]));
    }

    let required_sections = ["Getting Started", "Core Concepts", "API Reference"];
    let section_errors: Vec<ValidationError> = required_sections
        .iter()
        .filter(|section| !content.contains(&format!("## {section}")))
        .map(|section| {
            make_error(
                "sections",
                &format!("Missing required section: {section}"),
                Severity::Error,
            )
        })
        .collect();

    let index_ref_errors: Vec<ValidationError> = if content.contains("INDEX.json") {
        vec![]
    } else {
        vec![make_error(
            "index_reference",
            "No reference to INDEX.json found",
            Severity::Info,
        )]
    };

    let lines: Vec<&str> = content.lines().collect();
    let has_h1 = lines.iter().any(|line| line.starts_with("# "));
    let has_h2 = lines.iter().any(|line| line.starts_with("## "));

    let structure_errors: Vec<ValidationError> = [
        (!has_h1).then(|| make_error("structure", "No H1 heading found", Severity::Warning)),
        (!has_h2).then(|| make_error("structure", "No H2 headings found", Severity::Error)),
    ]
    .into_iter()
    .flatten()
    .collect();

    let word_count = content.split_whitespace().count();
    let length_errors: Vec<ValidationError> = if word_count < 100 {
        vec![make_error(
            "length",
            &format!("File seems too short ({word_count} words)"),
            Severity::Warning,
        )]
    } else {
        vec![]
    };

    let link_errors = validate_links_in_content(&content);

    let index_file_errors: Vec<ValidationError> = content
        .contains("INDEX.json")
        .then(|| {
            path.parent()
                .map(|p| p.join("INDEX.json"))
                .filter(|index_path| !index_path.exists())
                .map(|_| {
                    make_error(
                        "index_reference",
                        "Referenced INDEX.json file not found in same directory",
                        Severity::Warning,
                    )
                })
                .into_iter()
                .collect::<Vec<_>>()
        })
        .map_or(Vec::new(), |v| v);

    let mut errors = section_errors;
    errors.extend(index_ref_errors);
    errors.extend(structure_errors);
    errors.extend(length_errors);
    errors.extend(link_errors);
    errors.extend(index_file_errors);
    Ok(validation_result(errors))
}

/// Validate INDEX.json file
pub(crate) fn validate_index_json(path: &Path) -> Result<ValidationResult> {
    if !path.exists() {
        return Ok(validation_result(vec![make_error(
            "file",
            "INDEX.json does not exist",
            Severity::Error,
        )]));
    }

    let content =
        fs::read_to_string(path).with_context(|| format!("Failed to read {}", path.display()))?;

    let index: IndexJson = match serde_json::from_str(&content) {
        Ok(idx) => idx,
        Err(e) => {
            return Ok(validation_result(vec![make_error(
                "json",
                &format!("Invalid JSON: {e}"),
                Severity::Error,
            )]));
        }
    };

    let field_errors: Vec<ValidationError> = [
        index.version.is_none().then(|| {
            make_error(
                "version",
                "Missing required field: version",
                Severity::Error,
            )
        }),
        index.project.is_none().then(|| {
            make_error(
                "project",
                "Missing required field: project",
                Severity::Error,
            )
        }),
        index.updated.is_none().then(|| {
            make_error(
                "updated",
                "Missing required field: updated",
                Severity::Warning,
            )
        }),
    ]
    .into_iter()
    .flatten()
    .collect();

    let doc_errors: Vec<ValidationError> = match &index.documents {
        None => vec![make_error(
            "documents",
            "Missing required field: documents",
            Severity::Error,
        )],
        Some(docs) if docs.is_empty() => {
            vec![make_error(
                "documents",
                "Documents array is empty",
                Severity::Error,
            )]
        }
        Some(docs) => {
            let dup_ids: HashSet<&str> = docs
                .iter()
                .map(|doc| doc.id.as_str())
                .filter(|id| docs.iter().filter(|d| d.id.as_str() == *id).count() > 1)
                .collect();

            let dup_errors: Vec<ValidationError> = dup_ids
                .into_iter()
                .map(|id| {
                    make_error(
                        "documents",
                        &format!("Duplicate document ID: {id}"),
                        Severity::Error,
                    )
                })
                .collect();

            let field_val_errors: Vec<ValidationError> = docs
                .iter()
                .flat_map(|doc| {
                    [
                        doc.title.is_empty().then(|| {
                            make_error(
                                "documents",
                                &format!("Document {} has empty title", doc.id),
                                Severity::Warning,
                            )
                        }),
                        doc.path.is_empty().then(|| {
                            make_error(
                                "documents",
                                &format!("Document {} has empty path", doc.id),
                                Severity::Error,
                            )
                        }),
                    ]
                    .into_iter()
                    .flatten()
                })
                .collect();

            dup_errors.into_iter().chain(field_val_errors).collect()
        }
    };

    let chunk_errors: Vec<ValidationError> = match &index.chunks {
        None => vec![],
        Some(chunks) if chunks.is_empty() => {
            vec![make_error(
                "chunks",
                "Chunks array is empty",
                Severity::Warning,
            )]
        }
        Some(chunks) => {
            let doc_ids: HashSet<&str> = index
                .documents
                .as_ref()
                .map(|docs| docs.iter().map(|d| d.id.as_str()).collect())
                .unwrap_or_default();

            let dup_chunk_ids: HashSet<&str> = chunks
                .iter()
                .map(|chunk| chunk.chunk_id.as_str())
                .filter(|id| chunks.iter().filter(|c| c.chunk_id.as_str() == *id).count() > 1)
                .collect();

            let dup_errors: Vec<ValidationError> = dup_chunk_ids
                .into_iter()
                .map(|id| {
                    make_error(
                        "chunks",
                        &format!("Duplicate chunk ID: {id}"),
                        Severity::Error,
                    )
                })
                .collect();

            let ref_errors: Vec<ValidationError> = chunks
                .iter()
                .filter(|chunk| !doc_ids.contains(chunk.doc_id.as_str()))
                .map(|chunk| {
                    make_error(
                        "chunks",
                        &format!(
                            "Chunk {} references non-existent document: {}",
                            chunk.chunk_id, chunk.doc_id
                        ),
                        Severity::Error,
                    )
                })
                .collect();

            let level_errors: Vec<ValidationError> = chunks
                .iter()
                .filter_map(|chunk| {
                    chunk.chunk_level.as_ref().and_then(|level| {
                        (!["summary", "standard", "detailed"].contains(&level.as_str())).then(
                            || {
                                make_error(
                                    "chunks",
                                    &format!("Invalid chunk_level: {level}"),
                                    Severity::Error,
                                )
                            },
                        )
                    })
                })
                .collect();

            let path_errors = path
                .parent()
                .map(|base_dir| validate_chunk_paths(chunks, base_dir))
                .map_or(Vec::new(), |v| v);

            dup_errors
                .into_iter()
                .chain(ref_errors)
                .chain(level_errors)
                .chain(path_errors)
                .collect()
        }
    };

    let mut errors = field_errors;
    errors.extend(doc_errors);
    errors.extend(chunk_errors);
    Ok(validation_result(errors))
}

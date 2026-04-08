//! Change plan formatting as Markdown and JSON, and writing reports to disk.

use std::fmt::Write as _;
use std::path::Path;

use itertools::Itertools;

use super::{ChangeKind, ChangePlan, ChangeSummary, PageChange};

/// Format a change plan as a human-readable report.
#[must_use]
pub fn format_plan_markdown(plan: &ChangePlan) -> String {
    fn changes_body(changes: &[PageChange]) -> String {
        let (added, removed, modified) = changes.iter().fold(
            (String::new(), String::new(), String::new()),
            |(mut a, mut r, mut m), c| {
                let buf = match c.kind {
                    ChangeKind::Added => &mut a,
                    ChangeKind::Removed => &mut r,
                    ChangeKind::Modified => &mut m,
                };
                let prefix = match c.kind {
                    ChangeKind::Added => "+ ",
                    ChangeKind::Removed => "- ",
                    ChangeKind::Modified => "~ ",
                };
                let _ = writeln!(buf, "{prefix}`{}` — {}", c.url, c.title);
                (a, r, m)
            },
        );

        fn section(header: &str, body: &str) -> Option<String> {
            (!body.is_empty()).then(|| format!("### {header}\n\n{body}\n"))
        }

        [
            section("Added", &added),
            section("Removed", &removed),
            section("Modified", &modified),
        ]
        .into_iter()
        .flatten()
        .collect()
    }

    fn summary_lines(s: &ChangeSummary) -> String {
        [
            format!("- **Added:** {} pages", s.added),
            format!("- **Removed:** {} pages", s.removed),
            format!("- **Modified:** {} pages", s.modified),
            format!("- **Unchanged:** {} pages", s.unchanged),
            format!(
                "- **Total:** {} (was {})",
                s.total_current, s.total_previous
            ),
        ]
        .into_iter()
        .join("\n")
    }

    let estimated = 200usize.saturating_add(plan.changes.len().saturating_mul(80));
    let mut out = String::with_capacity(estimated);

    let timestamp_str = plan.timestamp.format("%Y-%m-%d %H:%M:%S UTC").to_string();
    let _ = writeln!(out, "# Documentation Change Plan");
    let _ = writeln!(out);
    let _ = writeln!(out, "**Target:** {}", plan.target_url);
    let _ = writeln!(out, "**Generated:** {timestamp_str}");
    let _ = writeln!(out);

    let _ = writeln!(out, "## Summary");
    let _ = writeln!(out);
    let summary = summary_lines(&plan.summary);
    let _ = writeln!(out, "{summary}");

    let _ = writeln!(out);
    let body = if plan.changes.is_empty() {
        "No changes detected. The documentation is up to date.".to_string()
    } else {
        format!(
            "## Changes\n\n{body}---\n\nRun `ctd apply` to commit these changes.",
            body = changes_body(&plan.changes)
        )
    };

    let _ = writeln!(out, "{body}");
    out
}

/// Format a change plan as JSON.
///
/// # Errors
///
/// Returns an error if serialization fails (should not happen with valid plan).
pub fn format_plan_json(plan: &ChangePlan) -> Result<String, serde_json::Error> {
    serde_json::to_string_pretty(plan)
}

/// Write a change plan to disk as both JSON and Markdown.
///
/// # Errors
///
/// Returns an error if file writing fails.
pub fn write_plan_reports(plan: &ChangePlan, output_dir: &Path) -> Result<(), anyhow::Error> {
    std::fs::create_dir_all(output_dir)?;

    let json_path = output_dir.join("change-plan.json");
    let md_path = output_dir.join("change-plan.md");

    let json_content = format_plan_json(plan)?;
    let md_content = format_plan_markdown(plan);

    std::fs::write(&json_path, json_content)?;
    std::fs::write(&md_path, md_content)?;

    Ok(())
}

use crate::cli::config::IndexConfig;
use crate::cmd::index::run_index;
use crate::sys::utils::extract_last_path_segment;
use anyhow::Result;

pub fn run_ingest_git(
    repo_url: &str,
    output: &std::path::Path,
    branch: Option<&str>,
    _depth: u32,
    project_name: Option<String>,
    filter: Option<String>,
) -> Result<()> {
    let temp_dir = output.join(".git-clone");
    std::fs::create_dir_all(&temp_dir)?;

    let git_dir = temp_dir.join(".git");
    if git_dir.exists() {
        println!("[GIT CLONE] Existing .git directory detected");
        println!("  Checking for markdown files...");
    } else {
        println!("[GIT CLONE] Cloning repository (full depth)...");

        let cloned = if let Some(branch_name) = branch {
            git2::build::RepoBuilder::new()
                .branch(branch_name)
                .clone(repo_url, &temp_dir)
        } else {
            git2::build::RepoBuilder::new().clone(repo_url, &temp_dir)
        };

        cloned.map_err(|e| anyhow::anyhow!("Failed to clone repository: {e}"))?;

        println!("  ✓ Clone successful\n");
    }

    let markdown_files: Vec<_> = walkdir::WalkDir::new(&temp_dir)
        .into_iter()
        .filter_map(Result::ok)
        .filter(|entry| entry.file_type().is_file())
        .filter_map(|entry| {
            entry.path().extension().and_then(|ext| {
                ext.eq_ignore_ascii_case("md")
                    .then(|| entry.path().to_path_buf())
            })
        })
        .collect();

    println!("[DISCOVER] Found {} markdown files\n", markdown_files.len());

    let index_config = IndexConfig {
        generate_llms: true,
        project_name: project_name.unwrap_or_else(|| {
            extract_last_path_segment(repo_url).unwrap_or_else(|| "Documentation".to_string())
        }),
        project_desc: format!("Documentation cloned from {repo_url}"),
        path_filter: filter,
        ..Default::default()
    };

    let index_result = run_index(&temp_dir, output, &index_config);

    if temp_dir.exists() {
        println!("[CLEANUP] Removing temporary clone directory...");
        if let Err(err) = std::fs::remove_dir_all(&temp_dir) {
            eprintln!("Warning: cleanup failed: {err}");
        }
    }

    index_result?;

    println!("\n{}", "=".repeat(70));
    println!("GIT INGEST COMPLETE");
    println!("{}", "=".repeat(70));
    println!("Source:     {repo_url}");
    println!("Output:     {}", output.display());
    println!("Documents:  {}", markdown_files.len());
    println!("Entry:      llms.txt (AI should read this first)");
    println!("{}\n", "=".repeat(70));

    Ok(())
}

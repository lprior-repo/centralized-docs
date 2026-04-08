//! Command dispatch: maps parsed CLI commands to their handler functions.
//!
//! Extracted from `main.rs` for file-length compliance.

use crate::cli::{Commands, McpCommand};
use std::process::ExitCode;

/// Dispatch a parsed CLI command to its handler.
///
/// Returns `(result, search_context)` where `search_context` is `Some((json_mode, query))`
/// for the Search command (used for JSON error formatting).
pub async fn dispatch(command: Commands) -> (anyhow::Result<()>, Option<(bool, String)>) {
    match command {
        Commands::Search {
            query,
            index_dir,
            limit,
            no_color,
            json,
        } => (
            crate::cmd::search::run_search(&query, &index_dir, limit, !no_color, json),
            Some((json, query.clone())),
        ),
        Commands::Mcp { command } => match command {
            McpCommand::Serve { index_dir } => {
                (crate::cmd::mcp::run_mcp_serve(&index_dir).await, None)
            }
        },
        Commands::Scrape {
            url,
            output,
            no_sitemap,
            spider,
            crawl,
        } => {
            let config = crate::cli::config::ScrapeCommandConfig {
                sitemap_strategy: if no_sitemap {
                    crate::scrape::SitemapStrategy::CrawlOnly
                } else {
                    crate::scrape::SitemapStrategy::UseSitemap
                },
                filter: spider.filter,
                delay: spider.delay,
                query: crawl.query,
                threshold: crawl.threshold,
                request_timeout_secs: spider.request_timeout_secs,
                connect_timeout_secs: spider.connect_timeout_secs,
                max_retries: spider.max_retries,
                redirect_policy: spider.redirect_policy,
                max_page_bytes: crawl.max_page_bytes,
                max_total_bytes: crawl.max_total_bytes,
                concurrency_limit: spider.concurrency,
            };
            (
                crate::cmd::scrape::run_scrape(&url, &output, &config).await,
                None,
            )
        }
        Commands::Index {
            source,
            output,
            llms_txt,
            with_agents,
            project_name,
            project_desc,
            category_config,
            max_related_chunks,
            max_chunk_keywords,
            hnsw_m,
            hnsw_ef_construction,
            max_document_bytes,
        } => {
            let config = crate::cli::config::IndexConfig {
                generate_llms: llms_txt,
                generate_agents: with_agents,
                project_name,
                project_desc,
                category_config,
                max_related_chunks,
                max_chunk_keywords,
                hnsw_m,
                hnsw_ef_construction,
                max_document_bytes: max_document_bytes.unwrap_or(10 * 1024 * 1024),
                path_filter: None,
            };
            (
                crate::cmd::index::run_index(&source, &output, &config),
                None,
            )
        }
        Commands::IngestGit {
            repo_url,
            output,
            branch,
            depth,
            project_name,
            filter,
        } => (
            crate::cmd::git::run_ingest_git(
                &repo_url,
                &output,
                branch.as_deref(),
                depth,
                project_name,
                filter,
            ),
            None,
        ),
        Commands::Ingest {
            url,
            output,
            spider,
            crawl,
            project_name,
        } => {
            let config = crate::cli::config::IngestConfig {
                filter: spider.filter,
                delay: spider.delay,
                query: crawl.query,
                threshold: crawl.threshold,
                project_name,
                request_timeout_secs: spider.request_timeout_secs,
                connect_timeout_secs: spider.connect_timeout_secs,
                max_retries: spider.max_retries,
                redirect_policy: spider.redirect_policy,
                max_page_bytes: crawl.max_page_bytes,
                max_total_bytes: crawl.max_total_bytes,
                concurrency_limit: spider.concurrency,
            };
            (
                crate::cmd::ingest::run_ingest(&url, &output, &config).await,
                None,
            )
        }
        Commands::Watch {
            url,
            output,
            cache,
            no_sitemap,
            spider,
            json,
        } => {
            let config = crate::cmd::watch::WatchConfig {
                delay: spider.delay,
                request_timeout_secs: spider.request_timeout_secs,
                connect_timeout_secs: spider.connect_timeout_secs,
                max_retries: spider.max_retries,
                redirect_policy: spider.redirect_policy,
                concurrency: spider.concurrency,
                output_format: if json {
                    crate::cmd::watch::OutputFormat::Json
                } else {
                    crate::cmd::watch::OutputFormat::Markdown
                },
                sitemap_strategy: if no_sitemap {
                    crate::scrape::SitemapStrategy::CrawlOnly
                } else {
                    crate::scrape::SitemapStrategy::UseSitemap
                },
            };
            (
                crate::cmd::watch::run_watch(
                    &url,
                    &output,
                    &cache,
                    spider.filter.as_deref(),
                    &config,
                )
                .await,
                None,
            )
        }
        Commands::Apply {
            url,
            cache,
            scrape_dir,
            yes,
        } => {
            let mode = if yes {
                crate::cmd::watch::ConfirmMode::AutoConfirm
            } else {
                crate::cmd::watch::ConfirmMode::Interactive
            };
            (
                crate::cmd::watch::run_apply(&url, &cache, &scrape_dir, mode).await,
                None,
            )
        }
        Commands::Diff {
            dir_a,
            dir_b,
            output,
            json,
        } => {
            let fmt = if json {
                crate::cmd::watch::OutputFormat::Json
            } else {
                crate::cmd::watch::OutputFormat::Markdown
            };
            (
                crate::cmd::watch::run_diff(&dir_a, &dir_b, output.as_deref(), fmt),
                None,
            )
        }
        Commands::Compact { path } => (crate::cmd::compact::run_compact(&path), None),
    }
}

/// Handle an error from command execution, returning the appropriate exit code.
pub fn handle_error(err: anyhow::Error, search_context: Option<(bool, String)>) -> ExitCode {
    if let Some((json_mode, search_query)) = search_context {
        if json_mode {
            let msg = err.to_string();
            if msg.starts_with(crate::cmd::search::SEARCH_JSON_ALREADY_EMITTED_PREFIX) {
                return ExitCode::SUCCESS;
            }
            let json = serde_json::json!({"status": "error", "query": search_query, "error": msg});
            match serde_json::to_string_pretty(&json) {
                Ok(s) => println!("{s}"),
                Err(_) => eprintln!("Error: {err}"),
            }
            return ExitCode::FAILURE;
        }
    }
    let code = crate::sys::error::map_error_to_exit_code(&err);
    eprintln!("Error: {err}");
    match code {
        0 => ExitCode::SUCCESS,
        2 => ExitCode::from(2),
        _ => ExitCode::FAILURE,
    }
}

/// Handle clap parsing errors, returning the appropriate exit code.
pub fn exit_clap(e: clap::error::Error) -> ExitCode {
    let is_help = matches!(
        e.kind(),
        clap::error::ErrorKind::DisplayHelp | clap::error::ErrorKind::DisplayVersion
    );
    let is_val = matches!(
        e.kind(),
        clap::error::ErrorKind::ValueValidation | clap::error::ErrorKind::InvalidValue
    );
    eprintln!("{e}");
    match if is_help {
        0
    } else if is_val {
        1
    } else {
        2
    } {
        0 => ExitCode::SUCCESS,
        2 => ExitCode::from(2),
        _ => ExitCode::FAILURE,
    }
}

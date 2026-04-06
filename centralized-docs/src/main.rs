//! `ctd` v0.6.1 — AI-Optimized Documentation Indexer
#![allow(clippy::all)]
#![cfg_attr(
    not(test),
    deny(
        clippy::unwrap_used,
        clippy::panic,
        clippy::arithmetic_side_effects,
        clippy::expect_used
    )
)]
#![cfg_attr(
    test,
    allow(
        clippy::unwrap_used,
        clippy::expect_used,
        clippy::panic,
        clippy::arithmetic_side_effects
    )
)]

pub mod analyze;
pub mod assign;
pub mod cache;
pub mod calc;
pub mod chunk;
pub mod chunking_adapter;
pub mod config;
pub mod diff;
pub mod discover;
pub mod errors;
#[cfg(feature = "enhanced")]
pub mod features;
pub mod filter;
pub mod graph;
pub mod highlight;
pub mod index;
pub mod llms;
pub mod math_types;
pub mod persisted;
pub mod scrape;
pub mod search;
pub mod state;
pub mod transform;
pub mod types;
pub mod validate;
pub mod watch;

pub mod cli;
pub mod cmd;
pub mod sys;

use clap::{CommandFactory, FromArgMatches};
use cli::{Cli, Commands, McpCommand};
use std::process::ExitCode;
use tracing::instrument;
#[tokio::main]
#[instrument(skip_all)]
async fn main() -> ExitCode {
    let cmd = Cli::command();
    let cli_matches = match cmd.try_get_matches() {
        Ok(matches) => matches,
        Err(e) => return exit_clap(e),
    };
    let cli_app = match Cli::from_arg_matches(&cli_matches) {
        Ok(c) => c,
        Err(e) => return exit_clap(e),
    };

    let (result, search_context) = match cli_app.command {
        Commands::Search {
            query,
            index_dir,
            limit,
            no_color,
            json,
        } => (
            cmd::search::run_search(&query, &index_dir, limit, !no_color, json),
            Some((json, query.clone())),
        ),
        Commands::Mcp { command } => match command {
            McpCommand::Serve { index_dir } => (cmd::mcp::run_mcp_serve(&index_dir).await, None),
        },
        Commands::Scrape {
            url,
            output,
            no_sitemap,
            filter,
            delay,
            query,
            threshold,
            request_timeout_secs,
            connect_timeout_secs,
            max_retries,
            redirect_policy,
            max_page_bytes,
            max_total_bytes,
            concurrency,
        } => {
            let config = cli::config::ScrapeCommandConfig {
                sitemap_strategy: if no_sitemap {
                    scrape::SitemapStrategy::CrawlOnly
                } else {
                    scrape::SitemapStrategy::UseSitemap
                },
                filter,
                delay,
                query,
                threshold,
                request_timeout_secs,
                connect_timeout_secs,
                max_retries,
                redirect_policy,
                max_page_bytes,
                max_total_bytes,
                concurrency_limit: concurrency,
            };
            (cmd::scrape::run_scrape(&url, &output, &config).await, None)
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
            let config = cli::config::IndexConfig {
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
            (cmd::index::run_index(&source, &output, &config), None)
        }
        Commands::IngestGit {
            repo_url,
            output,
            branch,
            depth,
            project_name,
            filter,
        } => (
            cmd::git::run_ingest_git(
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
            filter,
            delay,
            request_timeout_secs,
            connect_timeout_secs,
            max_retries,
            redirect_policy,
            max_page_bytes,
            max_total_bytes,
            concurrency,
            query,
            threshold,
            project_name,
        } => {
            let config = cli::config::IngestConfig {
                filter,
                delay,
                query,
                threshold,
                project_name,
                request_timeout_secs,
                connect_timeout_secs,
                max_retries,
                redirect_policy,
                max_page_bytes,
                max_total_bytes,
                concurrency_limit: concurrency,
            };
            (cmd::ingest::run_ingest(&url, &output, &config).await, None)
        }
        Commands::Watch {
            url,
            output,
            cache,
            no_sitemap,
            filter,
            delay,
            request_timeout_secs,
            max_retries,
            redirect_policy,
            concurrency,
            json,
        } => {
            let fmt = if json {
                cmd::watch::OutputFormat::Json
            } else {
                cmd::watch::OutputFormat::Markdown
            };
            let sitemap_strategy = if no_sitemap {
                scrape::SitemapStrategy::CrawlOnly
            } else {
                scrape::SitemapStrategy::UseSitemap
            };
            (
                cmd::watch::run_watch(
                    &url,
                    &output,
                    &cache,
                    filter.as_deref(),
                    delay,
                    request_timeout_secs,
                    max_retries,
                    redirect_policy,
                    concurrency,
                    fmt,
                    sitemap_strategy,
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
                cmd::watch::ConfirmMode::AutoConfirm
            } else {
                cmd::watch::ConfirmMode::Interactive
            };
            (
                cmd::watch::run_apply(&url, &cache, &scrape_dir, mode).await,
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
                cmd::watch::OutputFormat::Json
            } else {
                cmd::watch::OutputFormat::Markdown
            };
            (
                cmd::watch::run_diff(&dir_a, &dir_b, output.as_deref(), fmt),
                None,
            )
        }
    };

    match result {
        Ok(()) => ExitCode::SUCCESS,
        Err(err) => handle_error(err, search_context),
    }
}

fn handle_error(err: anyhow::Error, search_context: Option<(bool, String)>) -> ExitCode {
    if let Some((json_mode, search_query)) = search_context {
        if json_mode {
            let msg = err.to_string();
            if msg.starts_with(cmd::search::SEARCH_JSON_ALREADY_EMITTED_PREFIX) {
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
    let code = sys::error::map_error_to_exit_code(&err);
    eprintln!("Error: {err}");
    match code {
        0 => ExitCode::SUCCESS,
        2 => ExitCode::from(2),
        _ => ExitCode::FAILURE,
    }
}

fn exit_clap(e: clap::error::Error) -> ExitCode {
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
    } else if is_val && !e.to_string().to_lowercase().contains("limit must be") {
        2
    } else {
        1
    } {
        0 => ExitCode::SUCCESS,
        2 => ExitCode::from(2),
        _ => ExitCode::FAILURE,
    }
}

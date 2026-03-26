//! `ctd` v0.6.1 — AI-Optimized Documentation Indexer
//!
//! CLI entry point for the `ctd` pipeline. Exposes four sub-commands
//! that can be composed to go from a raw documentation source (local files **or**
//! a live website) to a fully indexed, AI-queryable knowledge base.
//!

// Strict functional programming constraints
#![allow(clippy::all)]
#![deny(clippy::unwrap_used)]
#![deny(clippy::panic)]
#![deny(clippy::arithmetic_side_effects)]
#![deny(clippy::expect_used)]

pub mod analyze;
pub mod assign;
pub mod chunk;
pub mod chunking_adapter;
pub mod config;
pub mod discover;
#[cfg(feature = "enhanced")]
pub mod features;
pub mod filter;
pub mod graph;
pub mod highlight;
pub mod index;
pub mod llms;
pub mod math_types;
pub mod scrape;
pub mod search;
pub mod transform;
pub mod types;
pub mod validate;

pub mod cli;
pub mod cmd;
pub mod sys;

use anyhow::Result;
use clap::{CommandFactory, FromArgMatches};
use cli::{Cli, Commands};
use std::process;

#[tokio::main]
async fn main() -> Result<()> {
    let cmd = Cli::command();

    let cli_matches = match cmd.try_get_matches() {
        Ok(matches) => matches,
        Err(e) => {
            if e.kind() == clap::error::ErrorKind::DisplayHelp
                || e.kind() == clap::error::ErrorKind::DisplayVersion
            {
                eprintln!("{}", e);
                process::exit(0);
            }
            let validation_errors = [
                clap::error::ErrorKind::ValueValidation,
                clap::error::ErrorKind::InvalidValue,
            ];
            let err_str: String = e.to_string().to_lowercase();
            let exit_code = if validation_errors.contains(&e.kind()) {
                if err_str.contains("limit must be") {
                    1
                } else {
                    2
                }
            } else {
                1
            };
            eprintln!("{}", e);
            process::exit(exit_code);
        }
    };

    let cli_app = match Cli::from_arg_matches(&cli_matches) {
        Ok(c) => c,
        Err(e) => {
            let validation_errors = [
                clap::error::ErrorKind::ValueValidation,
                clap::error::ErrorKind::InvalidValue,
            ];
            let err_str: String = e.to_string().to_lowercase();
            let exit_code = if validation_errors.contains(&e.kind()) {
                if err_str.contains("limit must be") {
                    1
                } else {
                    2
                }
            } else {
                1
            };
            eprintln!("{}", e);
            process::exit(exit_code);
        }
    };

    let (result, search_context) = match cli_app.command {
        Commands::Search {
            query,
            index_dir,
            limit,
            no_color,
            json,
        } => {
            let ctx = Some((json, query.clone()));
            (
                cmd::search::run_search(&query, &index_dir, limit, !no_color, json),
                ctx,
            )
        }
        Commands::Scrape {
            url,
            output,
            no_sitemap,
            filter,
            delay,
            query,
            threshold,
            request_timeout_secs,
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
            filter,
            delay,
            request_timeout_secs,
            max_retries,
            redirect_policy,
            concurrency,
            json,
        } => (
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
                json,
            )
            .await,
            None,
        ),
        Commands::Apply {
            url,
            cache,
            scrape_dir,
            yes,
        } => (
            cmd::watch::run_apply(&url, &cache, &scrape_dir, yes).await,
            None,
        ),
        Commands::Diff {
            dir_a,
            dir_b,
            output,
            json,
        } => (
            cmd::watch::run_diff(&dir_a, &dir_b, output.as_deref(), json),
            None,
        ),
    };

    match result {
        Ok(()) => Ok(()),
        Err(err) => {
            if let Some((json_mode, search_query)) = search_context {
                if json_mode {
                    let error_message: String = err.to_string();
                    if error_message.starts_with(cmd::search::SEARCH_JSON_ALREADY_EMITTED_PREFIX) {
                        process::exit(0);
                    }
                    let json_error = serde_json::json!({
                        "status": "error",
                        "query": search_query,
                        "error": error_message,
                    });
                    println!("{}", serde_json::to_string_pretty(&json_error)?);
                    process::exit(1);
                }
            }
            let exit_code = sys::error::map_error_to_exit_code(&err);
            eprintln!("Error: {err}");
            process::exit(exit_code);
        }
    }
}

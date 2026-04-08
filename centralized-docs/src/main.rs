//! `ctd` v0.7.0 — AI-Optimized Documentation Indexer
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
mod dispatch;
pub mod sys;

use clap::{CommandFactory, FromArgMatches};
use cli::Cli;
use std::process::ExitCode;
use tracing::instrument;

#[tokio::main]
#[instrument(skip_all)]
async fn main() -> ExitCode {
    let cmd = Cli::command();
    let cli_matches = match cmd.try_get_matches() {
        Ok(matches) => matches,
        Err(e) => return dispatch::exit_clap(e),
    };
    let cli_app = match Cli::from_arg_matches(&cli_matches) {
        Ok(c) => c,
        Err(e) => return dispatch::exit_clap(e),
    };

    let (result, search_context) = dispatch::dispatch(cli_app.command).await;

    match result {
        Ok(()) => ExitCode::SUCCESS,
        Err(err) => dispatch::handle_error(err, search_context),
    }
}

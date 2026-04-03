import os

with open("src/main.rs", "r") as f:
    lines = f.readlines()

def write_slice(filename, start_line, end_line, prefix="", suffix=""):
    os.makedirs(os.path.dirname(filename), exist_ok=True)
    with open(filename, "w") as f:
        f.write(prefix)
        # start_line is 1-indexed
        f.write("".join(lines[start_line-1:end_line]))
        f.write(suffix)

# Create src/cli/config.rs
write_slice("src/cli/config.rs", 88, 186, 
            prefix="use std::path::PathBuf;\nuse crate::scrape::SitemapStrategy;\nuse spider::configuration::RedirectPolicy;\n\n")

# Create src/cli/validation.rs
# Lines 188-465
write_slice("src/cli/validation.rs", 188, 465, 
            prefix="use anyhow::Result;\nuse spider::configuration::RedirectPolicy;\nuse crate::validate;\n\n")

# Create src/cli/mod.rs
# Lines 468-715
write_slice("src/cli/mod.rs", 468, 715, 
            prefix="pub mod config;\npub mod validation;\n\nuse clap::{Parser, Subcommand};\nuse std::path::PathBuf;\nuse spider::configuration::RedirectPolicy;\nuse validation::*;\n\n")

# Create src/sys/error.rs
# Lines 1012-1099
write_slice("src/sys/error.rs", 1012, 1099, prefix="use anyhow::Result;\n\n")

# Create src/sys/lock.rs
# Lines 1447-1618
write_slice("src/sys/lock.rs", 1447, 1618, prefix="use serde::{Deserialize, Serialize};\nuse std::path::{Path, PathBuf};\nuse std::process;\nuse std::fs::OpenOptions;\nuse std::io::Write;\nuse fs2::FileExt;\nuse anyhow::Result;\nuse std::time::{SystemTime, UNIX_EPOCH};\n\n")

# Create src/sys/mod.rs
write_slice("src/sys/mod.rs", 1, 0, prefix="pub mod error;\npub mod lock;\n")

# Create src/cmd/scrape.rs
# Contains validate_query_length (1101-1117), apply_query_filter (1119-1226), run_scrape (1228-1379)
write_slice("src/cmd/scrape.rs", 1101, 1379, prefix="use anyhow::Result;\nuse std::path::Path;\nuse std::process;\nuse crate::scrape;\nuse crate::cli::config::ScrapeCommandConfig;\nuse crate::cli::validation::validate_threshold;\n\n")

# Create src/cmd/index.rs
# Contains run_index (1620-1794)
write_slice("src/cmd/index.rs", 1620, 1794, prefix="use anyhow::Result;\nuse std::path::Path;\nuse std::path::PathBuf;\nuse crate::{analyze, assign, chunking_adapter, discover, index, llms, transform, validate};\nuse crate::cli::config::IndexConfig;\nuse crate::sys::lock::acquire_output_lock;\n\n")

# Create src/cmd/ingest.rs
# Contains run_ingest (1796-1913)
write_slice("src/cmd/ingest.rs", 1796, 1913, prefix="use anyhow::Result;\nuse std::path::Path;\nuse crate::scrape::{self, SitemapStrategy};\nuse crate::cli::config::{IngestConfig, IndexConfig};\nuse crate::cmd::scrape::{apply_query_filter, validate_query_length};\nuse crate::cmd::index::run_index;\nuse crate::cli::validation::validate_filter_regex;\n\nconst DEFAULT_MAX_PAGE_SIZE_BYTES: u64 = 10 * 1024 * 1024;\nconst DEFAULT_MAX_TOTAL_SIZE_BYTES: u64 = 500 * 1024 * 1024;\n\n")

# Create src/cmd/search.rs
# Contains run_search (1915-2065)
write_slice("src/cmd/search.rs", 1915, 2065, prefix="use anyhow::Result;\nuse std::path::Path;\nuse serde::Serialize;\nuse crate::validate;\nuse crate::cli::validation::*;\n\nconst SEARCH_JSON_ALREADY_EMITTED_PREFIX: &str = \"__SEARCH_JSON_ALREADY_EMITTED__\";\n\n")

# utils for dir (1381-1445)
write_slice("src/sys/dir.rs", 1381, 1445, prefix="use anyhow::Result;\nuse std::path::Path;\n\n")

# utils.rs extract_last_path_segment (1568-1574)
write_slice("src/sys/utils.rs", 1568, 1574, prefix="")

# Update sys/mod.rs
with open("src/sys/mod.rs", "w") as f:
    f.write("pub mod error;\npub mod lock;\npub mod dir;\npub mod utils;\n")

# We will need the git ingest code. Let's put it in src/cmd/git.rs
# Lines 857-945
write_slice("src/cmd/git.rs", 857, 945, prefix="use anyhow::Result;\nuse std::path::PathBuf;\nuse crate::cli::config::IndexConfig;\nuse crate::cmd::index::run_index;\nuse crate::sys::utils::extract_last_path_segment;\n\n")

# The main.rs should now just be the entrypoint and module definitions
# Lines 1-86, then the `#[tokio::main] async fn main()` (717-1010)

print("Split completed.")

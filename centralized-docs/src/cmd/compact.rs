//! Handler for the `ctd compact` CLI subcommand.
//!
//! Compacts an on-disk redb state database to reclaim space from deleted
//! and updated entries. This is an expensive operation that requires
//! exclusive access to the database.

use crate::state::commit::compact_state_db;
use std::path::Path;

/// Run the `ctd compact` command.
///
/// Opens the database at `path` exclusively, compacts it, and reports
/// the result to stdout.
///
/// # Errors
///
/// Returns an error if the database cannot be opened or compaction fails.
pub fn run_compact(path: &Path) -> Result<(), anyhow::Error> {
    eprintln!("Compacting database at {}...", path.display());
    let compacted = compact_state_db(path)?;
    if compacted {
        eprintln!("Compaction completed successfully.");
    } else {
        eprintln!("No further compaction possible — database is already compact.");
    }
    Ok(())
}

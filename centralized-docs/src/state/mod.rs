//! State database table definitions and Pod types for raw state storage.

#![deny(clippy::unwrap_used)]
#![deny(clippy::expect_used)]
#![deny(clippy::panic)]
#![warn(clippy::pedantic)]
#![forbid(unsafe_code)]

pub mod bulk_load;
pub mod commit;

mod error;
mod init;
mod pod;
mod serialization;
mod tables;

pub use error::{DurabilityConfig, StateError, StateLoadError};
pub use init::initialize_tables;
pub use pod::{read_file_state_raw, read_url_state_raw, FileStateRaw, UrlStateRaw};
pub use serialization::serialize_snapshot;
pub use tables::{
    analysis_outputs_table, chunk_outputs_table, file_state_table, metadata_table,
    scrape_outputs_table, snapshots_table, source_path_chunks_table, transform_outputs_table,
    url_state_table, validate_hash_key, validate_source_path, validate_url_key,
    TABLE_NAME_ANALYSIS_OUTPUTS, TABLE_NAME_CHUNK_OUTPUTS, TABLE_NAME_FILE_STATE,
    TABLE_NAME_METADATA, TABLE_NAME_SCRAPE_OUTPUTS, TABLE_NAME_SNAPSHOTS,
    TABLE_NAME_SOURCE_PATH_CHUNKS, TABLE_NAME_TRANSFORM_OUTPUTS, TABLE_NAME_URL_STATE,
};

pub use commit::*;

#[cfg(test)]
mod tests_integration;

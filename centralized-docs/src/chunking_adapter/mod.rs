//! Adapter layer between `ctd` and `contextual-chunker`
//!
//! This module provides conversion functions between `ctd`'s types
//! and `contextual-chunker`'s types, enabling clean separation of concerns while
//! maintaining all `ctd`-specific functionality.

pub mod cache_ops;
pub mod chunk_all;
pub mod chunk_all_cached;
pub mod error;
pub mod types;

// Re-exports for backward compatibility
pub use cache_ops::{
    chunk_cache_key, compute_chunker_config_hash, partition_by_cache_status, write_chunk_file,
};
pub use chunk_all::chunk_all;
pub use chunk_all_cached::chunk_all_cached;
pub use error::ChunkReuseError;
pub use types::{Chunk, ChunksResult};

#[cfg(test)]
mod tests;

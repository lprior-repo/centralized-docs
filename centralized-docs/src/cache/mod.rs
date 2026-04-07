//! Idempotent cache layer using redb for high-performance ACID key-value storage.

mod config;
mod doc_cache;
mod types;

pub use config::*;
pub use doc_cache::DocCache;
pub use types::*;

#[cfg(test)]
mod tests;

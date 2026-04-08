//! Spider-rs feature extensions with type-safe configuration.
//!
//! All feature configuration uses newtype patterns for compile-time safety
//! and zero-cost abstraction when features are disabled.

#![allow(dead_code)]
#![allow(clippy::unnecessary_wraps)]
#![allow(clippy::trivially_copy_pass_by_ref)]

mod cache;
mod config;
mod errors;
mod filtering;

#[cfg(feature = "anti-detection")]
mod anti_detection;

#[cfg(feature = "javascript")]
mod javascript;

// Re-exports — preserve all public APIs
pub use cache::{CacheConfig, CacheTtl};
pub use config::{FeatureConfig, FeatureConfigBuilder};
pub use errors::FeatureError;
pub use filtering::{FilteringConfig, GlobPattern, RegexPattern};

#[cfg(feature = "anti-detection")]
pub use anti_detection::{AntiDetectionConfig, Strategy};

#[cfg(feature = "javascript")]
pub use javascript::{JavascriptConfig, Milliseconds, RenderMode};

#[cfg(test)]
mod tests;

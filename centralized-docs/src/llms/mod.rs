//! llms.txt generation module
//!
//! Generates llms.txt and AGENTS.md files following the llms.txt specification.
//! These files provide AI-friendly entry points into the documentation.
//!
//! Specification: <https://llmstxt.org/>

pub mod config;
pub mod generate_agents;
pub mod generate_llms;
pub mod text_utils;

// Re-export all public API for backward compatibility
pub use config::LlmsConfig;
pub use generate_agents::generate_agents_md;
pub use generate_llms::generate_llms_txt;
pub use text_utils::truncate_summary;

#[cfg(test)]
mod tests;

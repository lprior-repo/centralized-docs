//! Configuration for knowledge graph construction and category detection.

mod category_config;
mod graph_config;

#[cfg(test)]
mod tests_category;
#[cfg(test)]
mod tests_graph;
#[cfg(test)]
mod tests_graph_edge;

pub use category_config::{CategoryConfig, CategoryRule, MatchCriteria};
pub use graph_config::GraphConfig;

//! Knowledge graph module - Directed Acyclic Graph for knowledge representation.

mod dag;
mod types;

pub use dag::KnowledgeDAG;
pub use types::*;

#[cfg(test)]
mod tests;

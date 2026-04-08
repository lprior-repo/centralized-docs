use serde::{Deserialize, Serialize};

use super::{ScipSymbolId, SymbolKind, SymbolRole};

/// Directed semantic relationship between symbols.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum RelationshipKind {
    Imports,
    Contains,
    Implements,
    Calls,
    References,
    Overrides,
}

impl std::fmt::Display for RelationshipKind {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Imports => write!(f, "imports"),
            Self::Contains => write!(f, "contains"),
            Self::Implements => write!(f, "implements"),
            Self::Calls => write!(f, "calls"),
            Self::References => write!(f, "references"),
            Self::Overrides => write!(f, "overrides"),
        }
    }
}

/// A directed semantic edge from one symbol to another.
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct SymbolRelationship {
    pub target: ScipSymbolId,
    pub kind: RelationshipKind,
}

/// A semantic symbol with identity, kind, roles, and relationships.
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct SymbolNode {
    pub scip_id: ScipSymbolId,
    pub kind: SymbolKind,
    pub roles: SymbolRole,
    pub display_name: String,
    pub signature: Option<String>,
    pub documentation: Option<String>,
    pub relationships: Vec<SymbolRelationship>,
}

#[cfg(test)]
#[path = "symbol_graph_tests_relationship.rs"]
mod tests_relationship;

#[cfg(test)]
#[path = "symbol_graph_tests_node.rs"]
mod tests_node;

//! SCIP-inspired semantic domain model for source code symbols.

mod scip_symbol_id;
mod symbol_graph;
mod symbol_kind;
mod symbol_role;

pub use scip_symbol_id::{ScipSymbolId, ScipSymbolIdError};
pub use symbol_graph::{RelationshipKind, SymbolNode, SymbolRelationship};
pub use symbol_kind::{SymbolKind, SymbolKindError};
pub use symbol_role::{SymbolRole, SymbolRoleError};

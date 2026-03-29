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
mod tests {
    use super::*;
    use proptest::prelude::*;
    use std::collections::{HashMap, HashSet};

    // ═══════════════════════════════════════════════════════════════════════════
    // RelationshipKind (BDD 3.18)
    // ═══════════════════════════════════════════════════════════════════════════

    #[test]
    fn relationship_kind_display_outputs_lowercase_name_for_each_variant() {
        assert_eq!(format!("{}", RelationshipKind::Imports), "imports");
        assert_eq!(format!("{}", RelationshipKind::Contains), "contains");
        assert_eq!(format!("{}", RelationshipKind::Implements), "implements");
        assert_eq!(format!("{}", RelationshipKind::Calls), "calls");
        assert_eq!(format!("{}", RelationshipKind::References), "references");
        assert_eq!(format!("{}", RelationshipKind::Overrides), "overrides");
    }

    #[test]
    fn relationship_kind_serializes_as_lowercase_string_for_each_variant() {
        assert_eq!(
            serde_json::to_string(&RelationshipKind::Implements).unwrap(),
            "\"implements\""
        );
    }

    #[test]
    fn relationship_kind_deserializes_from_lowercase_string_for_each_variant() {
        assert_eq!(
            serde_json::from_str::<RelationshipKind>("\"calls\"").unwrap(),
            RelationshipKind::Calls
        );
    }

    #[test]
    fn relationship_kind_returns_deserialization_error_when_given_unknown_string() {
        let result = serde_json::from_str::<RelationshipKind>("\"FooBar\"");
        let err = result.expect_err("unknown variant should produce deserialization error");
        let msg = err.to_string();
        assert!(
            msg.contains("FooBar"),
            "Expected error containing 'FooBar', got: {msg}"
        );
    }

    #[test]
    fn relationship_kind_round_trips_imports_through_json() {
        let json = serde_json::to_string(&RelationshipKind::Imports).unwrap();
        let back: RelationshipKind = serde_json::from_str(&json).unwrap();
        assert_eq!(back, RelationshipKind::Imports);
    }

    #[test]
    fn relationship_kind_round_trips_contains_through_json() {
        let json = serde_json::to_string(&RelationshipKind::Contains).unwrap();
        let back: RelationshipKind = serde_json::from_str(&json).unwrap();
        assert_eq!(back, RelationshipKind::Contains);
    }

    #[test]
    fn relationship_kind_round_trips_implements_through_json() {
        let json = serde_json::to_string(&RelationshipKind::Implements).unwrap();
        let back: RelationshipKind = serde_json::from_str(&json).unwrap();
        assert_eq!(back, RelationshipKind::Implements);
    }

    #[test]
    fn relationship_kind_round_trips_calls_through_json() {
        let json = serde_json::to_string(&RelationshipKind::Calls).unwrap();
        let back: RelationshipKind = serde_json::from_str(&json).unwrap();
        assert_eq!(back, RelationshipKind::Calls);
    }

    #[test]
    fn relationship_kind_round_trips_references_through_json() {
        let json = serde_json::to_string(&RelationshipKind::References).unwrap();
        let back: RelationshipKind = serde_json::from_str(&json).unwrap();
        assert_eq!(back, RelationshipKind::References);
    }

    #[test]
    fn relationship_kind_round_trips_overrides_through_json() {
        let json = serde_json::to_string(&RelationshipKind::Overrides).unwrap();
        let back: RelationshipKind = serde_json::from_str(&json).unwrap();
        assert_eq!(back, RelationshipKind::Overrides);
    }

    // ═══════════════════════════════════════════════════════════════════════════
    // SymbolRelationship (BDD 3.19)
    // ═══════════════════════════════════════════════════════════════════════════

    #[test]
    fn symbol_relationship_equality_compares_target_and_kind_when_compared() {
        let id1 = ScipSymbolId::new("rust", "auth", "login").unwrap();
        let id2 = ScipSymbolId::new("python", "auth", "login").unwrap();
        let rel1 = SymbolRelationship {
            target: id1.clone(),
            kind: RelationshipKind::Calls,
        };
        let rel2 = SymbolRelationship {
            target: id1.clone(),
            kind: RelationshipKind::Calls,
        };
        let rel3 = SymbolRelationship {
            target: id2,
            kind: RelationshipKind::Calls,
        };
        assert_eq!(rel1, rel2);
        assert_ne!(rel1, rel3);
    }

    #[test]
    fn symbol_relationship_hash_allows_use_in_hashset_when_inserted() {
        let id = ScipSymbolId::new("rust", "auth", "login").unwrap();
        let rel = SymbolRelationship {
            target: id.clone(),
            kind: RelationshipKind::Imports,
        };
        assert!(HashSet::from([rel.clone()]).contains(&rel));
    }

    #[test]
    fn symbol_relationship_hash_eq_consistency_when_same_target_different_kind() {
        let id = ScipSymbolId::new("rust", "auth", "login").unwrap();
        let rel_calls = SymbolRelationship {
            target: id.clone(),
            kind: RelationshipKind::Calls,
        };
        let rel_refs = SymbolRelationship {
            target: id,
            kind: RelationshipKind::References,
        };
        assert_ne!(rel_calls, rel_refs);
        let set: HashSet<SymbolRelationship> = HashSet::from([rel_calls.clone(), rel_refs.clone()]);
        assert_eq!(set.len(), 2);
        let map: HashMap<SymbolRelationship, i32> = HashMap::from([(rel_calls, 1), (rel_refs, 2)]);
        assert_eq!(map.len(), 2);
    }

    #[test]
    fn symbol_relationship_round_trips_through_json_when_serialized() {
        let id = ScipSymbolId::new("rust", "auth/AuthService", "login()").unwrap();
        let rel = SymbolRelationship {
            target: id.clone(),
            kind: RelationshipKind::Calls,
        };
        let json = serde_json::to_string(&rel).unwrap();
        let reconstructed: SymbolRelationship = serde_json::from_str(&json).unwrap();
        assert_eq!(reconstructed, rel);
    }

    // ═══════════════════════════════════════════════════════════════════════════
    // SymbolNode (BDD 3.20)
    // ═══════════════════════════════════════════════════════════════════════════

    #[test]
    fn symbol_node_equality_compares_all_fields_when_compared() {
        let id = ScipSymbolId::new("rust", "auth", "f").unwrap();
        let node1 = SymbolNode {
            scip_id: id.clone(),
            kind: SymbolKind::Struct,
            roles: SymbolRole::DEFINITION,
            display_name: "Auth".to_string(),
            signature: Some("fn()".to_string()),
            documentation: Some("doc".to_string()),
            relationships: vec![],
        };
        let node2 = node1.clone();
        let node3 = SymbolNode {
            display_name: "DifferentName".to_string(),
            ..node1.clone()
        };
        assert_eq!(node1, node2);
        assert_ne!(node1, node3);
    }

    #[test]
    fn symbol_node_hash_allows_use_as_graph_key_in_hashmap_when_inserted() {
        let id = ScipSymbolId::new("rust", "auth", "f").unwrap();
        let node = SymbolNode {
            scip_id: id.clone(),
            kind: SymbolKind::Struct,
            roles: SymbolRole::DEFINITION,
            display_name: "Auth".to_string(),
            signature: None,
            documentation: None,
            relationships: vec![],
        };
        let map: HashMap<ScipSymbolId, SymbolNode> = HashMap::from([(id, node.clone())]);
        assert_eq!(map.get(&node.scip_id), Some(&node));
    }

    #[test]
    fn symbol_node_hash_eq_consistency_when_nodes_differ_in_non_id_fields() {
        let id = ScipSymbolId::new("rust", "auth", "f").unwrap();
        let node_a = SymbolNode {
            scip_id: id.clone(),
            kind: SymbolKind::Struct,
            roles: SymbolRole::DEFINITION,
            display_name: "AuthService".to_string(),
            signature: None,
            documentation: None,
            relationships: vec![],
        };
        let node_b = SymbolNode {
            scip_id: id,
            kind: SymbolKind::Struct,
            roles: SymbolRole::DEFINITION,
            display_name: "DifferentName".to_string(),
            signature: None,
            documentation: None,
            relationships: vec![],
        };
        assert_ne!(node_a, node_b);
        let set: HashSet<SymbolNode> = HashSet::from([node_a.clone(), node_b.clone()]);
        assert_eq!(set.len(), 2);
    }

    #[test]
    fn symbol_node_serializes_all_fields_when_all_fields_populated() {
        let id = ScipSymbolId::new("rust", "auth", "f").unwrap();
        let rel = SymbolRelationship {
            target: id.clone(),
            kind: RelationshipKind::Calls,
        };
        let node = SymbolNode {
            scip_id: id,
            kind: SymbolKind::Struct,
            roles: SymbolRole::DEFINITION,
            display_name: "Auth".to_string(),
            signature: Some("fn()".to_string()),
            documentation: Some("doc".to_string()),
            relationships: vec![rel],
        };
        let json = serde_json::to_string(&node).unwrap();
        assert!(json.contains("\"scip_id\""));
        assert!(json.contains("\"kind\""));
        assert!(json.contains("\"roles\""));
        assert!(json.contains("\"display_name\""));
        assert!(json.contains("\"signature\""));
        assert!(json.contains("\"documentation\""));
        assert!(json.contains("\"relationships\""));
    }

    #[test]
    fn symbol_node_deserializes_from_json_when_all_fields_present() {
        let json = r#"{"scip_id":"rust/auth#f","kind":"struct","roles":1,"display_name":"Auth","signature":"fn()","documentation":"doc","relationships":[]}"#;
        let node: SymbolNode = serde_json::from_str(json).expect("should deserialize");
        assert_eq!(node.scip_id.as_str(), "rust/auth#f");
        assert_eq!(node.kind, SymbolKind::Struct);
        assert_eq!(node.roles.bits(), 1);
        assert_eq!(node.display_name, "Auth");
        assert_eq!(node.signature, Some("fn()".to_string()));
        assert_eq!(node.documentation, Some("doc".to_string()));
        assert!(node.relationships.is_empty());
    }

    #[test]
    fn symbol_node_round_trips_with_none_optionals_when_fields_are_empty() {
        let id = ScipSymbolId::new("rust", "auth", "f").unwrap();
        let node = SymbolNode {
            scip_id: id,
            kind: SymbolKind::Struct,
            roles: SymbolRole::DEFINITION,
            display_name: "Auth".to_string(),
            signature: None,
            documentation: None,
            relationships: vec![],
        };
        let json = serde_json::to_string(&node).unwrap();
        let reconstructed: SymbolNode = serde_json::from_str(&json).unwrap();
        assert_eq!(reconstructed.signature, None);
        assert_eq!(reconstructed.documentation, None);
        assert!(reconstructed.relationships.is_empty());
    }

    #[test]
    fn symbol_node_round_trips_with_populated_optionals_when_all_fields_set() {
        let id = ScipSymbolId::new("rust", "auth", "f").unwrap();
        let target_id = ScipSymbolId::new("rust", "other", "g").unwrap();
        let rel = SymbolRelationship {
            target: target_id,
            kind: RelationshipKind::Calls,
        };
        let node = SymbolNode {
            scip_id: id,
            kind: SymbolKind::Struct,
            roles: SymbolRole::DEFINITION,
            display_name: "Auth".to_string(),
            signature: Some("fn login() -> bool".to_string()),
            documentation: Some("Logs in user".to_string()),
            relationships: vec![rel],
        };
        let json = serde_json::to_string(&node).unwrap();
        let reconstructed: SymbolNode = serde_json::from_str(&json).unwrap();
        assert_eq!(
            reconstructed.signature,
            Some("fn login() -> bool".to_string())
        );
        assert_eq!(
            reconstructed.documentation,
            Some("Logs in user".to_string())
        );
        assert_eq!(reconstructed.relationships.len(), 1);
    }

    // ═══════════════════════════════════════════════════════════════════════════
    // Proptests
    // ═════════════════════════════════════════════════════════════════════════

    proptest! {
        #[test]
        fn symbol_node_serde_roundtrips_for_valid_fields(
            scheme in "[a-z]{1,5}",
            module_path in "[a-z]{1,5}",
            descriptor in "[a-z]{1,5}",
            display_name in "[a-zA-Z]{1,10}",
        ) {
            let id = ScipSymbolId::new(&scheme, &module_path, &descriptor).unwrap();
            let node = SymbolNode {
                scip_id: id.clone(),
                kind: SymbolKind::Struct,
                roles: SymbolRole::DEFINITION,
                display_name,
                signature: None,
                documentation: None,
                relationships: vec![],
            };
            let json = serde_json::to_value(&node).unwrap();
            let reconstructed: SymbolNode = serde_json::from_value(json).unwrap();
            assert_eq!(reconstructed, node);
        }
    }
}

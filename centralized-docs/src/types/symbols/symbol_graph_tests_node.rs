#![allow(clippy::unwrap_used, clippy::expect_used)]
use super::*;
use proptest::prelude::*;
use std::collections::{HashMap, HashSet};

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

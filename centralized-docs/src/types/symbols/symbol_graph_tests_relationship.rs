#![allow(clippy::unwrap_used, clippy::expect_used)]
use super::*;
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

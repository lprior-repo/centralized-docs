#![allow(clippy::unwrap_used, clippy::expect_used)]
use super::*;
use proptest::prelude::*;

// ═══════════════════════════════════════════════════════════════════════════
// Display (BDD 3.17)
// ═══════════════════════════════════════════════════════════════════════════

#[test]
fn symbol_kind_display_outputs_lowercase_name_for_each_variant() {
    assert_eq!(format!("{}", SymbolKind::Struct), "struct");
    assert_eq!(format!("{}", SymbolKind::Function), "function");
    assert_eq!(format!("{}", SymbolKind::Trait), "trait");
    assert_eq!(format!("{}", SymbolKind::Enum), "enum");
    assert_eq!(format!("{}", SymbolKind::Module), "module");
    assert_eq!(format!("{}", SymbolKind::Constant), "constant");
    assert_eq!(format!("{}", SymbolKind::Method), "method");
    assert_eq!(format!("{}", SymbolKind::Field), "field");
    assert_eq!(format!("{}", SymbolKind::Interface), "interface");
    assert_eq!(format!("{}", SymbolKind::TypeAlias), "type_alias");
    assert_eq!(format!("{}", SymbolKind::Variable), "variable");
    assert_eq!(format!("{}", SymbolKind::Package), "package");
}

// ═══════════════════════════════════════════════════════════════════════════
// Serialize (BDD 3.17)
// ═══════════════════════════════════════════════════════════════════════════

#[test]
fn symbol_kind_serializes_as_lowercase_string_when_all_variants_tested() {
    assert_eq!(
        serde_json::to_string(&SymbolKind::Struct).unwrap(),
        "\"struct\""
    );
    assert_eq!(
        serde_json::to_string(&SymbolKind::Function).unwrap(),
        "\"function\""
    );
    assert_eq!(
        serde_json::to_string(&SymbolKind::Trait).unwrap(),
        "\"trait\""
    );
    assert_eq!(
        serde_json::to_string(&SymbolKind::Enum).unwrap(),
        "\"enum\""
    );
    assert_eq!(
        serde_json::to_string(&SymbolKind::Module).unwrap(),
        "\"module\""
    );
    assert_eq!(
        serde_json::to_string(&SymbolKind::Constant).unwrap(),
        "\"constant\""
    );
    assert_eq!(
        serde_json::to_string(&SymbolKind::Method).unwrap(),
        "\"method\""
    );
    assert_eq!(
        serde_json::to_string(&SymbolKind::Field).unwrap(),
        "\"field\""
    );
    assert_eq!(
        serde_json::to_string(&SymbolKind::Interface).unwrap(),
        "\"interface\""
    );
    assert_eq!(
        serde_json::to_string(&SymbolKind::TypeAlias).unwrap(),
        "\"type_alias\""
    );
    assert_eq!(
        serde_json::to_string(&SymbolKind::Variable).unwrap(),
        "\"variable\""
    );
    assert_eq!(
        serde_json::to_string(&SymbolKind::Package).unwrap(),
        "\"package\""
    );
}

// ═══════════════════════════════════════════════════════════════════════════
// Deserialize (BDD 3.17)
// ═══════════════════════════════════════════════════════════════════════════

#[test]
fn symbol_kind_deserializes_from_lowercase_string_for_each_variant() {
    assert_eq!(
        serde_json::from_str::<SymbolKind>("\"struct\"").unwrap(),
        SymbolKind::Struct
    );
    assert_eq!(
        serde_json::from_str::<SymbolKind>("\"function\"").unwrap(),
        SymbolKind::Function
    );
    assert_eq!(
        serde_json::from_str::<SymbolKind>("\"trait\"").unwrap(),
        SymbolKind::Trait
    );
    assert_eq!(
        serde_json::from_str::<SymbolKind>("\"enum\"").unwrap(),
        SymbolKind::Enum
    );
    assert_eq!(
        serde_json::from_str::<SymbolKind>("\"module\"").unwrap(),
        SymbolKind::Module
    );
    assert_eq!(
        serde_json::from_str::<SymbolKind>("\"constant\"").unwrap(),
        SymbolKind::Constant
    );
    assert_eq!(
        serde_json::from_str::<SymbolKind>("\"method\"").unwrap(),
        SymbolKind::Method
    );
    assert_eq!(
        serde_json::from_str::<SymbolKind>("\"field\"").unwrap(),
        SymbolKind::Field
    );
    assert_eq!(
        serde_json::from_str::<SymbolKind>("\"interface\"").unwrap(),
        SymbolKind::Interface
    );
    assert_eq!(
        serde_json::from_str::<SymbolKind>("\"type_alias\"").unwrap(),
        SymbolKind::TypeAlias
    );
    assert_eq!(
        serde_json::from_str::<SymbolKind>("\"variable\"").unwrap(),
        SymbolKind::Variable
    );
    assert_eq!(
        serde_json::from_str::<SymbolKind>("\"package\"").unwrap(),
        SymbolKind::Package
    );
}

#[test]
fn symbol_kind_returns_unknown_kind_error_when_deserializing_invalid_string() {
    let result = serde_json::from_str::<SymbolKind>("\"unknown_kind\"");
    let err = result.expect_err("should fail for unknown kind");
    let msg = err.to_string();
    assert!(
        msg.contains("unknown_kind"),
        "Expected error containing 'unknown_kind', got: {msg}"
    );
}

#[test]
fn symbol_kind_returns_unknown_kind_error_when_deserializing_empty_string() {
    let result = serde_json::from_str::<SymbolKind>("\"\"");
    let err = result.expect_err("should fail for empty string");
    let msg = err.to_string();
    assert!(
        msg.contains("Unknown SymbolKind"),
        "Expected error containing 'Unknown SymbolKind', got: {msg}"
    );
}

#[test]
fn symbol_kind_deserializes_case_insensitively_when_input_has_mixed_case() {
    assert_eq!(
        serde_json::from_str::<SymbolKind>("\"Struct\"").unwrap(),
        SymbolKind::Struct
    );
    assert_eq!(
        serde_json::from_str::<SymbolKind>("\"FUNCTION\"").unwrap(),
        SymbolKind::Function
    );
    assert_eq!(
        serde_json::from_str::<SymbolKind>("\"Type_Alias\"").unwrap(),
        SymbolKind::TypeAlias
    );
}

// ═══════════════════════════════════════════════════════════════════════════
// Round-Trip Per Variant (BDD 3.17)
// ═══════════════════════════════════════════════════════════════════════════

#[test]
fn symbol_kind_round_trips_struct_through_json() {
    let json = serde_json::to_string(&SymbolKind::Struct).unwrap();
    let back: SymbolKind = serde_json::from_str(&json).unwrap();
    assert_eq!(back, SymbolKind::Struct);
}

#[test]
fn symbol_kind_round_trips_function_through_json() {
    let json = serde_json::to_string(&SymbolKind::Function).unwrap();
    let back: SymbolKind = serde_json::from_str(&json).unwrap();
    assert_eq!(back, SymbolKind::Function);
}

#[test]
fn symbol_kind_round_trips_trait_through_json() {
    let json = serde_json::to_string(&SymbolKind::Trait).unwrap();
    let back: SymbolKind = serde_json::from_str(&json).unwrap();
    assert_eq!(back, SymbolKind::Trait);
}

#[test]
fn symbol_kind_round_trips_enum_through_json() {
    let json = serde_json::to_string(&SymbolKind::Enum).unwrap();
    let back: SymbolKind = serde_json::from_str(&json).unwrap();
    assert_eq!(back, SymbolKind::Enum);
}

#[test]
fn symbol_kind_round_trips_module_through_json() {
    let json = serde_json::to_string(&SymbolKind::Module).unwrap();
    let back: SymbolKind = serde_json::from_str(&json).unwrap();
    assert_eq!(back, SymbolKind::Module);
}

#[test]
fn symbol_kind_round_trips_constant_through_json() {
    let json = serde_json::to_string(&SymbolKind::Constant).unwrap();
    let back: SymbolKind = serde_json::from_str(&json).unwrap();
    assert_eq!(back, SymbolKind::Constant);
}

#[test]
fn symbol_kind_round_trips_method_through_json() {
    let json = serde_json::to_string(&SymbolKind::Method).unwrap();
    let back: SymbolKind = serde_json::from_str(&json).unwrap();
    assert_eq!(back, SymbolKind::Method);
}

#[test]
fn symbol_kind_round_trips_field_through_json() {
    let json = serde_json::to_string(&SymbolKind::Field).unwrap();
    let back: SymbolKind = serde_json::from_str(&json).unwrap();
    assert_eq!(back, SymbolKind::Field);
}

#[test]
fn symbol_kind_round_trips_interface_through_json() {
    let json = serde_json::to_string(&SymbolKind::Interface).unwrap();
    let back: SymbolKind = serde_json::from_str(&json).unwrap();
    assert_eq!(back, SymbolKind::Interface);
}

#[test]
fn symbol_kind_round_trips_type_alias_through_json() {
    let json = serde_json::to_string(&SymbolKind::TypeAlias).unwrap();
    let back: SymbolKind = serde_json::from_str(&json).unwrap();
    assert_eq!(back, SymbolKind::TypeAlias);
}

#[test]
fn symbol_kind_round_trips_variable_through_json() {
    let json = serde_json::to_string(&SymbolKind::Variable).unwrap();
    let back: SymbolKind = serde_json::from_str(&json).unwrap();
    assert_eq!(back, SymbolKind::Variable);
}

#[test]
fn symbol_kind_round_trips_package_through_json() {
    let json = serde_json::to_string(&SymbolKind::Package).unwrap();
    let back: SymbolKind = serde_json::from_str(&json).unwrap();
    assert_eq!(back, SymbolKind::Package);
}

// ═══════════════════════════════════════════════════════════════════════════
// Error Display
// ═══════════════════════════════════════════════════════════════════════════

#[test]
fn symbol_kind_error_displays_unknown_kind_message_when_variant_constructed() {
    assert_eq!(
        format!("{}", SymbolKindError::UnknownKind("Foo".to_string())),
        "Unknown SymbolKind: Foo"
    );
}

// ═══════════════════════════════════════════════════════════════════════════
// Proptests
// ═══════════════════════════════════════════════════════════════════════════

proptest! {
    #[test]
    fn symbol_kind_serde_roundtrips_for_all_variants(kind in proptest::sample::select(&[
        SymbolKind::Struct,
        SymbolKind::Function,
        SymbolKind::Trait,
        SymbolKind::Enum,
        SymbolKind::Module,
        SymbolKind::Constant,
        SymbolKind::Method,
        SymbolKind::Field,
        SymbolKind::Interface,
        SymbolKind::TypeAlias,
        SymbolKind::Variable,
        SymbolKind::Package,
    ])) {
        let json = serde_json::to_string(&kind).unwrap();
        let back: SymbolKind = serde_json::from_str(&json).unwrap();
        assert_eq!(back, kind);
    }
}

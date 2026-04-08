#![allow(clippy::unwrap_used, clippy::expect_used)]
use super::*;
use proptest::prelude::*;

// ═══════════════════════════════════════════════════════════════════════════
// Accessors (BDD 3.7)
// ═══════════════════════════════════════════════════════════════════════════

#[test]
fn scip_symbol_id_as_str_returns_canonical_format_when_id_is_valid() {
    let id = ScipSymbolId::new("rust", "auth/AuthService", "login()").unwrap();
    assert_eq!(id.as_str(), "rust/auth/AuthService#login()");
}

#[test]
fn scip_symbol_id_scheme_returns_scheme_portion_when_id_is_valid() {
    let id = ScipSymbolId::new("rust", "auth/AuthService", "login()").unwrap();
    assert_eq!(id.scheme(), "rust");
}

#[test]
fn scip_symbol_id_module_path_returns_path_portion_when_id_is_valid() {
    let id = ScipSymbolId::new("rust", "auth/AuthService", "login()").unwrap();
    assert_eq!(id.module_path(), "auth/AuthService");
}

#[test]
fn scip_symbol_id_descriptor_returns_descriptor_portion_when_id_is_valid() {
    let id = ScipSymbolId::new("rust", "auth/AuthService", "login()").unwrap();
    assert_eq!(id.descriptor(), "login()");
}

#[test]
fn scip_symbol_id_into_string_returns_owned_string_equal_to_as_str_when_consumed() {
    let id = ScipSymbolId::new("rust", "auth/AuthService", "login()").unwrap();
    let s = id.into_string();
    assert_eq!(s, "rust/auth/AuthService#login()");
}

// ═══════════════════════════════════════════════════════════════════════════
// Trait Implementations (BDD 3.8)
// ═══════════════════════════════════════════════════════════════════════════

#[test]
fn scip_symbol_id_equality_holds_when_components_are_identical() {
    let id1 = ScipSymbolId::new("rust", "auth/AuthService", "login()").unwrap();
    let id2 = ScipSymbolId::new("rust", "auth/AuthService", "login()").unwrap();
    assert_eq!(id1, id2);
}

#[test]
fn scip_symbol_id_inequality_holds_when_schemes_differ() {
    let id1 = ScipSymbolId::new("rust", "auth/AuthService", "login()").unwrap();
    let id2 = ScipSymbolId::new("python", "auth/AuthService", "login()").unwrap();
    assert_ne!(id1, id2);
}

#[test]
fn scip_symbol_id_ordering_is_lexicographic_when_comparing_different_schemes() {
    let id_a = ScipSymbolId::new("python", "auth", "f").unwrap();
    let id_b = ScipSymbolId::new("rust", "auth", "f").unwrap();
    assert_eq!(id_a.cmp(&id_b), std::cmp::Ordering::Less);
}

#[test]
fn scip_symbol_id_display_outputs_canonical_string_when_formatted() {
    let id = ScipSymbolId::new("rust", "auth/AuthService", "login()").unwrap();
    assert_eq!(format!("{id}"), "rust/auth/AuthService#login()");
}

#[test]
fn scip_symbol_id_deref_returns_str_reference_when_dereferenced() {
    let id = ScipSymbolId::new("rust", "auth", "f").unwrap();
    let s: &str = &*id;
    assert_eq!(s, "rust/auth#f");
}

#[test]
fn scip_symbol_id_as_ref_returns_str_reference_when_called() {
    let id = ScipSymbolId::new("rust", "auth", "f").unwrap();
    let s: &str = id.as_ref();
    assert_eq!(s, "rust/auth#f");
}

#[test]
fn scip_symbol_id_borrow_returns_str_reference_when_borrowed() {
    let id = ScipSymbolId::new("rust", "auth", "f").unwrap();
    let s: &str = id.borrow();
    assert_eq!(s, "rust/auth#f");
}

// ═══════════════════════════════════════════════════════════════════════════
// JSON Round-Trip (BDD 3.9)
// ═══════════════════════════════════════════════════════════════════════════

#[test]
fn scip_symbol_id_round_trips_through_json_when_serialized_and_deserialized() {
    let id = ScipSymbolId::new("rust", "auth/AuthService", "login()").unwrap();
    let json = serde_json::to_string(&id).unwrap();
    let reconstructed: ScipSymbolId = serde_json::from_str(&json).unwrap();
    assert_eq!(reconstructed, id);
    assert_eq!(json, "\"rust/auth/AuthService#login()\"");
}

// ═══════════════════════════════════════════════════════════════════════════
// Error Display Messages (BDD 3.21)
// ═══════════════════════════════════════════════════════════════════════════

#[test]
fn scip_symbol_id_error_displays_empty_scheme_message_when_variant_constructed() {
    assert_eq!(
        format!("{}", ScipSymbolIdError::EmptyScheme),
        "SCIP scheme cannot be empty"
    );
}

#[test]
fn scip_symbol_id_error_displays_invalid_scheme_message_when_variant_constructed() {
    assert_eq!(
        format!("{}", ScipSymbolIdError::InvalidScheme("/".to_string())),
        "SCIP scheme contains invalid character: /"
    );
}

#[test]
fn scip_symbol_id_error_displays_empty_module_path_message_when_variant_constructed() {
    assert_eq!(
        format!("{}", ScipSymbolIdError::EmptyModulePath),
        "SCIP module path cannot be empty"
    );
}

#[test]
fn scip_symbol_id_error_displays_empty_module_segment_message_when_variant_constructed() {
    assert_eq!(
        format!("{}", ScipSymbolIdError::EmptyModuleSegment(5)),
        "SCIP module path contains empty segment at position 5"
    );
}

#[test]
fn scip_symbol_id_error_displays_leading_slash_message_when_variant_constructed() {
    assert_eq!(
        format!("{}", ScipSymbolIdError::LeadingSlash),
        "SCIP module path must not start with '/'"
    );
}

#[test]
fn scip_symbol_id_error_displays_trailing_slash_message_when_variant_constructed() {
    assert_eq!(
        format!("{}", ScipSymbolIdError::TrailingSlash),
        "SCIP module path must not end with '/'"
    );
}

#[test]
fn scip_symbol_id_error_displays_hash_in_module_path_message_when_variant_constructed() {
    assert_eq!(
        format!("{}", ScipSymbolIdError::HashInModulePath),
        "SCIP module path must not contain '#'"
    );
}

#[test]
fn scip_symbol_id_error_displays_empty_descriptor_message_when_variant_constructed() {
    assert_eq!(
        format!("{}", ScipSymbolIdError::EmptyDescriptor),
        "SCIP descriptor cannot be empty"
    );
}

#[test]
fn scip_symbol_id_error_displays_slash_in_descriptor_message_when_variant_constructed() {
    assert_eq!(
        format!("{}", ScipSymbolIdError::SlashInDescriptor),
        "SCIP descriptor must not contain '/'"
    );
}

#[test]
fn scip_symbol_id_error_displays_invalid_format_message_when_variant_constructed() {
    assert_eq!(
        format!("{}", ScipSymbolIdError::InvalidFormat("bad".to_string())),
        "Invalid SCIP symbol format: bad"
    );
}

// ═══════════════════════════════════════════════════════════════════════════
// Error Payload Data (behavior 93)
// ═══════════════════════════════════════════════════════════════════════════

#[test]
fn scip_symbol_id_error_preserves_position_in_empty_module_segment_variant() {
    let err = ScipSymbolIdError::EmptyModuleSegment(5);
    assert_eq!(err, ScipSymbolIdError::EmptyModuleSegment(5));
    if let ScipSymbolIdError::EmptyModuleSegment(pos) = err {
        assert_eq!(pos, 5);
    } else {
        panic!("Expected EmptyModuleSegment variant");
    }
}

// ═══════════════════════════════════════════════════════════════════════════
// Proptests
// ═══════════════════════════════════════════════════════════════════════════

fn arb_valid_scheme() -> impl Strategy<Value = String> {
    proptest::string::string_regex("[a-z]{1,10}").unwrap()
}

fn arb_valid_module_path() -> impl Strategy<Value = String> {
    proptest::string::string_regex("[a-zA-Z0-9_]+(/[a-zA-Z0-9_]+){0,5}").unwrap()
}

fn arb_valid_descriptor() -> impl Strategy<Value = String> {
    proptest::string::string_regex("[a-zA-Z0-9_.()]{1,30}").unwrap()
}

proptest! {
    #[test]
    fn scip_symbol_id_new_then_parse_roundtrips_for_valid_inputs(
        scheme in arb_valid_scheme(),
        module_path in arb_valid_module_path(),
        descriptor in arb_valid_descriptor(),
    ) {
        let id = ScipSymbolId::new(&scheme, &module_path, &descriptor).unwrap();
        let parsed = ScipSymbolId::parse(id.as_str()).unwrap();
        assert_eq!(parsed, id);
    }
}

proptest! {
    #[test]
    fn scip_symbol_id_as_str_contains_exactly_one_hash(
        scheme in arb_valid_scheme(),
        module_path in arb_valid_module_path(),
        descriptor in arb_valid_descriptor(),
    ) {
        let id = ScipSymbolId::new(&scheme, &module_path, &descriptor).unwrap();
        assert_eq!(id.as_str().matches('#').count(), 1);
    }
}

proptest! {
    #[test]
    fn scip_symbol_id_module_path_has_no_empty_segments(
        scheme in arb_valid_scheme(),
        module_path in arb_valid_module_path(),
        descriptor in arb_valid_descriptor(),
    ) {
        let id = ScipSymbolId::new(&scheme, &module_path, &descriptor).unwrap();
        if let Some(pre_hash) = id.as_str().split('#').next() {
            assert!(!pre_hash.contains("//"), "module path must not contain empty segments: {pre_hash}");
        }
    }
}

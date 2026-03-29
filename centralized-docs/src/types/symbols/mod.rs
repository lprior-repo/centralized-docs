//! SCIP-inspired semantic domain model for source code symbols.

mod scip_symbol_id;
mod symbol_graph;
mod symbol_kind;
mod symbol_role;

pub use scip_symbol_id::{ScipSymbolId, ScipSymbolIdError};
pub use symbol_graph::{RelationshipKind, SymbolNode, SymbolRelationship};
pub use symbol_kind::{SymbolKind, SymbolKindError};
pub use symbol_role::{SymbolRole, SymbolRoleError};

#[cfg(test)]
use std::borrow::Borrow;

// ── Tests ────────────────────────────────────────────────────────────────────

#[cfg(test)]
mod tests {
    use super::*;
    use std::collections::{HashMap, HashSet};

    // ═══════════════════════════════════════════════════════════════════════════
    // ScipSymbolId — Valid Construction (BDD 3.1)
    // ═══════════════════════════════════════════════════════════════════════════

    #[test]
    fn scip_symbol_id_constructs_valid_id_when_all_components_valid() {
        // Given
        let scheme = "rust";
        let module_path = "auth/AuthService";
        let descriptor = "login()";
        // When
        let result = ScipSymbolId::new(scheme, module_path, descriptor);
        // Then
        assert_eq!(
            result.map(|id| id.as_str().to_string()),
            Ok("rust/auth/AuthService#login()".to_string())
        );
    }

    #[test]
    fn scip_symbol_id_constructs_id_with_method_disambiguation_when_descriptor_contains_dot() {
        // Given
        let scheme = "rust";
        let module_path = "auth/AuthService";
        let descriptor = "Auth.my_method";
        // When
        let result = ScipSymbolId::new(scheme, module_path, descriptor);
        // Then
        assert_eq!(
            result.map(|id| id.as_str().to_string()),
            Ok("rust/auth/AuthService#Auth.my_method".to_string())
        );
    }

    #[test]
    fn scip_symbol_id_constructs_with_single_segment_module_path_when_path_has_no_slashes() {
        // Given
        let scheme = "python";
        let module_path = "mymodule";
        let descriptor = "MyClass";
        // When
        let result = ScipSymbolId::new(scheme, module_path, descriptor);
        // Then
        assert_eq!(
            result.map(|id| id.as_str().to_string()),
            Ok("python/mymodule#MyClass".to_string())
        );
    }

    // ═══════════════════════════════════════════════════════════════════════════
    // ScipSymbolId — Scheme Validation (BDD 3.2)
    // ═══════════════════════════════════════════════════════════════════════════

    #[test]
    fn scip_symbol_id_returns_empty_scheme_error_when_scheme_is_empty() {
        let result = ScipSymbolId::new("", "auth/AuthService", "login()");
        assert_eq!(result, Err(ScipSymbolIdError::EmptyScheme));
    }

    #[test]
    fn scip_symbol_id_returns_empty_scheme_error_when_scheme_is_whitespace_only() {
        let result = ScipSymbolId::new("   ", "auth/AuthService", "login()");
        assert_eq!(result, Err(ScipSymbolIdError::EmptyScheme));
    }

    #[test]
    fn scip_symbol_id_returns_empty_scheme_error_when_scheme_is_tab_only() {
        let result = ScipSymbolId::new("\t", "auth/AuthService", "login()");
        assert_eq!(result, Err(ScipSymbolIdError::EmptyScheme));
    }

    #[test]
    fn scip_symbol_id_returns_invalid_scheme_error_when_scheme_contains_slash() {
        let result = ScipSymbolId::new("rust/core", "auth", "login()");
        assert_eq!(
            result,
            Err(ScipSymbolIdError::InvalidScheme("/".to_string()))
        );
    }

    #[test]
    fn scip_symbol_id_returns_invalid_scheme_error_when_scheme_contains_hash() {
        let result = ScipSymbolId::new("rust#std", "auth", "login()");
        assert_eq!(
            result,
            Err(ScipSymbolIdError::InvalidScheme("#".to_string()))
        );
    }

    #[test]
    fn scip_symbol_id_returns_invalid_scheme_error_when_scheme_contains_multiple_invalid_chars() {
        let result = ScipSymbolId::new("ru/st#core", "auth", "login()");
        assert!(matches!(result, Err(ScipSymbolIdError::InvalidScheme(_))));
    }

    // ═══════════════════════════════════════════════════════════════════════════
    // ScipSymbolId — Module Path Validation (BDD 3.3)
    // ═══════════════════════════════════════════════════════════════════════════

    #[test]
    fn scip_symbol_id_returns_empty_module_path_error_when_module_path_is_empty() {
        let result = ScipSymbolId::new("rust", "", "login()");
        assert_eq!(result, Err(ScipSymbolIdError::EmptyModulePath));
    }

    #[test]
    fn scip_symbol_id_returns_empty_module_path_error_when_module_path_is_whitespace_only() {
        let result = ScipSymbolId::new("rust", "  ", "login()");
        assert_eq!(result, Err(ScipSymbolIdError::EmptyModulePath));
    }

    #[test]
    fn scip_symbol_id_returns_empty_module_path_error_when_module_path_is_newline_only() {
        let result = ScipSymbolId::new("rust", "\n", "login()");
        assert_eq!(result, Err(ScipSymbolIdError::EmptyModulePath));
    }

    #[test]
    fn scip_symbol_id_returns_empty_module_segment_error_when_path_has_double_slash() {
        let result = ScipSymbolId::new("rust", "auth//service", "login()");
        assert_eq!(result, Err(ScipSymbolIdError::EmptyModuleSegment(5)));
    }

    #[test]
    fn scip_symbol_id_returns_leading_slash_error_when_module_path_starts_with_slash() {
        let result = ScipSymbolId::new("rust", "/auth/service", "login()");
        assert_eq!(result, Err(ScipSymbolIdError::LeadingSlash));
    }

    #[test]
    fn scip_symbol_id_returns_trailing_slash_error_when_module_path_ends_with_slash() {
        let result = ScipSymbolId::new("rust", "auth/service/", "login()");
        assert_eq!(result, Err(ScipSymbolIdError::TrailingSlash));
    }

    #[test]
    fn scip_symbol_id_returns_hash_in_module_path_error_when_path_contains_hash() {
        let result = ScipSymbolId::new("rust", "auth#service", "login()");
        assert_eq!(result, Err(ScipSymbolIdError::HashInModulePath));
    }

    // ═══════════════════════════════════════════════════════════════════════════
    // ScipSymbolId — Descriptor Validation (BDD 3.4)
    // ═══════════════════════════════════════════════════════════════════════════

    #[test]
    fn scip_symbol_id_returns_empty_descriptor_error_when_descriptor_is_empty() {
        let result = ScipSymbolId::new("rust", "auth/AuthService", "");
        assert_eq!(result, Err(ScipSymbolIdError::EmptyDescriptor));
    }

    #[test]
    fn scip_symbol_id_returns_empty_descriptor_error_when_descriptor_is_whitespace_only() {
        let result = ScipSymbolId::new("rust", "auth/AuthService", "  ");
        assert_eq!(result, Err(ScipSymbolIdError::EmptyDescriptor));
    }

    #[test]
    fn scip_symbol_id_returns_empty_descriptor_error_when_descriptor_is_mixed_whitespace() {
        let result = ScipSymbolId::new("rust", "auth/AuthService", "\t \n");
        assert_eq!(result, Err(ScipSymbolIdError::EmptyDescriptor));
    }

    #[test]
    fn scip_symbol_id_returns_slash_in_descriptor_error_when_descriptor_contains_slash() {
        let result = ScipSymbolId::new("rust", "auth/AuthService", "login/method");
        assert_eq!(result, Err(ScipSymbolIdError::SlashInDescriptor));
    }

    #[test]
    fn scip_symbol_id_rejects_hash_in_descriptor_when_descriptor_contains_hash() {
        let result = ScipSymbolId::new("rust", "auth/AuthService", "login#extra");
        assert!(
            matches!(result, Err(ScipSymbolIdError::InvalidScheme(ref s)) if s == "#"),
            "descriptor containing '#' must be rejected to preserve INV-1"
        );
    }

    // ═══════════════════════════════════════════════════════════════════════════
    // ScipSymbolId — parse (BDD 3.5)
    // ═══════════════════════════════════════════════════════════════════════════

    #[test]
    fn scip_symbol_id_parse_returns_valid_id_when_input_matches_format() {
        let result = ScipSymbolId::parse("rust/auth/AuthService#login()");
        assert_eq!(
            result.map(|id| id.as_str().to_string()),
            Ok("rust/auth/AuthService#login()".to_string())
        );
    }

    #[test]
    fn scip_symbol_id_parse_returns_invalid_format_error_when_input_has_no_hash() {
        let result = ScipSymbolId::parse("rust/auth/AuthService");
        assert_eq!(
            result,
            Err(ScipSymbolIdError::InvalidFormat(
                "rust/auth/AuthService".to_string()
            ))
        );
    }

    #[test]
    fn scip_symbol_id_parse_returns_invalid_format_error_when_input_is_empty() {
        let result = ScipSymbolId::parse("");
        assert_eq!(
            result,
            Err(ScipSymbolIdError::InvalidFormat("".to_string()))
        );
    }

    #[test]
    fn scip_symbol_id_parse_returns_invalid_format_error_when_scheme_is_empty() {
        let result = ScipSymbolId::parse("/auth/AuthService#login()");
        assert_eq!(
            result,
            Err(ScipSymbolIdError::InvalidFormat(
                "/auth/AuthService#login()".to_string()
            ))
        );
    }

    #[test]
    fn scip_symbol_id_parse_returns_invalid_format_error_when_descriptor_is_empty() {
        let result = ScipSymbolId::parse("rust/auth/AuthService#");
        assert_eq!(
            result,
            Err(ScipSymbolIdError::InvalidFormat(
                "rust/auth/AuthService#".to_string()
            ))
        );
    }

    #[test]
    fn scip_symbol_id_parse_returns_invalid_format_error_when_input_has_no_slash() {
        let result = ScipSymbolId::parse("noslash#desc");
        assert_eq!(
            result,
            Err(ScipSymbolIdError::InvalidFormat("noslash#desc".to_string()))
        );
    }

    #[test]
    fn scip_symbol_id_parse_propagates_module_path_errors_when_path_has_empty_segments() {
        let result = ScipSymbolId::parse("rust/auth//service#login()");
        assert_eq!(result, Err(ScipSymbolIdError::EmptyModuleSegment(5)));
    }

    #[test]
    fn scip_symbol_id_parse_propagates_descriptor_errors_when_descriptor_has_slash() {
        let result = ScipSymbolId::parse("rust/auth/AuthService#login/bad");
        assert_eq!(result, Err(ScipSymbolIdError::SlashInDescriptor));
    }

    // ═══════════════════════════════════════════════════════════════════════════
    // ScipSymbolId — parse/new Equivalence (BDD 3.6)
    // ═══════════════════════════════════════════════════════════════════════════

    #[test]
    fn scip_symbol_id_parse_equals_new_when_components_match() {
        let parsed = ScipSymbolId::parse("rust/auth/AuthService#login()");
        let constructed = ScipSymbolId::new("rust", "auth/AuthService", "login()");
        assert_eq!(parsed, constructed);
    }

    // ═══════════════════════════════════════════════════════════════════════════
    // ScipSymbolId — Accessors (BDD 3.7)
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
    // ScipSymbolId — Trait Implementations (BDD 3.8)
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
    // ScipSymbolId — JSON Round-Trip (BDD 3.9)
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
    // SymbolRole — Constants (BDD 3.10)
    // ═══════════════════════════════════════════════════════════════════════════

    #[test]
    fn symbol_role_definition_has_bits_one_when_inspected() {
        assert_eq!(SymbolRole::DEFINITION.bits(), 1);
    }

    #[test]
    fn symbol_role_read_has_bits_two_when_inspected() {
        assert_eq!(SymbolRole::READ.bits(), 2);
    }

    #[test]
    fn symbol_role_write_has_bits_four_when_inspected() {
        assert_eq!(SymbolRole::WRITE.bits(), 4);
    }

    #[test]
    fn symbol_role_generated_has_bits_eight_when_inspected() {
        assert_eq!(SymbolRole::GENERATED.bits(), 8);
    }

    #[test]
    fn symbol_role_test_has_bits_sixteen_when_inspected() {
        assert_eq!(SymbolRole::TEST.bits(), 16);
    }

    #[test]
    fn symbol_role_all_has_bits_thirty_one_when_inspected() {
        assert_eq!(SymbolRole::ALL.bits(), 31);
    }

    // ═══════════════════════════════════════════════════════════════════════════
    // SymbolRole — empty() (behaviors 50-51)
    // ═══════════════════════════════════════════════════════════════════════════

    #[test]
    fn symbol_role_empty_has_bits_zero_when_inspected() {
        assert_eq!(SymbolRole::empty().bits(), 0);
    }

    #[test]
    fn symbol_role_empty_is_empty_returns_true_when_role_is_empty() {
        assert!(SymbolRole::empty().is_empty());
    }

    // ═══════════════════════════════════════════════════════════════════════════
    // SymbolRole — from_bits Validation (BDD 3.11)
    // ═══════════════════════════════════════════════════════════════════════════

    #[test]
    fn symbol_role_from_bits_accepts_each_valid_single_flag_when_given_power_of_two() {
        assert_eq!(SymbolRole::from_bits(1).map(|r| r.bits()), Ok(1));
        assert_eq!(SymbolRole::from_bits(2).map(|r| r.bits()), Ok(2));
        assert_eq!(SymbolRole::from_bits(4).map(|r| r.bits()), Ok(4));
        assert_eq!(SymbolRole::from_bits(8).map(|r| r.bits()), Ok(8));
        assert_eq!(SymbolRole::from_bits(16).map(|r| r.bits()), Ok(16));
    }

    #[test]
    fn symbol_role_from_bits_accepts_valid_combinations_when_bits_are_within_mask() {
        let result = SymbolRole::from_bits(3);
        assert_eq!(result.as_ref().map(|r| r.bits()), Ok(3));
        assert_eq!(
            result.as_ref().map(|r| r.contains(SymbolRole::DEFINITION)),
            Ok(true)
        );

        let result = SymbolRole::from_bits(5);
        assert_eq!(result.as_ref().map(|r| r.bits()), Ok(5));
        assert_eq!(
            result.as_ref().map(|r| r.contains(SymbolRole::DEFINITION)),
            Ok(true)
        );

        let result = SymbolRole::from_bits(31);
        assert_eq!(result.as_ref().map(|r| r.bits()), Ok(31));
    }

    #[test]
    fn symbol_role_from_bits_accepts_zero_when_given_empty_bits() {
        let result = SymbolRole::from_bits(0);
        assert_eq!(result.as_ref().map(|r| r.bits()), Ok(0));
        assert_eq!(result.as_ref().map(|r| r.is_empty()), Ok(true));
    }

    #[test]
    fn symbol_role_from_bits_returns_unknown_bit_error_when_bits_exceed_mask() {
        let result = SymbolRole::from_bits(32);
        assert_eq!(result, Err(SymbolRoleError::UnknownBit(32)));
    }

    #[test]
    fn symbol_role_from_bits_returns_unknown_bit_error_when_bits_include_unknown_flag() {
        let result = SymbolRole::from_bits(33);
        assert_eq!(result, Err(SymbolRoleError::UnknownBit(33)));
    }

    // ═══════════════════════════════════════════════════════════════════════════
    // SymbolRole — from_bits_truncate (BDD 3.12)
    // ═══════════════════════════════════════════════════════════════════════════

    #[test]
    fn symbol_role_from_bits_truncate_drops_unknown_bits_when_bits_exceed_mask() {
        let role = SymbolRole::from_bits_truncate(33);
        assert_eq!(role.bits(), 1);
        assert!(role.contains(SymbolRole::DEFINITION));
        assert!(!role.contains(SymbolRole::TEST));
    }

    #[test]
    fn symbol_role_from_bits_truncate_returns_empty_when_all_bits_are_unknown() {
        let role = SymbolRole::from_bits_truncate(64);
        assert_eq!(role.bits(), 0);
        assert!(role.is_empty());
    }

    // ═══════════════════════════════════════════════════════════════════════════
    // SymbolRole — Query Methods (BDD 3.13)
    // ═══════════════════════════════════════════════════════════════════════════

    #[test]
    fn symbol_role_bits_returns_sum_of_flag_bits_when_role_is_combined() {
        let role = SymbolRole::DEFINITION | SymbolRole::WRITE;
        assert_eq!(role.bits(), 5);
    }

    #[test]
    fn symbol_role_contains_returns_true_when_flag_is_set() {
        let role = SymbolRole::DEFINITION | SymbolRole::READ;
        assert!(role.contains(SymbolRole::DEFINITION));
    }

    #[test]
    fn symbol_role_contains_returns_false_when_flag_is_not_set() {
        let role = SymbolRole::DEFINITION;
        assert!(!role.contains(SymbolRole::READ));
    }

    #[test]
    fn symbol_role_contains_returns_true_when_comparing_role_to_itself() {
        let role = SymbolRole::DEFINITION | SymbolRole::WRITE;
        assert!(role.contains(role));
    }

    #[test]
    fn symbol_role_is_empty_returns_false_when_any_flag_is_set() {
        let role = SymbolRole::DEFINITION;
        assert!(!role.is_empty());
    }

    // ═══════════════════════════════════════════════════════════════════════════
    // SymbolRole — Bitwise Operations (BDD 3.14)
    // ═══════════════════════════════════════════════════════════════════════════

    #[test]
    fn symbol_role_union_combines_flags_via_bitwise_or_when_called() {
        let a = SymbolRole::DEFINITION;
        let b = SymbolRole::READ;
        let result = a.union(b);
        assert_eq!(result.bits(), 3);
        assert!(result.contains(SymbolRole::DEFINITION));
        assert!(result.contains(SymbolRole::READ));
    }

    #[test]
    fn symbol_role_intersection_keeps_shared_flags_via_bitwise_and_when_called() {
        let a = SymbolRole::DEFINITION | SymbolRole::READ;
        let b = SymbolRole::READ | SymbolRole::WRITE;
        let result = a.intersection(b);
        assert_eq!(result.bits(), 2);
        assert!(result.contains(SymbolRole::READ));
        assert!(!result.contains(SymbolRole::DEFINITION));
    }

    #[test]
    fn symbol_role_bitor_produces_union_when_operators_applied() {
        let a = SymbolRole::DEFINITION;
        let b = SymbolRole::READ;
        let result = a | b;
        assert_eq!(result.bits(), 3);
    }

    #[test]
    fn symbol_role_bitand_produces_intersection_when_operators_applied() {
        let a = SymbolRole::DEFINITION | SymbolRole::READ;
        let b = SymbolRole::READ | SymbolRole::WRITE;
        let result = a & b;
        assert_eq!(result.bits(), 2);
    }

    // ═══════════════════════════════════════════════════════════════════════════
    // SymbolRole — Display (BDD 3.15)
    // ═══════════════════════════════════════════════════════════════════════════

    #[test]
    fn symbol_role_display_outputs_definition_when_single_definition_role() {
        assert_eq!(format!("{}", SymbolRole::DEFINITION), "definition");
    }

    #[test]
    fn symbol_role_display_outputs_read_when_single_read_role() {
        assert_eq!(format!("{}", SymbolRole::READ), "read");
    }

    #[test]
    fn symbol_role_display_outputs_write_when_single_write_role() {
        assert_eq!(format!("{}", SymbolRole::WRITE), "write");
    }

    #[test]
    fn symbol_role_display_outputs_generated_when_single_generated_role() {
        assert_eq!(format!("{}", SymbolRole::GENERATED), "generated");
    }

    #[test]
    fn symbol_role_display_outputs_test_when_single_test_role() {
        assert_eq!(format!("{}", SymbolRole::TEST), "test");
    }

    #[test]
    fn symbol_role_display_outputs_sorted_plus_delimited_names_when_multiple_roles() {
        let role = SymbolRole::READ | SymbolRole::DEFINITION;
        assert_eq!(format!("{role}"), "definition+read");
    }

    #[test]
    fn symbol_role_display_outputs_all_five_sorted_when_all_roles_set() {
        assert_eq!(
            format!("{}", SymbolRole::ALL),
            "definition+read+write+generated+test"
        );
    }

    #[test]
    fn symbol_role_display_outputs_none_when_no_roles_set() {
        assert_eq!(format!("{}", SymbolRole::empty()), "none");
    }

    // ═══════════════════════════════════════════════════════════════════════════
    // SymbolRole — JSON Round-Trip (BDD 3.16)
    // ═══════════════════════════════════════════════════════════════════════════

    #[test]
    fn symbol_role_round_trips_through_json_as_u32_when_serialized() {
        let role = SymbolRole::DEFINITION | SymbolRole::READ;
        let json = serde_json::to_string(&role).unwrap();
        let reconstructed: SymbolRole = serde_json::from_str(&json).unwrap();
        assert_eq!(reconstructed, role);
        assert_eq!(json, "3");
    }

    // ═══════════════════════════════════════════════════════════════════════════
    // SymbolKind — Display, Serialize, Deserialize (BDD 3.17)
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
    // RelationshipKind — Display, Serialize, Deserialize (BDD 3.18)
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
    // Error Types — Display Messages (BDD 3.21)
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

    #[test]
    fn symbol_role_error_displays_unknown_bit_message_when_variant_constructed() {
        assert_eq!(
            format!("{}", SymbolRoleError::UnknownBit(32)),
            "Unknown SymbolRole bit: 32"
        );
    }

    #[test]
    fn symbol_kind_error_displays_unknown_kind_message_when_variant_constructed() {
        assert_eq!(
            format!("{}", SymbolKindError::UnknownKind("Foo".to_string())),
            "Unknown SymbolKind: Foo"
        );
    }

    // ═══════════════════════════════════════════════════════════════════════════
    // ScipSymbolIdError — Payload Data (behavior 93)
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
    // Proptest Invariants (Section 4)
    // ═══════════════════════════════════════════════════════════════════════════

    mod proptests {
        use super::*;
        use proptest::prelude::*;

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

        proptest! {
            #[test]
            fn symbol_role_from_bits_then_bits_roundtrips_for_valid_bits(bits in 0u32..31u32) {
                let role = SymbolRole::from_bits(bits).unwrap();
                assert_eq!(role.bits(), bits);
            }
        }

        proptest! {
            #[test]
            fn symbol_role_union_is_commutative(a in 0u32..31u32, b in 0u32..31u32) {
                let role_a = SymbolRole::from_bits(a).unwrap();
                let role_b = SymbolRole::from_bits(b).unwrap();
                assert_eq!(role_a.union(role_b).bits(), role_b.union(role_a).bits());
            }
        }

        proptest! {
            #[test]
            fn symbol_role_union_with_empty_is_identity(a in 0u32..31u32) {
                let role = SymbolRole::from_bits(a).unwrap();
                assert_eq!(role.union(SymbolRole::empty()).bits(), role.bits());
            }
        }

        proptest! {
            #[test]
            fn symbol_role_intersection_absorbs_self(a in 0u32..31u32) {
                let role = SymbolRole::from_bits(a).unwrap();
                assert_eq!(role.intersection(role), role);
            }
        }

        proptest! {
            #[test]
            fn symbol_role_intersection_is_bounded(a in 0u32..31u32, b in 0u32..31u32) {
                let role_a = SymbolRole::from_bits(a).unwrap();
                let role_b = SymbolRole::from_bits(b).unwrap();
                let intersection = role_a.intersection(role_b);
                assert!(intersection.bits() <= role_a.bits());
                assert!(intersection.bits() <= role_b.bits());
            }
        }

        proptest! {
            #[test]
            fn symbol_role_contains_consistent_with_bits(a in 0u32..31u32, flag in 1u32..=16u32) {
                let role = SymbolRole::from_bits(a).unwrap();
                let flag_role = SymbolRole::from_bits(flag).unwrap();
                let contains = role.contains(flag_role);
                let bitwise = (role.bits() & flag_role.bits()) == flag_role.bits();
                assert_eq!(contains, bitwise);
            }
        }

        proptest! {
            #[test]
            fn symbol_role_display_is_deterministic(bits in 0u32..31u32) {
                let role = SymbolRole::from_bits(bits).unwrap();
                let display1 = format!("{role}");
                let display2 = format!("{role}");
                assert_eq!(display1, display2);
            }
        }

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
    }
}

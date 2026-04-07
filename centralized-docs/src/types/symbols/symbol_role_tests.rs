#![allow(clippy::unwrap_used, clippy::expect_used)]
use super::*;
use proptest::prelude::*;

// ═══════════════════════════════════════════════════════════════════════════
// Constants (BDD 3.10)
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
// empty() (behaviors 50-51)
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
// from_bits Validation (BDD 3.11)
// ═══════════════════════════════════════════════════════════════════════════

#[test]
fn symbol_role_from_bits_accepts_each_valid_single_flag_when_given_power_of_two() {
    assert_eq!(SymbolRole::from_bits(1).map(super::SymbolRole::bits), Ok(1));
    assert_eq!(SymbolRole::from_bits(2).map(super::SymbolRole::bits), Ok(2));
    assert_eq!(SymbolRole::from_bits(4).map(super::SymbolRole::bits), Ok(4));
    assert_eq!(SymbolRole::from_bits(8).map(super::SymbolRole::bits), Ok(8));
    assert_eq!(
        SymbolRole::from_bits(16).map(super::SymbolRole::bits),
        Ok(16)
    );
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
// from_bits_truncate (BDD 3.12)
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
// Query Methods (BDD 3.13)
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
// Bitwise Operations (BDD 3.14)
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
// Display (BDD 3.15)
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
// JSON Round-Trip (BDD 3.16)
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
// Error Display
// ═══════════════════════════════════════════════════════════════════════════

#[test]
fn symbol_role_error_displays_unknown_bit_message_when_variant_constructed() {
    assert_eq!(
        format!("{}", SymbolRoleError::UnknownBit(32)),
        "Unknown SymbolRole bit: 32"
    );
}

// ═══════════════════════════════════════════════════════════════════════════
// Proptests
// ═══════════════════════════════════════════════════════════════════════════

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

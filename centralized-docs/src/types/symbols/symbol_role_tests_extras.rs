#![allow(clippy::unwrap_used, clippy::expect_used)]
use super::*;
use proptest::prelude::*;

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

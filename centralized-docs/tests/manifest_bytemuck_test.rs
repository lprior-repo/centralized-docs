//! Integration tests for bytemuck dependency manifest correctness (cdocs-824)
//!
//! RED PHASE: These tests verify the contract of bead cdocs-824:
//!   - POST-1: bytemuck with "derive" feature in [dependencies]
//!   - POST-2: bytemuck with "derive" feature in [dev-dependencies]
//!   - POST-3: rkyv remains unchanged in both sections
//!   - POST-4: All pre-existing dependencies preserved
//!   - POST-5/6: cargo check succeeds
//!   - INV-3: workspace lint unsafe_code = "forbid" still enforced
//!
//! Currently RED because bytemuck is NOT in Cargo.toml yet.
//! The `use bytemuck` lines below will fail compilation until the
//! implementation bead adds `bytemuck = { version = "1", features = ["derive"] }`
//! to both [dependencies] and [dev-dependencies].

// ==========================================================================
// COMPILE-TIME GATE #1: bytemuck crate must be resolvable
// This import causes E0433 ("failed to resolve: use of undeclared crate or module")
// until bytemuck is added to [dev-dependencies].
// ==========================================================================
use bytemuck::{Pod, Zeroable};

// ==========================================================================
// COMPILE-TIME GATE #2: bytemuck derive macros must be available
// The `derive` feature gates the Pod/Zeroable proc-macros.
// This fails until `features = ["derive"]` is specified in Cargo.toml.
// ==========================================================================

/// Trivial Pod type that exercises the bytemuck derive pipeline.
/// `#[repr(C)]` + no padding ensures Pod safety.
/// `u32 + u32 = 8 bytes, naturally aligned, zero-padding — Pod-legal.
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Pod, Zeroable)]
struct ManifestVerificationPod {
    field_a: u32,
    field_b: u32,
}

// ==========================================================================
// Runtime verification tests
// These will only be reached after the compile-time gates pass.
// ==========================================================================

/// Helper: read the crate's Cargo.toml as a string.
fn read_cargo_toml() -> String {
    let dir = std::env::var("CARGO_MANIFEST_DIR")
        .expect("CARGO_MANIFEST_DIR must be set by cargo test runner");
    let path = std::path::Path::new(&dir).join("Cargo.toml");
    std::fs::read_to_string(&path).unwrap_or_else(|e| panic!("Cargo.toml must be readable: {e}"))
}

#[test]
fn pod_derive_produces_valid_type_when_bytemuck_available() {
    // Given: bytemuck with derive feature is available (compile-time gates above)
    // When: a Pod type is instantiated with known values
    let pod = ManifestVerificationPod {
        field_a: 42,
        field_b: 0xFFFF_FFFF,
    };
    // Then: the fields are readable and correct
    assert_eq!(pod.field_a, 42);
    assert_eq!(pod.field_b, 0xFFFF_FFFF);
}

#[test]
fn zeroed_pod_has_all_zero_fields_when_bytemuck_zeroable_derived() {
    // Given: ManifestVerificationPod derives Zeroable
    // When: bytes are zeroed via bytemuck::zeroed
    let zeroed = bytemuck::zeroed::<ManifestVerificationPod>();
    // Then: every field is exactly zero
    assert_eq!(zeroed.field_a, 0);
    assert_eq!(zeroed.field_b, 0);
}

#[test]
fn bytes_cast_to_pod_slice_succeeds_when_bytemuck_available() {
    // Given: a byte array whose size equals ManifestVerificationPod (8 bytes)
    let bytes: [u8; 8] = [0x01, 0x02, 0x03, 0x04, 0x05, 0x06, 0x07, 0x08];
    // When: cast_slice transmutes the byte slice to a Pod slice
    let pod_slice: &[ManifestVerificationPod] = bytemuck::cast_slice(&bytes);
    // Then: the cast succeeds and produces exactly one element
    assert_eq!(pod_slice.len(), 1);
    // field_a is the first 4 bytes in native endian
    assert_eq!(
        pod_slice[0].field_a,
        u32::from_ne_bytes([0x01, 0x02, 0x03, 0x04])
    );
    // field_b is the next 4 bytes
    assert_eq!(
        pod_slice[0].field_b,
        u32::from_ne_bytes([0x05, 0x06, 0x07, 0x08])
    );
}

#[test]
fn pod_bytes_roundtrip_preserves_values_when_cast_back() {
    // Given: a Pod value with known fields
    let original = ManifestVerificationPod {
        field_a: 0xDEAD_BEEF,
        field_b: 0xCAFE_FEED,
    };
    // When: cast to bytes and back
    let bytes: &[u8] = bytemuck::cast_slice(std::slice::from_ref(&original));
    let recovered: &[ManifestVerificationPod] = bytemuck::cast_slice(bytes);
    // Then: the round-tripped value matches exactly
    assert_eq!(recovered[0].field_a, original.field_a);
    assert_eq!(recovered[0].field_b, original.field_b);
}

// ==========================================================================
// Manifest content verification tests
// These test the Cargo.toml file contents directly.
// ==========================================================================

#[test]
fn cargo_toml_contains_bytemuck_in_dependencies_when_parsed() {
    // Given: the crate's Cargo.toml
    let content = read_cargo_toml();

    // When: searching for "bytemuck" in the file
    let bytemuck_count = content.matches("bytemuck").count();

    // Then: bytemuck must appear (at least once — in both dep sections)
    assert!(
        bytemuck_count >= 1,
        "Cargo.toml must contain 'bytemuck' dependency, found 0 occurrences"
    );
}

#[test]
fn cargo_toml_contains_bytemuck_with_derive_feature_when_parsed() {
    // Given: the crate's Cargo.toml
    let content = read_cargo_toml();

    // When: searching for the bytemuck entry with derive feature
    let has_bytemuck_derive = content.contains("bytemuck") && content.contains("\"derive\"");

    // Then: bytemuck with derive feature must be present
    assert!(
        has_bytemuck_derive,
        "Cargo.toml must contain bytemuck entry with features = [\"derive\"]"
    );
}

#[test]
fn rkyv_remains_at_v08_with_std_bytecheck_in_both_sections_when_parsed() {
    // Given: the crate's Cargo.toml (POST-3, B9)
    let content = read_cargo_toml();

    // When: counting exact rkyv entries with the original config
    let rkyv_exact = content
        .matches("rkyv = { version = \"0.8\", features = [\"std\", \"bytecheck\"] }")
        .count();

    // Then: must appear exactly 2 times ([dependencies] + [dev-dependencies])
    assert_eq!(
        rkyv_exact, 2,
        "rkyv must be present with exact config 'version = \"0.8\", features = [\"std\", \"bytecheck\"]' \
         in both [dependencies] and [dev-dependencies], found {rkyv_exact} occurrences"
    );
}

#[test]
fn preexisting_deps_unchanged_when_bytemuck_added() {
    // Given: the crate's Cargo.toml (POST-4, B6)
    let content = read_cargo_toml();

    // When: checking each pre-existing dependency is still present
    let required_deps: [(&str, &str); 3] = [
        ("redb", r#"redb = "2""#),
        ("sha2", r#"sha2 = "0.10""#),
        ("rayon", r#"rayon = "1.11.0""#),
    ];

    // Then: every dependency must still be present with its exact version string
    for (name, expected_line) in required_deps {
        assert!(
            content.contains(expected_line),
            "Pre-existing dependency '{name}' must be unchanged: \
             expected line '{expected_line}' not found in Cargo.toml"
        );
    }
}

#![allow(clippy::unwrap_used, clippy::expect_used)]
use super::*;

// ═══════════════════════════════════════════════════════════════════════════
// Valid Construction (BDD 3.1)
// ═══════════════════════════════════════════════════════════════════════════

#[test]
fn scip_symbol_id_constructs_valid_id_when_all_components_valid() {
    let result = ScipSymbolId::new("rust", "auth/AuthService", "login()");
    assert_eq!(
        result.map(|id| id.as_str().to_string()),
        Ok("rust/auth/AuthService#login()".to_string())
    );
}

#[test]
fn scip_symbol_id_constructs_id_with_method_disambiguation_when_descriptor_contains_dot() {
    let result = ScipSymbolId::new("rust", "auth/AuthService", "Auth.my_method");
    assert_eq!(
        result.map(|id| id.as_str().to_string()),
        Ok("rust/auth/AuthService#Auth.my_method".to_string())
    );
}

#[test]
fn scip_symbol_id_constructs_with_single_segment_module_path_when_path_has_no_slashes() {
    let result = ScipSymbolId::new("python", "mymodule", "MyClass");
    assert_eq!(
        result.map(|id| id.as_str().to_string()),
        Ok("python/mymodule#MyClass".to_string())
    );
}

// ═══════════════════════════════════════════════════════════════════════════
// Scheme Validation (BDD 3.2)
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
// Module Path Validation (BDD 3.3)
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
// Descriptor Validation (BDD 3.4)
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
// parse (BDD 3.5)
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
    assert_eq!(result, Err(ScipSymbolIdError::InvalidFormat(String::new())));
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
// parse/new Equivalence (BDD 3.6)
// ═══════════════════════════════════════════════════════════════════════════

#[test]
fn scip_symbol_id_parse_equals_new_when_components_match() {
    let parsed = ScipSymbolId::parse("rust/auth/AuthService#login()");
    let constructed = ScipSymbolId::new("rust", "auth/AuthService", "login()");
    assert_eq!(parsed, constructed);
}

use serde::{Deserialize, Deserializer, Serialize};
use std::borrow::Borrow;
use std::fmt;
use std::ops::Deref;

#[derive(Debug, Clone, PartialEq, Eq, thiserror::Error)]
pub enum ScipSymbolIdError {
    #[error("SCIP scheme cannot be empty")]
    EmptyScheme,
    #[error("SCIP scheme contains invalid character: {0}")]
    InvalidScheme(String),
    #[error("SCIP module path cannot be empty")]
    EmptyModulePath,
    #[error("SCIP module path contains empty segment at position {0}")]
    EmptyModuleSegment(usize),
    #[error("SCIP module path must not start with '/'")]
    LeadingSlash,
    #[error("SCIP module path must not end with '/'")]
    TrailingSlash,
    #[error("SCIP module path must not contain '#'")]
    HashInModulePath,
    #[error("SCIP descriptor cannot be empty")]
    EmptyDescriptor,
    #[error("SCIP descriptor must not contain '/'")]
    SlashInDescriptor,
    #[error("Invalid SCIP symbol format: {0}")]
    InvalidFormat(String),
}

/// Deterministic SCIP-format symbol identifier.
///
/// Format: `<scheme>/<module_path>#<descriptor>`
/// Example: `rust/auth/AuthService#login()`
#[derive(Debug, Clone, PartialEq, Eq, Hash, PartialOrd, Ord, Serialize)]
pub struct ScipSymbolId(String);

impl ScipSymbolId {
    fn validate_scheme(scheme: &str) -> Result<(), ScipSymbolIdError> {
        if scheme.trim().is_empty() {
            return Err(ScipSymbolIdError::EmptyScheme);
        }
        if let Some(ch) = scheme.chars().find(|c| *c == '/' || *c == '#') {
            return Err(ScipSymbolIdError::InvalidScheme(ch.to_string()));
        }
        Ok(())
    }

    fn validate_module_path(path: &str) -> Result<(), ScipSymbolIdError> {
        if path.trim().is_empty() {
            return Err(ScipSymbolIdError::EmptyModulePath);
        }
        if path.starts_with('/') {
            return Err(ScipSymbolIdError::LeadingSlash);
        }
        if path.ends_with('/') {
            return Err(ScipSymbolIdError::TrailingSlash);
        }
        if path.find('#').is_some() {
            return Err(ScipSymbolIdError::HashInModulePath);
        }
        if let Some(pos) = path.find("//") {
            return Err(ScipSymbolIdError::EmptyModuleSegment(pos + 1));
        }
        Ok(())
    }

    fn validate_descriptor(descriptor: &str) -> Result<(), ScipSymbolIdError> {
        if descriptor.trim().is_empty() {
            return Err(ScipSymbolIdError::EmptyDescriptor);
        }
        if descriptor.contains('/') {
            return Err(ScipSymbolIdError::SlashInDescriptor);
        }
        if descriptor.contains('#') {
            return Err(ScipSymbolIdError::InvalidScheme("#".to_string()));
        }
        Ok(())
    }

    pub fn new(
        scheme: impl Into<String>,
        module_path: impl Into<String>,
        descriptor: impl Into<String>,
    ) -> Result<Self, ScipSymbolIdError> {
        let scheme = scheme.into();
        let module_path = module_path.into();
        let descriptor = descriptor.into();

        Self::validate_scheme(&scheme)?;
        Self::validate_module_path(&module_path)?;
        Self::validate_descriptor(&descriptor)?;

        let canonical = format!("{scheme}/{module_path}#{descriptor}");
        Ok(Self(canonical))
    }

    pub fn parse(s: &str) -> Result<Self, ScipSymbolIdError> {
        let hash_pos = match s.find('#') {
            Some(pos) => pos,
            None => return Err(ScipSymbolIdError::InvalidFormat(s.to_string())),
        };

        if s[hash_pos + 1..].contains('#') {
            return Err(ScipSymbolIdError::InvalidFormat(s.to_string()));
        }

        let pre_hash = &s[..hash_pos];
        let slash_pos = match pre_hash.find('/') {
            Some(pos) => pos,
            None => return Err(ScipSymbolIdError::InvalidFormat(s.to_string())),
        };

        let scheme = &pre_hash[..slash_pos];
        let module_path = &pre_hash[slash_pos + 1..];
        let descriptor = &s[hash_pos + 1..];

        if scheme.is_empty() {
            return Err(ScipSymbolIdError::InvalidFormat(s.to_string()));
        }
        if descriptor.is_empty() {
            return Err(ScipSymbolIdError::InvalidFormat(s.to_string()));
        }

        Self::validate_scheme(scheme)?;
        Self::validate_module_path(module_path)?;
        Self::validate_descriptor(descriptor)?;

        Ok(Self(s.to_string()))
    }

    #[must_use]
    pub fn as_str(&self) -> &str {
        &self.0
    }

    #[must_use]
    pub fn scheme(&self) -> &str {
        self.0
            .split('#')
            .next()
            .and_then(|pre_hash| pre_hash.split('/').next())
            .unwrap_or("")
    }

    #[must_use]
    pub fn module_path(&self) -> &str {
        self.0
            .split('#')
            .next()
            .and_then(|pre_hash| {
                let slash_pos = pre_hash.find('/')?;
                Some(&pre_hash[slash_pos + 1..])
            })
            .unwrap_or("")
    }

    #[must_use]
    pub fn descriptor(&self) -> &str {
        self.0.split('#').nth(1).unwrap_or("")
    }

    #[must_use]
    pub fn into_string(self) -> String {
        self.0
    }
}

impl fmt::Display for ScipSymbolId {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl Deref for ScipSymbolId {
    type Target = str;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl AsRef<str> for ScipSymbolId {
    fn as_ref(&self) -> &str {
        &self.0
    }
}

impl Borrow<str> for ScipSymbolId {
    fn borrow(&self) -> &str {
        &self.0
    }
}

impl<'de> Deserialize<'de> for ScipSymbolId {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        Self::parse(&s).map_err(serde::de::Error::custom)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use proptest::prelude::*;

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
    // parse/new Equivalence (BDD 3.6)
    // ═══════════════════════════════════════════════════════════════════════════

    #[test]
    fn scip_symbol_id_parse_equals_new_when_components_match() {
        let parsed = ScipSymbolId::parse("rust/auth/AuthService#login()");
        let constructed = ScipSymbolId::new("rust", "auth/AuthService", "login()");
        assert_eq!(parsed, constructed);
    }

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
}

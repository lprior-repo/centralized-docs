#[cfg(test)]
mod tests {
    use crate::cli::validation::*;

    #[test]
    fn test_filter_regex_rejects_nested_star_quantifiers() {
        let result = validate_filter_regex("(.*)*");
        assert!(result.is_err());
    }

    #[test]
    fn test_filter_regex_rejects_nested_plus_quantifiers() {
        let result = validate_filter_regex("(.+)+");
        assert!(result.is_err());
    }

    #[test]
    fn test_filter_regex_rejects_too_long_pattern() {
        let long_pattern = "a".repeat(501);
        let result = validate_filter_regex(&long_pattern);
        assert!(result.is_err());
    }

    #[test]
    fn test_filter_regex_accepts_safe_patterns() {
        let safe_patterns = ["^/docs/", r"\d+", "^api/v[0-9]+", "[a-z]+"];
        for pattern in &safe_patterns {
            let result = validate_filter_regex(pattern);
            assert!(result.is_ok());
        }
    }

    #[test]
    fn test_filter_regex_rejects_invalid_syntax() {
        let result = validate_filter_regex("[unclosed");
        assert!(result.is_err());
    }
}

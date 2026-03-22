#[cfg(test)]
mod tests {
    use crate::cli::validation::*;

    #[test]
    fn test_validate_threshold_zero() {
        let result = validate_threshold("0.0");
        assert!(result.is_ok());
    }

    #[test]
    fn test_validate_threshold_positive() {
        let result = validate_threshold("0.5");
        assert!(result.is_ok());
    }

    #[test]
    fn test_validate_threshold_at_upper_bound() {
        let result = validate_threshold("10.0");
        assert!(result.is_ok());
    }

    #[test]
    fn test_validate_threshold_negative_rejected() {
        let result = validate_threshold("-0.5");
        assert!(result.is_err());
    }

    #[test]
    fn test_validate_threshold_too_large() {
        let result = validate_threshold("10.1");
        assert!(result.is_err());
    }

    #[test]
    fn test_validate_threshold_invalid_string() {
        let result = validate_threshold("invalid");
        assert!(result.is_err());
    }
}

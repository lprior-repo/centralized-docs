#[cfg(test)]
mod tests {
    use crate::cli::validation::*;

    #[test]
    fn test_validate_delay_zero() {
        let result = validate_delay("0");
        assert!(result.is_ok());
    }

    #[test]
    fn test_validate_delay_positive() {
        let result = validate_delay("500");
        assert!(result.is_ok());
    }

    #[test]
    fn test_validate_delay_negative_one_rejected() {
        let result = validate_delay("-1");
        assert!(result.is_err());
    }

    #[test]
    fn test_validate_delay_at_upper_bound() {
        let result = validate_delay("60000");
        assert!(result.is_ok());
    }

    #[test]
    fn test_validate_delay_exceeds_upper_bound() {
        let result = validate_delay("60001");
        assert!(result.is_err());
    }

    #[test]
    fn test_validate_delay_invalid_string() {
        let result = validate_delay("invalid");
        assert!(result.is_err());
    }
}

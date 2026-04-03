#[cfg(test)]
mod tests {
    use crate::cli::validation::*;

    #[test]
    fn test_validate_limit_one() {
        let result = validate_limit_cli("1");
        assert!(result.is_ok());
    }

    #[test]
    fn test_validate_limit_positive() {
        let result = validate_limit_cli("10");
        assert!(result.is_ok());
    }

    #[test]
    fn test_validate_limit_zero_rejected() {
        let result = validate_limit_cli("0");
        assert!(result.is_err());
    }

    #[test]
    fn test_validate_limit_negative_one_rejected() {
        let result = validate_limit_cli("-1");
        assert!(result.is_err());
    }

    #[test]
    fn test_validate_limit_exceeds_upper_bound() {
        let result = validate_limit_cli("1001");
        assert!(result.is_err());
    }
}

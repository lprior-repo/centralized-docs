// Unit test for safe numeric conversions
#[cfg(test)]
mod tests {
    use std::convert::TryFrom;

    #[test]
    fn test_header_level_conversion() {
        // Test case 1: Valid header level (1-6 hashes)
        for len in 1..=6 {
            let level = u8::try_from(len).unwrap_or(1);
            assert!(level > 0 && level <= 6, "Header level out of range");
        }
        
        // Test case 2: Edge case - very long string (overflow protection)
        let long_str = "#".repeat(300);
        let level = u8::try_from(long_str.len()).unwrap_or(1);
        // Should fallback to 1, not overflow
        assert_eq!(level, 1, "Should fallback on overflow");
    }

    #[test]
    fn test_backoff_calculation() {
        // Test exponential backoff with safe casting
        let base_ms = 250u64;
        let retry_count = 0u32;
        let multiplier = 2.0f32;
        
        // Safe cast with bounds checking
        let exponent = i32::try_from(retry_count).unwrap_or(i32::MAX);
        let backoff = base_ms as f32 * multiplier.powi(exponent);
        let clamped = backoff.max(0.0).min(f32::MAX);
        
        let result = if clamped.is_finite() && clamped >= 0.0 {
            (clamped as u64).min(30000)
        } else {
            30000
        };
        
        assert_eq!(result, 250, "First retry should be 250ms");
        
        // Test with retry_count = 1
        let retry_count = 1u32;
        let exponent = i32::try_from(retry_count).unwrap_or(i32::MAX);
        let backoff = base_ms as f32 * multiplier.powi(exponent);
        let clamped = backoff.max(0.0).min(f32::MAX);
        let result = if clamped.is_finite() && clamped >= 0.0 {
            (clamped as u64).min(30000)
        } else {
            30000
        };
        assert_eq!(result, 500, "Second retry should be 500ms");
    }

    #[test]
    fn test_word_count_conversion() {
        // Test safe conversion from u64 to usize
        let word_counts: Vec<u64> = vec![100, 500, 1000, 5000];
        let total: usize = word_counts.iter()
            .filter_map(|&c| usize::try_from(c).ok())
            .sum();
        
        assert_eq!(total, 6600, "Word count conversion should succeed");
        
        // Test with value that wouldn't overflow (word_count is realistic)
        let word_count = 1u64;
        let _converted = usize::try_from(word_count).expect("Should convert small numbers");
    }

    #[test]
    fn test_heading_text_length_conversion() {
        // Test safe conversion of heading text length from usize to u32
        let heading_text = "Example Heading";
        let len = heading_text.len();
        
        let level = u32::try_from(len).unwrap_or(1);
        assert!(level > 0, "Heading level should be positive");
        assert_eq!(level, 15, "Heading text length should be 15");
    }
}

fn main() {
    println!("Safe numeric conversion tests - see #[cfg(test)] module");
}

/// Truncate summary to fit in a description
#[must_use]
pub fn truncate_summary(text: &str, max_len: usize) -> String {
    let cleaned = text.replace('\n', " ").trim().to_string();
    let char_count = cleaned.chars().count();

    if char_count <= max_len {
        return cleaned;
    }

    // Handle edge cases
    if max_len == 0 {
        return String::new();
    }

    if max_len <= 3 {
        // Can't fit "...", just return truncated without ellipsis
        return safe_truncate_chars(&cleaned, max_len);
    }

    // Normal case: truncate and add "..."
    let truncated = safe_truncate_chars(&cleaned, max_len.saturating_sub(3));
    format!("{truncated}...")
}

/// Safely truncate a string to a maximum number of characters, ensuring UTF-8 character boundaries
pub(crate) fn safe_truncate_chars(text: &str, max_chars: usize) -> String {
    if max_chars == 0 {
        return String::new();
    }

    text.char_indices()
        .take(max_chars)
        .last()
        .map(|(idx, c)| {
            let byte_end = idx.saturating_add(c.len_utf8());
            text[..byte_end].to_string()
        })
        .map_or_else(String::new, std::convert::identity)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_truncate_summary() {
        assert_eq!(truncate_summary("Short text", 20), "Short text");
        assert_eq!(
            truncate_summary(
                "This is a much longer piece of text that needs truncation",
                20
            ),
            "This is a much lo..."
        );
    }

    #[test]
    fn test_truncate_summary_empty_string() {
        assert_eq!(truncate_summary("", 10), "");
    }

    #[test]
    fn test_truncate_summary_zero_max_len() {
        assert_eq!(truncate_summary("Hello", 0), "");
    }

    #[test]
    fn test_truncate_summary_max_len_one() {
        let result = truncate_summary("Hello", 1);
        assert_eq!(result, "H");
    }

    #[test]
    fn test_truncate_summary_max_len_two() {
        let result = truncate_summary("Hello", 2);
        assert_eq!(result, "He");
    }

    #[test]
    fn test_truncate_summary_max_len_three() {
        let result = truncate_summary("Hello", 3);
        assert_eq!(result, "Hel");
    }

    #[test]
    fn test_truncate_summary_newlines_stripped() {
        assert_eq!(truncate_summary("Line1\nLine2", 20), "Line1 Line2");
    }

    #[test]
    fn test_safe_truncate_chars() {
        assert_eq!(safe_truncate_chars("hello", 3), "hel");
        assert_eq!(safe_truncate_chars("hello", 0), "");
        assert_eq!(safe_truncate_chars("hello", 10), "hello");
    }
}

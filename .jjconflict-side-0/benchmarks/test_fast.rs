fn contains_ignore_ascii_case(haystack: &str, needle: &str) -> bool {
    let needle_len = needle.len();
    if needle_len == 0 { return true; }
    if haystack.len() < needle_len { return false; }
    
    let n_bytes = needle.as_bytes();
    let first_lower = n_bytes[0].to_ascii_lowercase();
    let first_upper = n_bytes[0].to_ascii_uppercase();
    let h_bytes = haystack.as_bytes();
    
    let mut i = 0;
    let max_i = h_bytes.len() - needle_len;
    while i <= max_i {
        let b = h_bytes[i];
        if b == first_lower || b == first_upper {
            if h_bytes[i..i + needle_len].eq_ignore_ascii_case(n_bytes) {
                return true;
            }
        }
        i += 1;
    }
    false
}
fn main() {
    assert!(contains_ignore_ascii_case("Hello WORLD", "world"));
    assert!(!contains_ignore_ascii_case("Hello WORLD", "worlds"));
    assert!(contains_ignore_ascii_case("copyright 2024", "Copyright"));
}

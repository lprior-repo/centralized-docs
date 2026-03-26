use std::time::Instant;

fn search_slow(haystack: &str, needle: &str) -> bool {
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

fn search_fast(haystack: &str, needle: &str) -> bool {
    let needle_len = needle.len();
    if needle_len == 0 { return true; }
    if haystack.len() < needle_len { return false; }
    
    let n_bytes = needle.as_bytes();
    let h_bytes = haystack.as_bytes();
    
    let first_lower = n_bytes[0].to_ascii_lowercase();
    let first_upper = n_bytes[0].to_ascii_uppercase();
    let last_lower = n_bytes[needle_len - 1].to_ascii_lowercase();
    let last_upper = n_bytes[needle_len - 1].to_ascii_uppercase();
    
    let mut i = 0;
    let max_i = h_bytes.len() - needle_len;
    while i <= max_i {
        let b = h_bytes[i];
        if b == first_lower || b == first_upper {
            let last_b = h_bytes[i + needle_len - 1];
            if last_b == last_lower || last_b == last_upper {
                if h_bytes[i + 1..i + needle_len - 1].eq_ignore_ascii_case(&n_bytes[1..needle_len - 1]) {
                    return true;
                }
            }
        }
        i += 1;
    }
    false
}

fn main() {
    let haystack = "This is a regular text line with many char c inside it. Let's see if copyright keyword is checked slower. ".repeat(100);
    
    let t = Instant::now();
    for _ in 0..10000 { search_slow(&haystack, "copyright"); }
    println!("search_slow: {:?}", t.elapsed());
    
    let t = Instant::now();
    for _ in 0..10000 { search_fast(&haystack, "copyright"); }
    println!("search_fast: {:?}", t.elapsed());
}

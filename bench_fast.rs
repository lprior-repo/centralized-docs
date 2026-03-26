use std::time::Instant;

fn search1(haystack: &str, needle: &str) -> bool {
    let needle_len = needle.len();
    let n_bytes = needle.as_bytes();
    let first_lower = n_bytes[0].to_ascii_lowercase();
    let first_upper = n_bytes[0].to_ascii_uppercase();
    let h_bytes = haystack.as_bytes();
    let mut i = 0;
    if haystack.len() < needle_len { return false; }
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

fn search2(haystack: &str, needle: &str) -> bool {
    let needle_len = needle.len();
    if haystack.len() < needle_len { return false; }
    let n_bytes = needle.as_bytes();
    let first_lower = n_bytes[0].to_ascii_lowercase();
    let first_upper = n_bytes[0].to_ascii_uppercase();
    let mut haystack_bytes = haystack.as_bytes();
    
    while !haystack_bytes.is_empty() {
        let pos = match haystack_bytes.iter().position(|&b| b == first_lower || b == first_upper) {
            Some(p) => p,
            None => return false,
        };
        haystack_bytes = &haystack_bytes[pos..];
        if haystack_bytes.len() < needle_len { return false; }
        if haystack_bytes[..needle_len].eq_ignore_ascii_case(n_bytes) { return true; }
        haystack_bytes = &haystack_bytes[1..];
    }
    false
}

fn main() {
    let haystack = "This is a very long text ".repeat(100) + "Copyright 2024" + &" another text ".repeat(100);
    
    let t = Instant::now();
    for _ in 0..10000 { search1(&haystack, "copyright"); }
    println!("search1: {:?}", t.elapsed());
    
    let t = Instant::now();
    for _ in 0..10000 { search2(&haystack, "copyright"); }
    println!("search2: {:?}", t.elapsed());
}

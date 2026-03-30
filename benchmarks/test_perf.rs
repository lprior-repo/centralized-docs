use std::time::Instant;

fn bench_word_count() {
    let lines = vec!["this is a test line with some words"; 10000];
    
    let t = Instant::now();
    let c1: usize = lines.iter().map(|s| s.split_whitespace().count()).sum();
    println!("map sum: {:?}", t.elapsed());
    
    let t = Instant::now();
    let c2 = lines.join(" ").split_whitespace().count();
    println!("join split: {:?}", t.elapsed());
}

fn bench_case_insensitive() {
    let lines = vec!["this is a test line with some words without copyright"; 1000];
    
    let t = Instant::now();
    for line in &lines {
        let _ = line.to_lowercase().contains("copyright");
    }
    println!("to_lowercase: {:?}", t.elapsed());
    
    let t = Instant::now();
    for line in &lines {
        let _ = line.as_bytes().windows("copyright".len()).any(|w| w.eq_ignore_ascii_case(b"copyright"));
    }
    println!("windows eq_ignore: {:?}", t.elapsed());
}

fn main() {
    bench_word_count();
    bench_case_insensitive();
}

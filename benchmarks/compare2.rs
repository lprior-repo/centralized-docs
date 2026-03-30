use std::time::Instant;

fn main() {
    let lines: Vec<&str> = vec!["This is a simple text line with some words in it"; 1000];
    
    let t = Instant::now();
    for _ in 0..1000 {
        let joined = lines.join(" ");
        let count = joined.split_whitespace().count();
        core::hint::black_box(count);
    }
    println!("joined: {:?}", t.elapsed());
    
    let t = Instant::now();
    for _ in 0..1000 {
        let mut count = 0;
        for line in &lines {
            count += line.split_whitespace().count();
        }
        core::hint::black_box(count);
    }
    println!("per_line: {:?}", t.elapsed());
}

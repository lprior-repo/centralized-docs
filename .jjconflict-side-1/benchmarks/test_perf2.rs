use std::time::Instant;

fn bench_case_insensitive() {
    let lines = vec!["this is a test line with some words without copyright"; 1000];

    let t = Instant::now();
    for line in &lines {
        let res = line.to_lowercase().contains("copyright");
        std::hint::black_box(res);
    }
    println!("to_lowercase: {:?}", t.elapsed());

    let t = Instant::now();
    for line in &lines {
        let res = line
            .as_bytes()
            .windows("copyright".len())
            .any(|w| w.eq_ignore_ascii_case(b"copyright"));
        std::hint::black_box(res);
    }
    println!("windows eq_ignore: {:?}", t.elapsed());
}

fn main() {
    bench_case_insensitive();
}

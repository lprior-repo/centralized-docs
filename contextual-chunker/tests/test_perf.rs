use std::time::Instant;

#[test]
fn test_perf() {
    let text = "A".repeat(2000);
    let bpe = tiktoken_rs::cl100k_base().unwrap();

    let start = Instant::now();
    let tokens1 = bpe.encode_with_special_tokens(&text).len();
    println!(
        "encode_with_special_tokens: {}, Time: {:?}",
        tokens1,
        start.elapsed()
    );

    let start = Instant::now();
    let tokens2 = bpe.encode_ordinary(&text).len();
    println!("encode_ordinary: {}, Time: {:?}", tokens2, start.elapsed());
}

use std::time::Instant;

#[test]
fn test_perf() -> Result<(), Box<dyn std::error::Error>> {
    let text = "A".repeat(2000);
    let bpe = tiktoken_rs::cl100k_base()?;

    let start = Instant::now();
    let tokens1 = bpe.encode_with_special_tokens(&text).len();
    println!(
        "encode_with_special_tokens: {tokens1}, Time: {:?}",
        start.elapsed()
    );

    let start = Instant::now();
    let tokens2 = bpe.encode_ordinary(&text).len();
    println!("encode_ordinary: {tokens2}, Time: {:?}", start.elapsed());

    Ok(())
}

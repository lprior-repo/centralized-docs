use std::time::Instant;

#[test]
fn test_perf() -> Result<(), Box<dyn std::error::Error>> {
    let text = "A".repeat(2000);
    let bpe = tiktoken_rs::cl100k_base()?;

    let start = Instant::now();
    let encoded1 = bpe.encode_with_special_tokens(&text);
    let tokens1 = encoded1.len();
    println!(
        "encode_with_special_tokens: {tokens1}, Time: {:?}",
        start.elapsed()
    );

    let start = Instant::now();
    let encoded2 = bpe.encode_ordinary(&text);
    let tokens2 = encoded2.len();
    println!("encode_ordinary: {tokens2}, Time: {:?}", start.elapsed());

    assert!(
        !encoded1.is_empty(),
        "encode_with_special_tokens should produce tokens"
    );
    assert!(
        !encoded2.is_empty(),
        "encode_ordinary should produce tokens"
    );
    assert!(
        tokens1 > 0,
        "token count for encode_with_special_tokens must be positive"
    );
    assert!(
        tokens2 > 0,
        "token count for encode_ordinary must be positive"
    );

    Ok(())
}

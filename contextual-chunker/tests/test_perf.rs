use std::time::Instant;

#[test]
fn test_perf() {
    let mut text = String::with_capacity(2_000_000);
    for _ in 0..2_000_000 {
        text.push('A');
    }

    println!("String built");
    let start = Instant::now();
    let safe_text = if text.len() > 50_000 && !text.contains('\n') {
        use itertools::Itertools;
        text.chars()
            .chunks(500)
            .into_iter()
            .map(Iterator::collect::<String>)
            .join("\n")
    } else {
        text.to_string()
    };
    println!("Chunking took: {:?}", start.elapsed());

    println!("Starting text_splitter");
    let start = Instant::now();
    let tokenizer = tiktoken_rs::cl100k_base().unwrap();
    let config = text_splitter::ChunkConfig::new(512).with_sizer(tokenizer);
    let splitter = text_splitter::MarkdownSplitter::new(config);
    let chunks: Vec<String> = splitter
        .chunks(safe_text.as_str())
        .map(String::from)
        .collect();
    println!("Splitting took: {:?}", start.elapsed());
    println!("Generated {} chunks", chunks.len());
}

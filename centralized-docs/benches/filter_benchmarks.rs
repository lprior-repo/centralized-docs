#![allow(clippy::pedantic)]
#![allow(clippy::unwrap_used)]
#![allow(clippy::expect_used)]
#![allow(clippy::panic)]

use criterion::{black_box, criterion_group, criterion_main, Criterion};
use doc_transformer::filter::{filter_markdown, FilterConfig};

fn bench_filter_markdown(c: &mut Criterion) {
    let mut md = String::with_capacity(1024 * 1024);
    for i in 0..10000 {
        if i % 100 == 0 {
            md.push_str("## Navigation\n\n- Link 1\n- Link 2\n\n");
        } else if i % 50 == 0 {
            md.push_str("Copyright 2024 Company.\n\n");
        } else if i % 10 == 0 {
            md.push_str(&format!("## Heading {}\n\n", i));
        } else {
            md.push_str("This is a paragraph with some content that is meant to be kept and has enough words to pass the minimum word count threshold for filtering. It represents a typical paragraph in a documentation page.\n\n");
        }
    }

    let config = FilterConfig::default();

    c.bench_function("filter_markdown_1mb", |b| {
        b.iter(|| filter_markdown(black_box(&md), black_box(&config)))
    });
}

criterion_group!(benches, bench_filter_markdown);
criterion_main!(benches);

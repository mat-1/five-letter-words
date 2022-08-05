use criterion::{black_box, criterion_group, criterion_main, Criterion};
use five_letter_words::{get_results, word_to_num};
use std::collections::{BTreeSet, HashSet};

// const WORDS: &str = include_str!("wordle-words.txt");
const WORDS: &str = include_str!("../src/smaller.txt");

pub fn criterion_benchmark(c: &mut Criterion) {
    let words: HashSet<&str> = WORDS.lines().collect();
    let packed_words: BTreeSet<u32> = words.iter().copied().flat_map(word_to_num).collect();
    let packed_words: Vec<u32> = packed_words.into_iter().collect();

    let mut group = c.benchmark_group("get_results");
    group.sample_size(10);
    group.bench_function("Results", |b| {
        b.iter(|| black_box(get_results(&packed_words)))
    });
    group.finish();
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);

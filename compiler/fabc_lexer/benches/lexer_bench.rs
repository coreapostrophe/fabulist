use std::hint::black_box;

use criterion::{criterion_group, criterion_main, Criterion};
use fabc_lexer::Lexer;

fn lexer_performance(c: &mut Criterion) {
    let mut group = c.benchmark_group("lexer");

    group.bench_with_input(
        "tokenizes_simple_story",
        fabc_reg_test::SIMPLE_STORY,
        |b, source| {
            b.iter(|| {
                let _tokens = Lexer::tokenize(black_box(source));
            })
        },
    );

    group.bench_with_input(
        "tokenizes_complex_story",
        fabc_reg_test::COMPLEX_STORY,
        |b, source| {
            b.iter(|| {
                let _tokens = Lexer::tokenize(black_box(source));
            });
        },
    );

    group.finish();
}

criterion_group!(benches, lexer_performance);
criterion_main!(benches);

use std::hint::black_box;

use criterion::{criterion_group, criterion_main, Criterion};
use fabc_lexer::Lexer;
use fabc_parser::{ast::story::Story, Parser};

fn parser_performance(c: &mut Criterion) {
    let source = fabc_reg_test::SIMPLE_STORY;
    let tokens = Lexer::tokenize(source).expect("Failed to tokenize source");

    let mut group = c.benchmark_group("parser");

    group.bench_with_input("parses_simple_story", &tokens, |b, tokens| {
        b.iter(|| {
            let _ast = Parser::parse::<Story>(black_box(tokens)).unwrap();
        })
    });

    let source = fabc_reg_test::COMPLEX_STORY;
    let tokens = Lexer::tokenize(source).expect("Failed to tokenize source");

    group.bench_with_input("parses_complex_story", &tokens, |b, tokens| {
        b.iter(|| {
            let _ast = Parser::parse::<Story>(black_box(tokens)).unwrap();
        })
    });

    group.finish();
}

fn full_parsing_performance(c: &mut Criterion) {
    let mut group = c.benchmark_group("full_parsing");

    group.bench_with_input(
        "tokenizes_and_parses_simple_story",
        fabc_reg_test::COMPLEX_STORY,
        |b, source| {
            b.iter(|| {
                let tokens = Lexer::tokenize(black_box(source)).expect("Failed to tokenize source");
                let _ast = Parser::parse::<Story>(&tokens).unwrap();
            })
        },
    );

    group.bench_with_input(
        "tokenizes_and_parses_complex_story",
        fabc_reg_test::COMPLEX_STORY,
        |b, source| {
            b.iter(|| {
                let tokens = Lexer::tokenize(black_box(source)).expect("Failed to tokenize source");
                let _ast = Parser::parse::<Story>(&tokens).unwrap();
            })
        },
    );

    group.finish();
}

criterion_group!(benches, parser_performance, full_parsing_performance);
criterion_main!(benches);

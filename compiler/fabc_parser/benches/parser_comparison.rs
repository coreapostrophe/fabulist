use std::hint::black_box;

use criterion::{criterion_group, criterion_main, BenchmarkId, Criterion};

// Sample Fabulist code for testing
const SIMPLE_STORY: &str = r##"
story { "start":  "dialogue_1" }

## dialogue_1
[Jose]
> "What's up"
    - "The ceiling." => {
        "next": () => {
            goto dialogue_2;
        }
    }
    - "Nothing much." => {
        "next":  () => {
            goto dialogue_2;
        }
    }
"##;

const COMPLEX_STORY: &str = r##"
story { "start":  "part_1" }

## part_1
[Hero]
> "Hello there!" => {
    "mood": "happy",
    "next": () => {
        let x = 10;
        let y = 20;
        context.total = x + y;
        goto part_2;
    }
}
    - "Hi!" => {
        "goto": () => { goto part_2; }
    }
    - "Who are you?" => {
        "goto": () => { goto part_3; }
    }

## part_2
[Villain]
> "I've been expecting you."

## part_3
[Hero]
> "I don't trust you."
"##;

fn benchmark_compiler_parser(c: &mut Criterion) {
    let mut group = c.benchmark_group("parser_full");

    group.bench_with_input(
        BenchmarkId::new("compiler", "simple"),
        &SIMPLE_STORY,
        |b, source| {
            b.iter(|| {
                let mut lexer = fabc_lexer::Lexer::new(black_box(source));
                let tokens = lexer.tokenize().unwrap();
                let mut parser = fabc_parser::Parser::new(tokens);
                parser.parse()
            })
        },
    );

    group.bench_with_input(
        BenchmarkId::new("compiler", "complex"),
        &COMPLEX_STORY,
        |b, source| {
            b.iter(|| {
                let mut lexer = fabc_lexer::Lexer::new(black_box(source));
                let tokens = lexer.tokenize().unwrap();
                let mut parser = fabc_parser::Parser::new(tokens);
                parser.parse()
            })
        },
    );

    group.finish();
}

fn benchmark_legacy_parser(c: &mut Criterion) {
    let mut group = c.benchmark_group("parser_full");

    group.bench_with_input(
        BenchmarkId::new("fabulist_lang", "simple"),
        SIMPLE_STORY,
        |b, source| b.iter(|| fabulist_lang::parser::FabulistParser::parse(black_box(source))),
    );

    group.bench_with_input(
        BenchmarkId::new("fabulist_lang", "complex"),
        COMPLEX_STORY,
        |b, source| b.iter(|| fabulist_lang::parser::FabulistParser::parse(black_box(source))),
    );

    group.finish();
}

criterion_group!(benches, benchmark_compiler_parser, benchmark_legacy_parser);
criterion_main!(benches);

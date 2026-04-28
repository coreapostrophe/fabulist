use criterion::{criterion_group, criterion_main, Criterion};
use fabulist_lang::parser::FabulistParser;

const SIMPLE_STORY: &str = r##"
story { "start":  "dialogue_1" }

# dialogue_1
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

# part_1
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
        "next": () => { goto part_2; }
    }
    - "Who are you?" => {
        "next": () => { goto part_3; }
    }

# part_2
[Villain]
> "I've been expecting you."

# part_3
[Hero]
> "I don't trust you."
"##;

fn parsing_performance(c: &mut Criterion) {
    let mut group = c.benchmark_group("parsing");

    group.bench_with_input("parses_simple_story", SIMPLE_STORY, |b, source| {
        b.iter(|| {
            FabulistParser::parse(source).unwrap();
        })
    });

    group.bench_with_input("parses_complex_story", COMPLEX_STORY, |b, source| {
        b.iter(|| {
            FabulistParser::parse(source).unwrap();
        })
    });

    group.finish();
}

criterion_group!(benches, parsing_performance);
criterion_main!(benches);

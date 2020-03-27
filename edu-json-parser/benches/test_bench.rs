use criterion::{black_box, criterion_group, criterion_main, Criterion};

use edu_json_parser::{parse_json};
use serde_json::Value;

const MANY_CARDS: &'static str = include!("many.json");

const SPOILED_CARDS: &'static str = include!("spoiled.json");

const TWITTER_JSON: &'static str = include!("twitter.json");

fn bench_edu_just_nodes() -> String {
    let _many_cards = parse_json(MANY_CARDS);
    let _spoiled_cards = parse_json(SPOILED_CARDS);
    String::from("42")
}

fn json_bench() -> String {
    let _many_cards = json::parse(MANY_CARDS);
    let _spoiled_cards = json::parse(SPOILED_CARDS);
    String::from("42")
}

fn serde_bench() -> String {
    let _many_cards: serde_json::Result<Value> = serde_json::from_str(MANY_CARDS);
    let _spoiled_cards: serde_json::Result<Value> = serde_json::from_str(SPOILED_CARDS);
    String::from("42")
}

fn just_iterate() -> String {
    for _c in MANY_CARDS.chars() {

    }
    for _c in SPOILED_CARDS.chars() {

    }
    String::from("42")
}

fn bench_edu_just_nodes_twitter() -> String {
    let _many_cards = parse_json(TWITTER_JSON);
    String::from("42")
}

fn json_bench_twitter() -> String {
    let _many_cards = json::parse(TWITTER_JSON);
    String::from("42")
}

fn serde_bench_twitter() -> String {
    let _many_cards: serde_json::Result<Value> = serde_json::from_str(TWITTER_JSON);
    String::from("42")
}

fn just_iterate_twitter() -> String {
    for _c in TWITTER_JSON.chars() {

    }
    String::from("42")
}

fn bench_json(c: &mut Criterion) {
    c.bench_function("bench edu-json-parser", |b| b.iter(|| {
        let x = bench_edu_just_nodes();
        black_box(x)
    }));
    c.bench_function("bench json library", |b| b.iter(|| {
        let x = json_bench();
        black_box(x)
    }));
    c.bench_function("bench serde library", |b| b.iter(|| {
        let x = serde_bench();
        black_box(x)
    }));
    c.bench_function("bench just iterate over strings", |b| b.iter(|| {
        let x = just_iterate();
        black_box(x)
    }));
    c.bench_function("bench edu-json-parser twitter", |b| b.iter(|| {
        let x = bench_edu_just_nodes_twitter();
        black_box(x)
    }));
    c.bench_function("bench json library twitter", |b| b.iter(|| {
        let x = json_bench_twitter();
        black_box(x)
    }));
    c.bench_function("bench serde library twitter", |b| b.iter(|| {
        let x = serde_bench_twitter();
        black_box(x)
    }));
    c.bench_function("bench just iterate over strings twitter", |b| b.iter(|| {
        let x = just_iterate_twitter();
        black_box(x)
    }));
}

criterion_group!(json, bench_json);
criterion_main!(json);
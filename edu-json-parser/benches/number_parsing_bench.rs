use criterion::{black_box, criterion_group, criterion_main, Criterion};

pub const TEST_LITERAL: &'static str = "\
    0123456789012345\
    6789012345678901\
    2345678901234567\
    8901234567890123\
    4567890123456789\
    0123456789";

fn iter_sum() -> u64 {
    let bytes = TEST_LITERAL.as_bytes();
    let decrease = TEST_LITERAL.len() as u64 * 1024 * 1024 * b'0' as u64;
    let mut acc: u64 = 0;
    for _ in 0..1024*1024 {
        for b in bytes{
            acc += *b as u64;
        }
    }
    acc - decrease
}

fn iter_for_sum() -> u64 {
    let bytes = TEST_LITERAL.as_bytes();
    let mut acc: u64 = 0;
    for _ in 0..1024*1024 {
        for i in 0..bytes.len() {
            acc += (bytes[i] - b'0') as u64;
        }
    }
    acc
}

unsafe fn iter_for_unsafe_sum() -> u64 {
    let bytes = TEST_LITERAL.as_bytes();
    let mut acc: u64 = 0;
    let p = bytes.as_ptr();
    for _ in 0..1024*1024 {
        for i in 0..bytes.len() {
            acc += (*(p.add(i)) - b'0') as u64;
        }
    }
    acc
}

// fn for_sum() -> u32 {
//     let bytes = TEST_LITERAL.bytes();
//     let mut acc: u32 = 0;
//     for i in 0..bytes.len() {
//         acc += (bytes[i] - b'0') as u32;
//     }
//     acc
// }
//
// unsafe fn unsafe_for_sum() -> u32 {
//     let bytes = TEST_LITERAL.bytes();
//     let mut acc: u32 = 0;
//     let mut ptr = bytes.as_ptr();
//     for b in bytes{
//         acc += (b - b'0') as u32;
//     }
//     acc
// }

fn bench_accs(c: &mut Criterion) {
    c.bench_function("iter_sum", |b| b.iter(|| {
        black_box(iter_sum())
    }));

    c.bench_function("iter_for_sum", |b| b.iter(|| {
        black_box(iter_for_sum())
    }));

    c.bench_function("iter_for_unsafe_sum", |b| b.iter(|| {
        black_box(unsafe { iter_for_unsafe_sum() })
    }));
}

criterion_group!(accs, bench_accs);
criterion_main!(accs);
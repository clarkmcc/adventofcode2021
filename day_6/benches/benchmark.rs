use criterion::{criterion_group, criterion_main, Criterion};
use day_6::{num_laternfish, parse_input};

fn criterion_benchmark(c: &mut Criterion) {
    c.bench_function("num_laternfish_part_1", |b| {
        let input = parse_input();
        b.iter(|| {
            num_laternfish(input.clone(), 80);
        })
    });
    c.bench_function("num_laternfish_part_2", |b| {
        let input = parse_input();
        b.iter(|| {
            num_laternfish(input.clone(), 256);
        })
    });
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);

use criterion::{criterion_group, criterion_main, Criterion};
use day_7::{optimal_positions_part1, optimal_positions_part2, parse_input};

fn criterion_benchmark(c: &mut Criterion) {
    c.bench_function("optimal_positions_part1", |b| {
        let input = parse_input();
        b.iter(|| {
            optimal_positions_part1(input.clone());
        })
    });
    c.bench_function("optimal_positions_part2", |b| {
        let input = parse_input();
        b.iter(|| {
            optimal_positions_part2(input.clone(), 2);
        })
    });
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);

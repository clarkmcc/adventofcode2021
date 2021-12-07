use criterion::{criterion_group, criterion_main, Criterion};
use day_5::{count_overlapping_points_part_1, count_overlapping_points_part_2, parse_input};

fn criterion_benchmark(c: &mut Criterion) {
    c.bench_function("count_overlapping_points_part_1", |b| {
        let input = parse_input();
        b.iter(|| {
            count_overlapping_points_part_1(input.clone());
        })
    });
    c.bench_function("count_overlapping_points_part_2", |b| {
        let input = parse_input();
        b.iter(|| {
            count_overlapping_points_part_2(input.clone());
        })
    });
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);

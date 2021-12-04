use criterion::{criterion_group, criterion_main, Criterion};
use day_2::{calculate_final_position, calculate_final_position_part_2, parse_input};

fn criterion_benchmark(c: &mut Criterion) {
    c.bench_function("calculate_final_position_part_1", |b| {
        b.iter(|| {
            calculate_final_position(parse_input());
        })
    });
    c.bench_function("calculate_final_position_part_2", |b| {
        b.iter(|| {
            calculate_final_position_part_2(parse_input());
        })
    });
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);

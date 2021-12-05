use criterion::{criterion_group, criterion_main, Criterion};
use day_3::{calculate_oxygen_co2_rating, parse_input};

fn criterion_benchmark(c: &mut Criterion) {
    c.bench_function("calculate_oxygen_co2_rating", |b| {
        let (bytes, size) = parse_input();
        b.iter(|| {
            calculate_oxygen_co2_rating(bytes.clone(), size);
        })
    });
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);

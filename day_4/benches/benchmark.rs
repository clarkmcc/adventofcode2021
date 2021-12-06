use criterion::{criterion_group, criterion_main, Criterion};
use day_4::{evaluate_bingo_boards, parse_input};

fn criterion_benchmark(c: &mut Criterion) {
    c.bench_function("evaluate_bingo_boards", |b| {
        let input = parse_input();
        b.iter(|| {
            evaluate_bingo_boards(input.0.clone(), input.1.clone());
        })
    });
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);

use criterion::{criterion_group, criterion_main, Criterion};
use toy_prng::*;

pub fn criterion_benchmark(criteria: &mut Criterion) {
    // range exclusive benchmark
    criteria.bench_function("random: range exclusive", |bench| {
        let mut rng = RandomNumberGenerator::new();
        bench.iter(|| {
            rng.range(1.0_f32..10_000_000_f32)
        })
    });
    // range inclusive benchmark
    criteria.bench_function("random: range inclusive", |bench| {
        let mut rng = RandomNumberGenerator::new();
        bench.iter(|| {
            rng.range(1.0_f32..=10_000_000_f32)
        })
    });
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
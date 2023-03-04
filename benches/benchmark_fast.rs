use criterion::{criterion_group, criterion_main, Criterion};
use alea_js::{AleaFast};

fn rng_next_fast(c: &mut Criterion) {
    let mut alea = AleaFast::new("".to_string());
    c.bench_function("rng fast random", |b| b.iter(|| alea.random()));
}

fn rng_init_zero_fast(c: &mut Criterion) {
    c.bench_function("rng fast init tiny seed", |b| b.iter(|| AleaFast::new("".to_string())));
}

fn rng_init_long_fast(c: &mut Criterion) {
    c.bench_function("rng fast init large seed", |b| b.iter(|| AleaFast::new("8d6uQIixjuYYY6etTcUs".to_string())));
}


criterion_group!(benches_fast, rng_next_fast, rng_init_zero_fast, rng_init_long_fast);
criterion_main!(benches_fast);
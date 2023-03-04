use criterion::{criterion_group, criterion_main, Criterion};
use alea_js::Alea;

fn rng_next(c: &mut Criterion) {
    let mut alea = Alea::new("".to_string());
    c.bench_function("rng next", |b| b.iter(|| alea.next()));
}

fn rng_init_zero(c: &mut Criterion) {
    c.bench_function("rng init tiny seed", |b| b.iter(|| Alea::new("".to_string())));
}

fn rng_init_long(c: &mut Criterion) {
    c.bench_function("rng init large seed", |b| b.iter(|| Alea::new("8d6uQIixjuYYY6etTcUs".to_string())));
}

criterion_group!(benches, rng_next, rng_init_zero, rng_init_long);
criterion_main!(benches);
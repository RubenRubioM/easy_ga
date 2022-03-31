use criterion::{criterion_group, criterion_main};
mod bench_genetic_algorithm;

use bench_genetic_algorithm::benchmark;

criterion_group!(
    initialization,
    benchmark::new,
    benchmark::new_with_values,
    benchmark::new_chain_calling,
);

criterion_group!(benches, benchmark::init, benchmark::run);
criterion_main!(initialization, benches);

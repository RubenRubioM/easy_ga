use criterion::{criterion_group, criterion_main};
mod bench_genetic_algorithm;
mod bench_selection;

criterion_group!(
    instantiation,
    bench_genetic_algorithm::benchmark::new,
    bench_genetic_algorithm::benchmark::new_with_values,
    bench_genetic_algorithm::benchmark::new_chain_calling,
    bench_genetic_algorithm::benchmark::genetic_algorithm_instantiation_comparation,
);

criterion_group!(
    run_without_fitness_goal,
    bench_genetic_algorithm::benchmark::run_different_population_size,
);

criterion_group!(
    genetic_algorithm_functions,
    bench_genetic_algorithm::benchmark::init
);

criterion_group!(
    logger,
    bench_genetic_algorithm::benchmark::logger_log_with_different_verbosity,
    bench_genetic_algorithm::benchmark::logger_save_with_different_verbosity,
    bench_genetic_algorithm::benchmark::logger_log_and_save_with_different_verbosity,
);

criterion_group!(
    selection_algorithms,
    bench_selection::benchmark::roulette_different_sizes,
    bench_selection::benchmark::random_different_sizes,
    bench_selection::benchmark::stochastic_different_sizes,
    bench_selection::benchmark::tournament_different_sizes,
    bench_selection::benchmark::tournament_different_tournament_participant,
    bench_selection::benchmark::roulette_vs_tournament,
);

criterion_main!(
    instantiation,
    run_without_fitness_goal,
    genetic_algorithm_functions,
    logger,
    selection_algorithms,
);

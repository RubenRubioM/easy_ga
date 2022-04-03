pub mod benchmark {
    use criterion::BenchmarkId;
    use criterion::{black_box, Criterion};
    use easy_ga::samples::MyGene as MockMyGene;
    use easy_ga::GeneticAlgorithm;
    use easy_ga::SelectionAlgorithms;

    /// Benchmark the GeneticAlgorithm::new function.
    pub fn new(c: &mut Criterion) {
        c.bench_function("GeneticAlgorithm::new", |b| {
            b.iter(|| {
                for _ in 0..10000 {
                    GeneticAlgorithm::<MockMyGene>::new();
                }
            })
        });
    }

    /// Benchmark the GeneticAlgorithm::new_with_values function.
    pub fn new_with_values(c: &mut Criterion) {
        let mut group = c.benchmark_group("GeneticAlgorithm::new_with_values");

        group.bench_function("Tournament", |b| {
            b.iter(|| {
                for _ in 0..10000 {
                    GeneticAlgorithm::<MockMyGene>::new_with_values(
                        black_box(100),
                        black_box(1000),
                        black_box(0.05),
                        black_box(0.90),
                        Box::new(SelectionAlgorithms::Tournament(black_box(2))),
                        black_box(f64::MAX),
                    );
                }
            })
        });

        group.bench_function("Roulette", |b| {
            b.iter(|| {
                for _ in 0..10000 {
                    GeneticAlgorithm::<MockMyGene>::new_with_values(
                        black_box(100),
                        black_box(1000),
                        black_box(0.05),
                        black_box(0.90),
                        Box::new(SelectionAlgorithms::Roulette),
                        black_box(f64::MAX),
                    );
                }
            })
        });

        group.finish();
    }

    /// Benchmarks the initialization of a GeneticAlgorithm with a chain-calling method.
    pub fn new_chain_calling(c: &mut Criterion) {
        c.bench_function("GeneticAlgorithm::new_chain_calling", |b| {
            b.iter(|| {
                for _ in 0..10000 {
                    GeneticAlgorithm::<MockMyGene>::new()
                        .population_size(black_box(100))
                        .iterations(black_box(1000))
                        .mutation_rate(black_box(0.05))
                        .selection_rate(black_box(0.90))
                        .selection_algorithm(Box::new(SelectionAlgorithms::Tournament(black_box(
                            2,
                        ))))
                        .fitness_goal(black_box(f64::MAX));
                }
            })
        });
    }

    /// Benchmarks a comparation between the 3 ways for instantiate a GeneticAlgorithm.
    pub fn genetic_algorithm_instantiation_comparation(c: &mut Criterion) {
        let mut group = c.benchmark_group("GeneticAlgorithm instantiation");

        group.bench_function("new()", |b| {
            b.iter(|| {
                for _ in 0..10000 {
                    GeneticAlgorithm::<MockMyGene>::new();
                }
            })
        });

        group.bench_function("new_with_values()", |b| {
            b.iter(|| {
                for _ in 0..10000 {
                    GeneticAlgorithm::<MockMyGene>::new_with_values(
                        black_box(100),
                        black_box(1000),
                        black_box(0.05),
                        black_box(0.90),
                        Box::new(SelectionAlgorithms::Tournament(black_box(2))),
                        black_box(f64::MAX),
                    );
                }
            })
        });

        group.bench_function("chain_calling", |b| {
            b.iter(|| {
                for _ in 0..10000 {
                    GeneticAlgorithm::<MockMyGene>::new()
                        .population_size(black_box(100))
                        .iterations(black_box(1000))
                        .mutation_rate(black_box(0.05))
                        .selection_rate(black_box(0.90))
                        .selection_algorithm(Box::new(SelectionAlgorithms::Tournament(black_box(
                            2,
                        ))))
                        .fitness_goal(black_box(f64::MAX));
                }
            })
        });

        group.finish();
    }

    /// Benchmark the GeneticAlgorithm::init function.
    pub fn init(c: &mut Criterion) {
        c.bench_function("GeneticAlgorithm::init", |b| {
            b.iter(|| {
                for _ in 0..10000 {
                    let _ = GeneticAlgorithm::<MockMyGene>::new().init();
                }
            })
        });
    }

    /// Benchmarks the GeneticAlgorithm::run method with different population sizes.
    pub fn run_different_population_size(c: &mut Criterion) {
        let mut group = c.benchmark_group("GeneticAlgorithm::run - population_size");
        group.measurement_time(core::time::Duration::from_secs(10));
        let population_sizes = [10, 50, 100, 250, 500];

        for population_size in population_sizes {
            group.bench_with_input(
                BenchmarkId::from_parameter(population_size),
                &population_size,
                |b, &population_size| {
                    b.iter(|| {
                        GeneticAlgorithm::<MockMyGene>::new()
                            .population_size(black_box(population_size))
                            .iterations(black_box(1000))
                            .mutation_rate(black_box(0.05))
                            .selection_rate(black_box(0.90))
                            .selection_algorithm(Box::new(SelectionAlgorithms::Tournament(
                                black_box(2),
                            )))
                            .fitness_goal(black_box(f64::MAX))
                            .init()
                            .unwrap()
                            .run();
                    })
                },
            );
        }

        group.finish();
    }
}

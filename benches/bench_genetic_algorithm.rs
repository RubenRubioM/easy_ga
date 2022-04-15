pub mod benchmark {
    use criterion::BenchmarkId;
    use criterion::{black_box, Criterion};
    use easy_ga::samples::MyGene as MockMyGene;
    use easy_ga::GeneticAlgorithm;
    use easy_ga::LOG_verbosity;
    use easy_ga::LOG_verbosity_type;
    use easy_ga::SelectionAlgorithms;
    use easy_ga::VerbosityLevel;
    use easy_ga::VerbosityType;

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
                            .run();
                    })
                },
            );
        }

        group.finish();
    }

    /// Benchmarks the logger with VerbosityType::LOG.
    pub fn logger_log_with_different_verbosity(c: &mut Criterion) {
        let mut group = c.benchmark_group("Logger_LOG - VerbosityLevel::");
        group.measurement_time(core::time::Duration::from_secs(10));

        LOG_verbosity_type(VerbosityType::LOG);

        LOG_verbosity(VerbosityLevel::DISABLED);
        group.bench_function("DISABLED", |b| {
            b.iter(|| {
                GeneticAlgorithm::<MockMyGene>::new()
                    .population_size(black_box(20))
                    .iterations(black_box(100))
                    .mutation_rate(black_box(0.05))
                    .selection_rate(black_box(0.90))
                    .selection_algorithm(Box::new(SelectionAlgorithms::Tournament(black_box(2))))
                    .fitness_goal(black_box(f64::MAX))
                    .init()
                    .run();
            })
        });

        LOG_verbosity(VerbosityLevel::LOW);
        group.bench_function("LOW", |b| {
            b.iter(|| {
                GeneticAlgorithm::<MockMyGene>::new()
                    .population_size(black_box(20))
                    .iterations(black_box(100))
                    .mutation_rate(black_box(0.05))
                    .selection_rate(black_box(0.90))
                    .selection_algorithm(Box::new(SelectionAlgorithms::Tournament(black_box(2))))
                    .fitness_goal(black_box(f64::MAX))
                    .init()
                    .run();
            })
        });

        LOG_verbosity(VerbosityLevel::MID);
        group.bench_function("MID", |b| {
            b.iter(|| {
                GeneticAlgorithm::<MockMyGene>::new()
                    .population_size(black_box(20))
                    .iterations(black_box(100))
                    .mutation_rate(black_box(0.05))
                    .selection_rate(black_box(0.90))
                    .selection_algorithm(Box::new(SelectionAlgorithms::Tournament(black_box(2))))
                    .fitness_goal(black_box(f64::MAX))
                    .init()
                    .run();
            })
        });

        LOG_verbosity(VerbosityLevel::HIGH);
        group.bench_function("HIGH", |b| {
            b.iter(|| {
                GeneticAlgorithm::<MockMyGene>::new()
                    .population_size(black_box(20))
                    .iterations(black_box(100))
                    .mutation_rate(black_box(0.05))
                    .selection_rate(black_box(0.90))
                    .selection_algorithm(Box::new(SelectionAlgorithms::Tournament(black_box(2))))
                    .fitness_goal(black_box(f64::MAX))
                    .init()
                    .run();
            })
        });

        group.finish();
    }

    /// Benchmarks the logger with VerbosityType::SAVE.
    pub fn logger_save_with_different_verbosity(c: &mut Criterion) {
        let mut group = c.benchmark_group("Logger_SAVE - VerbosityLevel::");
        group.measurement_time(core::time::Duration::from_secs(10));

        LOG_verbosity_type(VerbosityType::SAVE);

        LOG_verbosity(VerbosityLevel::DISABLED);
        group.bench_function("DISABLED", |b| {
            b.iter(|| {
                GeneticAlgorithm::<MockMyGene>::new()
                    .population_size(black_box(20))
                    .iterations(black_box(100))
                    .mutation_rate(black_box(0.05))
                    .selection_rate(black_box(0.90))
                    .selection_algorithm(Box::new(SelectionAlgorithms::Tournament(black_box(2))))
                    .fitness_goal(black_box(f64::MAX))
                    .init()
                    .run();
            })
        });

        LOG_verbosity(VerbosityLevel::LOW);
        group.bench_function("LOW", |b| {
            b.iter(|| {
                GeneticAlgorithm::<MockMyGene>::new()
                    .population_size(black_box(20))
                    .iterations(black_box(100))
                    .mutation_rate(black_box(0.05))
                    .selection_rate(black_box(0.90))
                    .selection_algorithm(Box::new(SelectionAlgorithms::Tournament(black_box(2))))
                    .fitness_goal(black_box(f64::MAX))
                    .init()
                    .run();
            })
        });

        LOG_verbosity(VerbosityLevel::MID);
        group.bench_function("MID", |b| {
            b.iter(|| {
                GeneticAlgorithm::<MockMyGene>::new()
                    .population_size(black_box(20))
                    .iterations(black_box(100))
                    .mutation_rate(black_box(0.05))
                    .selection_rate(black_box(0.90))
                    .selection_algorithm(Box::new(SelectionAlgorithms::Tournament(black_box(2))))
                    .fitness_goal(black_box(f64::MAX))
                    .init()
                    .run();
            })
        });

        LOG_verbosity(VerbosityLevel::HIGH);
        group.bench_function("HIGH", |b| {
            b.iter(|| {
                GeneticAlgorithm::<MockMyGene>::new()
                    .population_size(black_box(20))
                    .iterations(black_box(100))
                    .mutation_rate(black_box(0.05))
                    .selection_rate(black_box(0.90))
                    .selection_algorithm(Box::new(SelectionAlgorithms::Tournament(black_box(2))))
                    .fitness_goal(black_box(f64::MAX))
                    .init()
                    .run();
            })
        });

        group.finish();
    }

    /// Benchmarks the logger with VerbosityType::SAVE.
    pub fn logger_log_and_save_with_different_verbosity(c: &mut Criterion) {
        let mut group = c.benchmark_group("Logger_LOG_AND_SAVE - VerbosityLevel::");
        group.measurement_time(core::time::Duration::from_secs(10));

        LOG_verbosity_type(VerbosityType::LOG_AND_SAVE);

        LOG_verbosity(VerbosityLevel::DISABLED);
        group.bench_function("DISABLED", |b| {
            b.iter(|| {
                GeneticAlgorithm::<MockMyGene>::new()
                    .population_size(black_box(20))
                    .iterations(black_box(100))
                    .mutation_rate(black_box(0.05))
                    .selection_rate(black_box(0.90))
                    .selection_algorithm(Box::new(SelectionAlgorithms::Tournament(black_box(2))))
                    .fitness_goal(black_box(f64::MAX))
                    .init()
                    .run();
            })
        });

        LOG_verbosity(VerbosityLevel::LOW);
        group.bench_function("LOW", |b| {
            b.iter(|| {
                GeneticAlgorithm::<MockMyGene>::new()
                    .population_size(black_box(20))
                    .iterations(black_box(100))
                    .mutation_rate(black_box(0.05))
                    .selection_rate(black_box(0.90))
                    .selection_algorithm(Box::new(SelectionAlgorithms::Tournament(black_box(2))))
                    .fitness_goal(black_box(f64::MAX))
                    .init()
                    .run();
            })
        });

        LOG_verbosity(VerbosityLevel::MID);
        group.bench_function("MID", |b| {
            b.iter(|| {
                GeneticAlgorithm::<MockMyGene>::new()
                    .population_size(black_box(20))
                    .iterations(black_box(100))
                    .mutation_rate(black_box(0.05))
                    .selection_rate(black_box(0.90))
                    .selection_algorithm(Box::new(SelectionAlgorithms::Tournament(black_box(2))))
                    .fitness_goal(black_box(f64::MAX))
                    .init()
                    .run();
            })
        });

        LOG_verbosity(VerbosityLevel::HIGH);
        group.bench_function("HIGH", |b| {
            b.iter(|| {
                GeneticAlgorithm::<MockMyGene>::new()
                    .population_size(black_box(20))
                    .iterations(black_box(100))
                    .mutation_rate(black_box(0.05))
                    .selection_rate(black_box(0.90))
                    .selection_algorithm(Box::new(SelectionAlgorithms::Tournament(black_box(2))))
                    .fitness_goal(black_box(f64::MAX))
                    .init()
                    .run();
            })
        });

        group.finish();
    }
}

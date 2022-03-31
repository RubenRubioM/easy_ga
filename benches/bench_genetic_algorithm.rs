pub mod benchmark {
    use std::error::Error;

    use criterion::{black_box, Criterion};
    use easy_ga::samples::MyGene as MockMyGene;
    use easy_ga::GeneticAlgorithm;
    use easy_ga::SelectionAlgorithms;

    /// Benchmark the GeneticAlgorithm::new function.
    pub fn new(c: &mut Criterion) {
        c.bench_function("GeneticAlgorithm::new", |b| {
            b.iter(|| GeneticAlgorithm::<MockMyGene>::new())
        });
    }

    /// Benchmark the GeneticAlgorithm::new_with_values function.
    pub fn new_with_values(c: &mut Criterion) {
        let mut group = c.benchmark_group("GeneticAlgorithm::new_with_values");

        group.bench_function("Tournament", |b| {
            b.iter(|| {
                GeneticAlgorithm::<MockMyGene>::new_with_values(
                    black_box(100),
                    black_box(1000),
                    black_box(0.05),
                    black_box(0.90),
                    Box::new(SelectionAlgorithms::Tournament(black_box(2))),
                    black_box(f64::MAX),
                )
            })
        });

        group.bench_function("Roulette", |b| {
            b.iter(|| {
                GeneticAlgorithm::<MockMyGene>::new_with_values(
                    black_box(100),
                    black_box(1000),
                    black_box(0.05),
                    black_box(0.90),
                    Box::new(SelectionAlgorithms::Roulette),
                    black_box(f64::MAX),
                )
            })
        });

        group.finish();
    }

    /// Benchmarks the initialization of a GeneticAlgorithm with a chain-calling method.
    pub fn new_chain_calling(c: &mut Criterion) {
        c.bench_function("GeneticAlgorithm::new_chain_calling", |b| {
            b.iter(|| {
                GeneticAlgorithm::<MockMyGene>::new()
                    .population_size(black_box(100))
                    .iterations(black_box(1000))
                    .mutation_rate(black_box(0.05))
                    .selection_rate(black_box(0.90))
                    .selection_algorithm(Box::new(SelectionAlgorithms::Tournament(black_box(2))))
                    .fitness_goal(black_box(f64::MAX))
            })
        });
    }

    /// Benchmark the GeneticAlgorithm::init function.
    pub fn init(c: &mut Criterion) {
        c.bench_function("GeneticAlgorithm::init", |b| {
            b.iter(|| -> Result<GeneticAlgorithm<MockMyGene>, Box<dyn Error>> {
                GeneticAlgorithm::<MockMyGene>::new().init()
            })
        });
    }

    pub fn run(c: &mut Criterion) {
        c.bench_function("GeneticAlgorithm::run", |b| {
            b.iter(|| {
                GeneticAlgorithm::<MockMyGene>::new()
                    .population_size(black_box(100))
                    .iterations(black_box(1000))
                    .mutation_rate(black_box(0.05))
                    .selection_rate(black_box(0.90))
                    .selection_algorithm(Box::new(SelectionAlgorithms::Tournament(black_box(2))))
                    .fitness_goal(black_box(f64::MAX))
                    .init()
                    .unwrap()
                    .run()
            })
        });
    }
}

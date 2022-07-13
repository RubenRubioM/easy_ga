pub mod benchmark {
    use criterion::Criterion;
    use easy_ga::selection::*;
    use rand::Rng;

    fn generate_fitnesses(length: usize) -> Vec<f64> {
        let mut rng = rand::thread_rng();
        let mut fitnesses: Vec<f64> = Vec::with_capacity(length);
        for i in 0..fitnesses.capacity() {
            fitnesses.insert(i, rng.gen());
        }
        fitnesses
    }

    /// Benchmarks the SelectionAlgorithms::Roulette with different sizes.
    pub fn roulette_different_sizes(c: &mut Criterion) {
        let selection_algorithm = SelectionAlgorithms::Roulette;
        let mut group = c.benchmark_group("SelectionAlgorithms::Roulette");
        let mut fitnesses: Vec<f64> = generate_fitnesses(50);

        group.bench_function("SelectionAlgorithms::Roulette - size/50", |b| {
            b.iter(|| {
                for _ in 0..100000 {
                    selection_algorithm.select(&fitnesses);
                }
            })
        });

        fitnesses = generate_fitnesses(100);
        group.bench_function("SelectionAlgorithms::Roulette - size/100", |b| {
            b.iter(|| {
                for _ in 0..100000 {
                    selection_algorithm.select(&fitnesses);
                }
            })
        });

        fitnesses = generate_fitnesses(250);
        group.bench_function("SelectionAlgorithms::Roulette - size/250", |b| {
            b.iter(|| {
                for _ in 0..100000 {
                    selection_algorithm.select(&fitnesses);
                }
            })
        });

        fitnesses = generate_fitnesses(500);
        group.bench_function("SelectionAlgorithms::Roulette - size/500", |b| {
            b.iter(|| {
                for _ in 0..100000 {
                    selection_algorithm.select(&fitnesses);
                }
            })
        });
    }

    /// Benchmarks the SelectionAlgorithms::Random with different sizes.
    pub fn random_different_sizes(c: &mut Criterion) {
        let selection_algorithm = SelectionAlgorithms::Random;
        let mut group = c.benchmark_group("SelectionAlgorithms::Random");
        let mut fitnesses: Vec<f64> = generate_fitnesses(50);

        group.bench_function("SelectionAlgorithms::Random - size/50", |b| {
            b.iter(|| {
                for _ in 0..100000 {
                    selection_algorithm.select(&fitnesses);
                }
            })
        });

        fitnesses = generate_fitnesses(100);
        group.bench_function("SelectionAlgorithms::Random - size/100", |b| {
            b.iter(|| {
                for _ in 0..100000 {
                    selection_algorithm.select(&fitnesses);
                }
            })
        });

        fitnesses = generate_fitnesses(250);
        group.bench_function("SelectionAlgorithms::Random - size/250", |b| {
            b.iter(|| {
                for _ in 0..100000 {
                    selection_algorithm.select(&fitnesses);
                }
            })
        });

        fitnesses = generate_fitnesses(500);
        group.bench_function("SelectionAlgorithms::Random - size/500", |b| {
            b.iter(|| {
                for _ in 0..100000 {
                    selection_algorithm.select(&fitnesses);
                }
            })
        });
    }

    /// Benchmarks the SelectionAlgorithms::Stochastic with different sizes.
    pub fn stochastic_different_sizes(c: &mut Criterion) {
        let selection_algorithm = SelectionAlgorithms::Stochastic;
        let mut group = c.benchmark_group("SelectionAlgorithms::Stochastic");
        let mut fitnesses: Vec<f64> = generate_fitnesses(50);

        group.bench_function("SelectionAlgorithms::Stochastic - size/50", |b| {
            b.iter(|| {
                for _ in 0..100000 {
                    selection_algorithm.select(&fitnesses);
                }
            })
        });

        fitnesses = generate_fitnesses(100);
        group.bench_function("SelectionAlgorithms::Stochastic - size/100", |b| {
            b.iter(|| {
                for _ in 0..100000 {
                    selection_algorithm.select(&fitnesses);
                }
            })
        });

        fitnesses = generate_fitnesses(250);
        group.bench_function("SelectionAlgorithms::Stochastic - size/250", |b| {
            b.iter(|| {
                for _ in 0..100000 {
                    selection_algorithm.select(&fitnesses);
                }
            })
        });

        fitnesses = generate_fitnesses(500);
        group.bench_function("SelectionAlgorithms::Stochastic - size/500", |b| {
            b.iter(|| {
                for _ in 0..100000 {
                    selection_algorithm.select(&fitnesses);
                }
            })
        });
    }

    /// Benchmarks the SelectionAlgorithms::Tournament with different sizes.
    pub fn tournament_different_sizes(c: &mut Criterion) {
        let selection_algorithm = SelectionAlgorithms::Tournament(10);
        let mut group = c.benchmark_group("SelectionAlgorithms::Tournament");
        let mut fitnesses: Vec<f64> = generate_fitnesses(50);

        group.bench_function("SelectionAlgorithms::Tournament - size/50", |b| {
            b.iter(|| {
                for _ in 0..100000 {
                    selection_algorithm.select(&fitnesses);
                }
            })
        });

        let selection_algorithm = SelectionAlgorithms::Tournament(10);
        fitnesses = generate_fitnesses(100);
        group.bench_function("SelectionAlgorithms::Tournament - size/100", |b| {
            b.iter(|| {
                for _ in 0..100000 {
                    selection_algorithm.select(&fitnesses);
                }
            })
        });

        let selection_algorithm = SelectionAlgorithms::Tournament(10);
        fitnesses = generate_fitnesses(250);
        group.bench_function("SelectionAlgorithms::Tournament - size/250", |b| {
            b.iter(|| {
                for _ in 0..100000 {
                    selection_algorithm.select(&fitnesses);
                }
            })
        });

        let selection_algorithm = SelectionAlgorithms::Tournament(10);
        fitnesses = generate_fitnesses(500);
        group.bench_function("SelectionAlgorithms::Tournament - size/500", |b| {
            b.iter(|| {
                for _ in 0..100000 {
                    selection_algorithm.select(&fitnesses);
                }
            })
        });
    }

    /// Benchmarks the SelectionAlgorithms::Tournament with different sizes.
    pub fn tournament_different_tournament_participant(c: &mut Criterion) {
        let selection_algorithm = SelectionAlgorithms::Tournament(2);
        let mut group = c.benchmark_group("SelectionAlgorithms::Tournament");
        let fitnesses: Vec<f64> = generate_fitnesses(500);

        group.bench_function("SelectionAlgorithms::Tournament(2)", |b| {
            b.iter(|| {
                for _ in 0..100000 {
                    selection_algorithm.select(&fitnesses);
                }
            })
        });

        let selection_algorithm = SelectionAlgorithms::Tournament(10);

        group.bench_function("SelectionAlgorithms::Tournament(10)", |b| {
            b.iter(|| {
                for _ in 0..100000 {
                    selection_algorithm.select(&fitnesses);
                }
            })
        });

        let selection_algorithm = SelectionAlgorithms::Tournament(25);

        group.bench_function("SelectionAlgorithms::Tournament(50)", |b| {
            b.iter(|| {
                for _ in 0..100000 {
                    selection_algorithm.select(&fitnesses);
                }
            })
        });

        let selection_algorithm = SelectionAlgorithms::Tournament(100);

        group.bench_function("SelectionAlgorithms::Tournament(100)", |b| {
            b.iter(|| {
                for _ in 0..100000 {
                    selection_algorithm.select(&fitnesses);
                }
            })
        });

        let selection_algorithm = SelectionAlgorithms::Tournament(250);

        group.bench_function("SelectionAlgorithms::Tournament(250)", |b| {
            b.iter(|| {
                for _ in 0..100000 {
                    selection_algorithm.select(&fitnesses);
                }
            })
        });

        let selection_algorithm = SelectionAlgorithms::Tournament(500);

        group.bench_function("SelectionAlgorithms::Tournament(500)", |b| {
            b.iter(|| {
                for _ in 0..100000 {
                    selection_algorithm.select(&fitnesses);
                }
            })
        });
    }

    /// Benchmarks Roullete vs Tournament.
    pub fn roulette_vs_tournament(c: &mut Criterion) {
        let mut group = c.benchmark_group("Roulette vs Tournament");

        let selection_algorithm = SelectionAlgorithms::Tournament(2);
        let mut rng = rand::thread_rng();
        let mut fitnesses_500: Vec<f64> = Vec::with_capacity(500);
        fitnesses_500.fill_with(|| rng.gen());

        group.bench_function("SelectionAlgorithms::Tournament(2)", |b| {
            b.iter(|| {
                for _ in 0..100000 {
                    selection_algorithm.select(&fitnesses_500);
                }
            })
        });

        let selection_algorithm = SelectionAlgorithms::Tournament(10);
        group.bench_function("SelectionAlgorithms::Tournament(10)", |b| {
            b.iter(|| {
                for _ in 0..100000 {
                    selection_algorithm.select(&fitnesses_500);
                }
            })
        });

        let selection_algorithm = SelectionAlgorithms::Tournament(25);
        group.bench_function("SelectionAlgorithms::Tournament(25)", |b| {
            b.iter(|| {
                for _ in 0..100000 {
                    selection_algorithm.select(&fitnesses_500);
                }
            })
        });

        let selection_algorithm = SelectionAlgorithms::Roulette;
        group.bench_function("SelectionAlgorithms::Roulette", |b| {
            b.iter(|| {
                for _ in 0..100000 {
                    selection_algorithm.select(&fitnesses_500);
                }
            })
        });
    }
}

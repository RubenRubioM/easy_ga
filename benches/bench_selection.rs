pub mod benchmark {
    use criterion::Criterion;
    use easy_ga::selection::*;
    use rand::Rng;

    /// Benchmarks the SelectionAlgorithms::Roulette with different sizes.
    pub fn roulette_different_sizes(c: &mut Criterion) {
        let selection_algorithm = SelectionAlgorithms::Roulette;
        let mut group = c.benchmark_group("SelectionAlgorithms::Roulette");
        let mut rng = rand::thread_rng();
        let mut fitnesses_50: Vec<f64> = Vec::with_capacity(50);
        fitnesses_50.fill_with(|| rng.gen());

        group.bench_function("SelectionAlgorithms::Roulette - size/50", |b| {
            b.iter(|| {
                for _ in 0..10000 {
                    selection_algorithm.select(&fitnesses_50);
                }
            })
        });

        let mut fitnesses_100: Vec<f64> = Vec::with_capacity(100);
        fitnesses_100.fill_with(|| rng.gen());

        group.bench_function("SelectionAlgorithms::Roulette - size/100", |b| {
            b.iter(|| {
                for _ in 0..10000 {
                    selection_algorithm.select(&fitnesses_100);
                }
            })
        });

        let mut fitnesses_250: Vec<f64> = Vec::with_capacity(250);
        fitnesses_250.fill_with(|| rng.gen());

        group.bench_function("SelectionAlgorithms::Roulette - size/250", |b| {
            b.iter(|| {
                for _ in 0..10000 {
                    selection_algorithm.select(&fitnesses_250);
                }
            })
        });

        let mut fitnesses_500: Vec<f64> = Vec::with_capacity(500);
        fitnesses_500.fill_with(|| rng.gen());

        group.bench_function("SelectionAlgorithms::Roulette - size/500", |b| {
            b.iter(|| {
                for _ in 0..10000 {
                    selection_algorithm.select(&fitnesses_500);
                }
            })
        });
    }

    /// Benchmarks the SelectionAlgorithms::Tournament with different sizes.
    pub fn tournament_different_sizes(c: &mut Criterion) {
        let selection_algorithm = SelectionAlgorithms::Tournament(10);
        let mut group = c.benchmark_group("SelectionAlgorithms::Tournament");
        let mut rng = rand::thread_rng();
        let mut fitnesses_50: Vec<f64> = Vec::with_capacity(50);
        fitnesses_50.fill_with(|| rng.gen());

        group.bench_function("SelectionAlgorithms::Tournament - size/50", |b| {
            b.iter(|| {
                for _ in 0..10000 {
                    selection_algorithm.select(&fitnesses_50);
                }
            })
        });

        let selection_algorithm = SelectionAlgorithms::Tournament(10);
        let mut fitnesses_100: Vec<f64> = Vec::with_capacity(100);
        fitnesses_100.fill_with(|| rng.gen());

        group.bench_function("SelectionAlgorithms::Tournament - size/100", |b| {
            b.iter(|| {
                for _ in 0..10000 {
                    selection_algorithm.select(&fitnesses_100);
                }
            })
        });

        let selection_algorithm = SelectionAlgorithms::Tournament(10);
        let mut fitnesses_250: Vec<f64> = Vec::with_capacity(250);
        fitnesses_250.fill_with(|| rng.gen());

        group.bench_function("SelectionAlgorithms::Tournament - size/250", |b| {
            b.iter(|| {
                for _ in 0..10000 {
                    selection_algorithm.select(&fitnesses_250);
                }
            })
        });

        let selection_algorithm = SelectionAlgorithms::Tournament(10);
        let mut fitnesses_500: Vec<f64> = Vec::with_capacity(500);
        fitnesses_500.fill_with(|| rng.gen());

        group.bench_function("SelectionAlgorithms::Tournament - size/500", |b| {
            b.iter(|| {
                for _ in 0..10000 {
                    selection_algorithm.select(&fitnesses_500);
                }
            })
        });
    }

    /// Benchmarks the SelectionAlgorithms::Tournament with different sizes.
    pub fn tournament_different_tournament_participant(c: &mut Criterion) {
        let selection_algorithm = SelectionAlgorithms::Tournament(2);
        let mut group = c.benchmark_group("SelectionAlgorithms::Tournament");
        let mut rng = rand::thread_rng();
        let mut fitnesses_500: Vec<f64> = Vec::with_capacity(500);
        fitnesses_500.fill_with(|| rng.gen());

        group.bench_function("SelectionAlgorithms::Tournament(2)", |b| {
            b.iter(|| {
                for _ in 0..10000 {
                    selection_algorithm.select(&fitnesses_500);
                }
            })
        });

        let selection_algorithm = SelectionAlgorithms::Tournament(10);

        group.bench_function("SelectionAlgorithms::Tournament(10)", |b| {
            b.iter(|| {
                for _ in 0..10000 {
                    selection_algorithm.select(&fitnesses_500);
                }
            })
        });

        let selection_algorithm = SelectionAlgorithms::Tournament(25);

        group.bench_function("SelectionAlgorithms::Tournament(50)", |b| {
            b.iter(|| {
                for _ in 0..10000 {
                    selection_algorithm.select(&fitnesses_500);
                }
            })
        });

        let selection_algorithm = SelectionAlgorithms::Tournament(100);

        group.bench_function("SelectionAlgorithms::Tournament(100)", |b| {
            b.iter(|| {
                for _ in 0..10000 {
                    selection_algorithm.select(&fitnesses_500);
                }
            })
        });

        let selection_algorithm = SelectionAlgorithms::Tournament(250);

        group.bench_function("SelectionAlgorithms::Tournament(250)", |b| {
            b.iter(|| {
                for _ in 0..10000 {
                    selection_algorithm.select(&fitnesses_500);
                }
            })
        });

        let selection_algorithm = SelectionAlgorithms::Tournament(500);

        group.bench_function("SelectionAlgorithms::Tournament(500)", |b| {
            b.iter(|| {
                for _ in 0..10000 {
                    selection_algorithm.select(&fitnesses_500);
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
                for _ in 0..10000 {
                    selection_algorithm.select(&fitnesses_500);
                }
            })
        });

        let selection_algorithm = SelectionAlgorithms::Tournament(10);
        group.bench_function("SelectionAlgorithms::Tournament(10)", |b| {
            b.iter(|| {
                for _ in 0..10000 {
                    selection_algorithm.select(&fitnesses_500);
                }
            })
        });

        let selection_algorithm = SelectionAlgorithms::Tournament(25);
        group.bench_function("SelectionAlgorithms::Tournament(25)", |b| {
            b.iter(|| {
                for _ in 0..10000 {
                    selection_algorithm.select(&fitnesses_500);
                }
            })
        });

        let selection_algorithm = SelectionAlgorithms::Roulette;
        group.bench_function("SelectionAlgorithms::Roulette", |b| {
            b.iter(|| {
                for _ in 0..10000 {
                    selection_algorithm.select(&fitnesses_500);
                }
            })
        });
    }
}

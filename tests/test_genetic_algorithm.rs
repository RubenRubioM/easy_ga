#![allow(non_snake_case)]

#[cfg(test)]
mod genetic_algorithm {
    use easy_ga::genetic_algorithm::StopCriteria;
    use easy_ga::Gene;
    use easy_ga::GeneticAlgorithm;
    use easy_ga::SelectionAlgorithms;

    use rand::Rng;

    #[derive(Clone, Copy, Default)]
    struct MockMyGene {
        pub x: f64,
        pub y: i32,
        fitness: f64,
    }

    impl Gene for MockMyGene {
        fn init() -> Self {
            let mut rng = rand::thread_rng();
            MockMyGene {
                x: rng.gen_range(0.0..100.0),
                y: rng.gen_range(0..100),
                fitness: 0.0,
            }
        }
        fn calculate_fitness(&mut self) -> f64 {
            self.fitness = self.x + self.y as f64;
            self.fitness
        }

        fn crossover(&self, other: &Self) -> Self {
            other.clone()
        }

        fn mutate(&mut self) {
            let mut rng = rand::thread_rng();
            self.x = rng.gen_range(0.0..100.0);
            self.y = rng.gen_range(0..100);
        }

        fn get_fitness(&self) -> f64 {
            self.fitness
        }
    }

    #[test]
    fn WhenNew_ThenEveryVariableIsInitializedSuccesfully() {
        let genetic_algorithm = GeneticAlgorithm::<MockMyGene>::new();

        assert_eq!(genetic_algorithm.get_population_size(), 100); // Default population_size = 100
        assert_eq!(genetic_algorithm.get_iterations(), 1000); // Default iterations = 1000
        assert_eq!(genetic_algorithm.get_current_iteration(), 0); // Default current_iteration = 0
        assert_eq!(genetic_algorithm.get_mutation_rate(), 0.05); // Default mutation_rate = 0.05
        assert_eq!(genetic_algorithm.get_selection_rate(), 0.90); // Default selection_rate = 0.90
        assert_eq!(genetic_algorithm.get_fitness_goal(), f64::MAX); // Default fitness_goal = f64::MAX
        assert_eq!(genetic_algorithm.is_running(), false); // Default running = false
    }

    #[test]
    fn WhenNewWithValues_ThenEveryVariableIsInitializedSuccesfully() {
        let (population_size, iterations, mutation_rate, selection_rate, fitness_goal) =
            (50_usize, 10000_u32, 0.10_f32, 0.80_f32, 1000_f64);
        let genetic_algorithm = GeneticAlgorithm::<MockMyGene>::new_with_values(
            population_size,
            iterations,
            mutation_rate,
            selection_rate,
            Box::new(SelectionAlgorithms::Tournament(10)),
            fitness_goal,
        );

        assert_eq!(genetic_algorithm.get_population_size(), population_size);
        assert_eq!(genetic_algorithm.get_iterations(), iterations);
        assert_eq!(genetic_algorithm.get_current_iteration(), 0);
        assert_eq!(genetic_algorithm.get_mutation_rate(), mutation_rate);
        assert_eq!(genetic_algorithm.get_selection_rate(), selection_rate);
        assert_eq!(genetic_algorithm.get_fitness_goal(), fitness_goal);
        assert_eq!(genetic_algorithm.is_running(), false);
    }

    #[test]
    fn WhenNewSettingValues_ThenEveryVariableIsInitializedSuccesfully() {
        let (population_size, iterations, mutation_rate, selection_rate, fitness_goal) =
            (50_usize, 10000_u32, 0.10_f32, 0.80_f32, 1000_f64);
        let genetic_algorithm = GeneticAlgorithm::<MockMyGene>::new()
            .population_size(population_size)
            .iterations(iterations)
            .mutation_rate(mutation_rate)
            .selection_rate(selection_rate)
            .fitness_goal(fitness_goal);

        assert_eq!(genetic_algorithm.get_population_size(), population_size);
        assert_eq!(genetic_algorithm.get_iterations(), iterations);
        assert_eq!(genetic_algorithm.get_current_iteration(), 0);
        assert_eq!(genetic_algorithm.get_mutation_rate(), mutation_rate);
        assert_eq!(genetic_algorithm.get_selection_rate(), selection_rate);
        assert_eq!(genetic_algorithm.get_fitness_goal(), fitness_goal);
        assert_eq!(genetic_algorithm.is_running(), false);
    }

    #[test]
    fn WhenSetPopulationSize_ThenSuccess() {
        let mut genetic_algorithm = GeneticAlgorithm::<MockMyGene>::new();
        let population_size_extend = 1000_usize;
        let population_size_reduce = 10_usize;

        genetic_algorithm = genetic_algorithm.population_size(population_size_extend);
        assert_eq!(
            genetic_algorithm.get_population_size(),
            population_size_extend
        );

        genetic_algorithm = genetic_algorithm.population_size(population_size_reduce);
        assert_eq!(
            genetic_algorithm.get_population_size(),
            population_size_reduce
        );
    }

    #[test]
    fn WhenSetIterations_ThenSuccess() {
        let mut genetic_algorithm = GeneticAlgorithm::<MockMyGene>::new();
        let iterations_add = 1000;
        let iterations_sub = 10;

        genetic_algorithm = genetic_algorithm.iterations(iterations_add);
        assert_eq!(genetic_algorithm.get_iterations(), iterations_add);

        genetic_algorithm = genetic_algorithm.iterations(iterations_sub);
        assert_eq!(genetic_algorithm.get_iterations(), iterations_sub);
    }

    #[test]
    fn WhenSetFitnessGoal_ThenSuccess() {
        let mut genetic_algorithm = GeneticAlgorithm::<MockMyGene>::new();
        let fitness_goal = 1000.0;

        genetic_algorithm = genetic_algorithm.fitness_goal(fitness_goal);
        assert_eq!(genetic_algorithm.get_fitness_goal(), fitness_goal);
    }

    #[test]
    fn WhenSetMutationRateWithCorrectValues_ThenSuccess() {
        let mut genetic_algorithm = GeneticAlgorithm::<MockMyGene>::new();
        let mutation_rate = 0.20;

        genetic_algorithm = genetic_algorithm.mutation_rate(mutation_rate);
        assert_eq!(genetic_algorithm.get_mutation_rate(), mutation_rate);
    }

    #[test]
    #[should_panic]
    fn WhenSetMutationRateWithOverflowValue_ThenPanic() {
        let genetic_algorithm = GeneticAlgorithm::<MockMyGene>::new();
        genetic_algorithm.mutation_rate(1.10);
    }

    #[test]
    #[should_panic]
    fn WhenSetMutationRateWithUnderflowValue_ThenPanic() {
        let genetic_algorithm = GeneticAlgorithm::<MockMyGene>::new();
        genetic_algorithm.mutation_rate(-0.10);
    }

    #[test]
    #[should_panic]
    fn WhenSetSelectionRateWithOverflowValue_ThenPanic() {
        let genetic_algorithm = GeneticAlgorithm::<MockMyGene>::new();
        genetic_algorithm.selection_rate(1.10);
    }

    #[test]
    #[should_panic]
    fn WhenSetSelectionRateWithUnderflowValue_ThenPanic() {
        let genetic_algorithm = GeneticAlgorithm::<MockMyGene>::new();
        genetic_algorithm.selection_rate(-0.10);
    }

    #[test]
    fn WhenInit_ThenSuccess() {
        let mut genetic_algorithm = GeneticAlgorithm::<MockMyGene>::new();

        assert_eq!(genetic_algorithm.is_running(), false);
        genetic_algorithm = genetic_algorithm.init().unwrap();
        assert_eq!(genetic_algorithm.is_running(), true);
    }

    #[test]
    fn WhenRun_ThenReachMaxIterations() {
        let genetic_algorithm = GeneticAlgorithm::<MockMyGene>::new().init().unwrap();

        let (_, stopCriteria) = genetic_algorithm.run();
        assert_eq!(stopCriteria, StopCriteria::MaxIterations);
    }

    #[test]
    fn WhenRun_ThenReachFitnessGoal() {
        let genetic_algorithm = GeneticAlgorithm::<MockMyGene>::new()
            .fitness_goal(10.0)
            .init()
            .unwrap();

        let (_, stopCriteria) = genetic_algorithm.run();
        assert_eq!(stopCriteria, StopCriteria::FitnessAchieved);
    }

    #[test]
    fn WhenRunWithDifferentSelectionAlgorithm_ThenSuccess() {
        let iterations = 10;
        let genetic_algorithm = GeneticAlgorithm::<MockMyGene>::new()
            .iterations(iterations)
            .selection_algorithm(Box::new(SelectionAlgorithms::Roulette))
            .init()
            .unwrap();

        let (_, stopCriteria) = genetic_algorithm.run();
        assert_eq!(stopCriteria, StopCriteria::MaxIterations);

        let population_size = 100;

        // Test Tournament(1)
        let genetic_algorithm = GeneticAlgorithm::<MockMyGene>::new()
            .iterations(iterations)
            .population_size(population_size)
            .selection_algorithm(Box::new(SelectionAlgorithms::Tournament(1)))
            .init()
            .unwrap();

        let (_, stopCriteria) = genetic_algorithm.run();
        assert_eq!(stopCriteria, StopCriteria::MaxIterations);

        // Test Tournament(half population)
        let genetic_algorithm = GeneticAlgorithm::<MockMyGene>::new()
            .iterations(iterations)
            .population_size(population_size)
            .selection_algorithm(Box::new(SelectionAlgorithms::Tournament(
                population_size / 2,
            )))
            .init()
            .unwrap();

        let (_, stopCriteria) = genetic_algorithm.run();
        assert_eq!(stopCriteria, StopCriteria::MaxIterations);

        // Test Tournament(population_size)
        let genetic_algorithm = GeneticAlgorithm::<MockMyGene>::new()
            .iterations(iterations)
            .population_size(population_size)
            .selection_algorithm(Box::new(SelectionAlgorithms::Tournament(population_size)))
            .init()
            .unwrap();

        let (_, stopCriteria) = genetic_algorithm.run();
        assert_eq!(stopCriteria, StopCriteria::MaxIterations);
    }
}

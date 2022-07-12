#![allow(non_snake_case)]

#[cfg(test)]
mod selection {
    use easy_ga::Selection;
    use easy_ga::SelectionAlgorithms;
    use rand::Rng;

    #[test]
    /// Tests the Selection::Select with SelectionAlgorithm::Roulette
    fn WhenSelectionWithRoulette_ThenSuccess() {
        // Fitness length = 0
        let selection_algorithm = SelectionAlgorithms::Roulette;
        selection_algorithm.select(&generate_fitnesses(0));
        // Fitness length = 1
        selection_algorithm.select(&generate_fitnesses(1));
        // Fitness length = 2
        selection_algorithm.select(&generate_fitnesses(2));
        // Fitness length = 100
        selection_algorithm.select(&generate_fitnesses(100));
    }

    #[test]
    /// Tests the Selection::Select with SelectionAlgorithm::Roulette
    fn WhenSelectionWithTournament_ThenSuccess() {
        // Fitness length = 0
        let selection_algorithm = SelectionAlgorithms::Tournament(0);
        selection_algorithm.select(&generate_fitnesses(0));
        // Fitness length = 1
        let selection_algorithm = SelectionAlgorithms::Tournament(1);
        selection_algorithm.select(&generate_fitnesses(1));
        // Fitness length = 2
        let selection_algorithm = SelectionAlgorithms::Tournament(2);
        selection_algorithm.select(&generate_fitnesses(2));
        // Fitness length = 100
        let selection_algorithm = SelectionAlgorithms::Tournament(100);
        selection_algorithm.select(&generate_fitnesses(100));
    }

    #[test]
    /// Tests the Selection::Select with SelectionAlgorithm::Random
    fn WhenSelectionWithRandom_ThenSuccess() {
        // Fitness length = 0
        let selection_algorithm = SelectionAlgorithms::Random;
        selection_algorithm.select(&generate_fitnesses(0));
        // Fitness length = 1
        selection_algorithm.select(&generate_fitnesses(1));
        // Fitness length = 2
        selection_algorithm.select(&generate_fitnesses(2));
        // Fitness length = 100
        selection_algorithm.select(&generate_fitnesses(100));
    }

    fn generate_fitnesses(length: usize) -> Vec<f64> {
        let mut rng = rand::thread_rng();
        let mut fitnesses: Vec<f64> = Vec::with_capacity(length);
        for i in 0..fitnesses.capacity() {
            fitnesses.insert(i, rng.gen());
        }
        fitnesses
    }
}
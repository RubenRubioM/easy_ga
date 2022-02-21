use easy_ga::Gene;
use easy_ga::GeneticAlgorithm;
use easy_ga::SelectionAlgorithms;
use rand::Rng;

#[derive(Clone, Copy, Default)]
struct MyGene {
    pub x: f64,
    pub y: i32,
    fitness: f64,
}

impl Gene for MyGene {
    fn init() -> Self {
        let mut rng = rand::thread_rng();
        MyGene {
            x: rng.gen_range(0.0..100.0),
            y: rng.gen_range(0..100),
            fitness: 0.0,
        }
    }

    fn calculate_fitness(&mut self) -> f64 {
        self.fitness = self.x + self.y as f64;
        self.fitness
    }


    // TODO: Implement good crossover.
    fn crossover(&self, other: &Self) -> Self {
        other.clone()
    }

    // TODO: Implement good mutate.
    fn mutate(&mut self) {
        let mut rng = rand::thread_rng();
        self.x = rng.gen_range(0.0..100.0);
        self.y = rng.gen_range(0..100);
    }

    fn get_fitness(&self) -> f64 {
        self.fitness
    }
}


fn main() {
    let genetic_algorithm = GeneticAlgorithm::<MyGene>::new()
                                                        .population_size(20)
                                                        .iterations(50)
                                                        .mutation_rate(0.10)
                                                        .selection_rate(0.90)
                                                        .selection_algorithm(Box::new(SelectionAlgorithms::Tournament(10)))
                                                        .fitness_goal(100.0)
                                                        .init().unwrap();

    let (gene, stop) = genetic_algorithm.run();

    println!("Best gene stopped because {:?} with {}", stop, gene.get_fitness());
    println!("Easy_GA");
}
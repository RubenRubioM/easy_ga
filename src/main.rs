use easy_ga::samples;
use easy_ga::Gene;
use easy_ga::GeneticAlgorithm;
use easy_ga::LOG_verbosity;
use easy_ga::LOG_verbosity_type;
use easy_ga::SelectionAlgorithms;
use easy_ga::VerbosityLevel;
use easy_ga::VerbosityType;

fn main() {
    LOG_verbosity(VerbosityLevel::LOW);
    LOG_verbosity_type(VerbosityType::LOG_AND_SAVE);

    let genetic_algorithm = GeneticAlgorithm::<samples::MyGene>::new()
        .population_size(100)
        .iterations(1000)
        .mutation_rate(0.10)
        .selection_rate(0.90)
        .selection_algorithm(Box::new(SelectionAlgorithms::Tournament(10)))
        .fitness_goal(f64::MAX)
        .init()
        .unwrap();

    let (gene, stop) = genetic_algorithm.run();

    println!(
        "Best gene stopped because {:?} with {}",
        stop,
        gene.get_fitness()
    );
    println!("Easy_GA");
}

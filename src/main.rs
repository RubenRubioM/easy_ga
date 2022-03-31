use easy_ga::samples;
use easy_ga::Gene;
use easy_ga::GeneticAlgorithm;
use easy_ga::SelectionAlgorithms;

fn main() {
    let genetic_algorithm = GeneticAlgorithm::<samples::MyGene>::new()
        .population_size(20)
        .iterations(50)
        .mutation_rate(0.10)
        .selection_rate(0.90)
        .selection_algorithm(Box::new(SelectionAlgorithms::Tournament(10)))
        .fitness_goal(100.0)
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

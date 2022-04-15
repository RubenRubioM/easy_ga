//! This module contains the definition and implementation of the GeneticAlgorithm class
//! wich is the handler for our `Gene` to do the logic.

use core::fmt;
use rand::Rng;

use crate::logger;
use crate::selection::*;
use crate::Gene;

/// Default value for our population size.
const POPULATION_SIZE_DEFAULT: usize = 100;
/// Default value for our max generations aka iterations.
const MAX_ITERATIONS_DEFAULT: u32 = 1000;
/// Default value for mutation probability to perform mutation on the genes.
const MUTATION_RATE_DEFAULT: f32 = 0.05;
/// Default value percentage of individuals to survive to the next generation.
const SELECTION_RATE_DEFAULT: f32 = 0.90;

/// Reasons to stop the algorithm.
#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub enum StopCriteria {
    MaxIterations,
    FitnessAchieved,
    Unknown,
}

/// Struct for our genetic algorithm handler.
pub struct GeneticAlgorithm<T: Gene + Copy> {
    /// Size of the population, wich means the amount of `Gene`'s our generation can handle.
    population_size: usize,
    /// Num of the max iterations our algorithm will perform.
    iterations: u32,
    /// The current iteration the algorithm is.
    current_iteration: u32,
    /// The current generation.
    generation: Vec<T>,
    /// Historic with all our past generations.
    generation_historic: Vec<Vec<T>>,
    /// The mutation percentage.
    mutation_rate: f32,
    /// The percentage of individuals to survive to the next generation.
    ///
    /// # Example
    /// If we have a `population_size` of 100 genes and a `selection_rate` of 0.9. Then, 90 genes will advance to the new generation and 10 will be generated via crossover.
    selection_rate: f32,
    /// The selection algorithm to perform the Selection::select.
    selection_algorithm: Box<dyn Selection>,
    /// The fitness value to reach to end the algorithm.
    fitness_goal: f64,
    /// If the algorithm is running or not
    running: bool,
    /// The best gene overall.
    best_gene: T,
    /// The stop reason if the algorithm has stopped.
    stop_criteria: StopCriteria,
}

impl<T: Gene + Copy> GeneticAlgorithm<T> {
    /// Creates a new `GeneticAlgorithm` with default values.
    /// * `population_size` = 100
    /// * `iterations` = 1000
    /// * `mutation_rate` = 0.05
    /// * `selection_rate` = 0.90
    /// * `selection_algorithm` = SelectionAlgorithms::Tournament(2)
    /// * `fitness_goal` = f64::MAX
    pub fn new() -> Self {
        let generation = (0..POPULATION_SIZE_DEFAULT).map(|_| T::init()).collect();
        let generation_historic = vec![Vec::clone(&generation)];
        let return_value = GeneticAlgorithm {
            population_size: POPULATION_SIZE_DEFAULT,
            iterations: MAX_ITERATIONS_DEFAULT,
            current_iteration: 0,
            generation,
            generation_historic,
            mutation_rate: MUTATION_RATE_DEFAULT,
            selection_rate: SELECTION_RATE_DEFAULT,
            selection_algorithm: Box::new(SelectionAlgorithms::Tournament(2)),
            fitness_goal: f64::MAX,
            running: false,
            best_gene: T::init(),
            stop_criteria: StopCriteria::Unknown,
        };

        logger::LOG(
            logger::VerbosityLevel::LOW,
            format!(
                "GeneticAlgorithm created with default values:\n{}",
                return_value
            )
            .as_str(),
        );

        return_value
    }

    /// Creates a new `GeneticAlgorithm` with specific values.
    ///
    /// # Arguments
    ///
    /// * `population_size` - The population of the generation.
    /// * `iterations` - The number of max iterations.
    /// * `mutation_rate` - The mutation probability for a gene.
    /// * `selection_rate` - The percentage of individuals to survive to the next generation.
    /// * `selection_algorithm` - Box object with a Selection trait to perform the selection algorithm.
    /// * `fitness_goal` - The fitness value target.
    pub fn new_with_values(
        population_size: usize,
        iterations: u32,
        mutation_rate: f32,
        selection_rate: f32,
        selection_algorithm: Box<dyn Selection>,
        fitness_goal: f64,
    ) -> Self {
        if mutation_rate > 1.0 || mutation_rate < 0.0 {
            panic!("Mutation rate not in rage between 0.0 and 1.0");
        }

        if selection_rate > 1.0 || selection_rate < 0.0 {
            panic!("Selection rate not in rage between 0.0 and 1.0");
        }

        let generation = (0..population_size).map(|_| T::init()).collect();
        let generation_historic = vec![vec![]];
        let return_value = GeneticAlgorithm {
            population_size,
            iterations,
            current_iteration: 0,
            generation,
            generation_historic,
            mutation_rate,
            selection_rate,
            selection_algorithm,
            fitness_goal,
            running: false,
            best_gene: T::init(),
            stop_criteria: StopCriteria::Unknown,
        };

        logger::LOG(
            logger::VerbosityLevel::LOW,
            format!("GeneticAlgorithm created with values:\n{}", return_value).as_str(),
        );

        return_value
    }

    // TODO: Implement error handling when the algorithm will not be able to init.
    /// Initiate the algorithm.    
    pub fn init(mut self) -> Self {
        self.running = true;
        self.save_generation();
        logger::LOG(
            logger::VerbosityLevel::HIGH,
            "Algorithm initiated properlly.",
        );

        self
    }

    /// Runs the algorithm by itself without user control.
    pub fn run(&mut self) -> (T, StopCriteria) {
        if !self.is_running() {
            panic!("GeneticAlgorithm not initializated. Call GeneticAlgorithm::init() before.");
        }

        logger::LOG(logger::VerbosityLevel::HIGH, "Algorithm run started.");

        while self.running {
            self.next_iteration();
        }

        (self.best_gene, self.stop_criteria)
    }

    /// Goes iteration by iteration in case the user wants to have more control over the lifetime of the algorithm.
    ///
    /// # Returns
    ///
    /// `self.generation` - The new generation.
    pub fn next_iteration(&mut self) -> &Vec<T> {
        if !self.is_running() {
            panic!("GeneticAlgorithm not initializated. Call GeneticAlgorithm::init() before.");
        }
        
        logger::LOG(
            logger::VerbosityLevel::LOW,
            format!(
                ">>>>>>> Started iteration {} <<<<<<<",
                self.current_iteration
            )
            .as_str(),
        );

        logger::LOG(
            logger::VerbosityLevel::HIGH,
            ">> Fitness calculation phase.",
        );
        // Calculate fitness.
        for (i, gene) in self.generation.iter_mut().enumerate() {
            gene.calculate_fitness();
            logger::LOG(
                logger::VerbosityLevel::MID,
                format!("Gene {i} = {:?}", gene.get_fitness()).as_str(),
            );
        }

        logger::LOG(logger::VerbosityLevel::HIGH, ">> Selection phase.");
        // Selection.
        let mut new_generation: Vec<T> = Vec::with_capacity(self.population_size);
        let num_survivors: usize = (self.generation.len() as f32 * self.selection_rate) as usize;
        let mut new_genes_num: usize = 0;

        logger::LOG(
            logger::VerbosityLevel::MID,
            format!("Number of survivors = {:?}", num_survivors).as_str(),
        );
        while new_genes_num < num_survivors {
            let gene_idx: usize = self.selection_algorithm.select(
                &self
                    .generation
                    .iter()
                    .map(|gene| gene.get_fitness())
                    .collect(),
            );
            new_generation.push(self.generation[gene_idx]);
            self.generation.remove(gene_idx);
            new_genes_num += 1;
        }

        logger::LOG(logger::VerbosityLevel::HIGH, ">> Crossover phase.");
        // Crossover
        let mut rng = rand::thread_rng();
        while new_genes_num < self.population_size {
            let gen1_idx = rng.gen_range(0..new_generation.len());
            let mut gen2_idx = gen1_idx;
            while gen2_idx == gen1_idx {
                gen2_idx = rng.gen_range(0..new_generation.len());
            }
            let gen1 = new_generation[gen1_idx];
            let crossover_gen = gen1.crossover(&new_generation[gen2_idx]);
            new_generation.push(crossover_gen);
            new_genes_num += 1;
            logger::LOG(
                logger::VerbosityLevel::MID,
                format!(
                    "Crossover between gen1({:?}) and gen2({:?})",
                    gen1.get_fitness(),
                    new_generation[gen2_idx].get_fitness()
                )
                .as_str(),
            );
        }

        logger::LOG(logger::VerbosityLevel::HIGH, ">> Mutation phase.");
        // Mutation
        let mut num_of_mutations = 0;
        for gen in new_generation.iter_mut() {
            if rng.gen_range(0.0..1.0) < self.mutation_rate {
                gen.mutate();
                num_of_mutations += 1;
            }
        }
        logger::LOG(
            logger::VerbosityLevel::MID,
            format!("{} mutations performed.", num_of_mutations).as_str(),
        );

        // Save generation_historic & best_gene
        self.generation = new_generation;
        self.save_generation();

        self.current_iteration += 1;

        // Check stop criteria
        self.check_stop_criteria();

        &self.generation
    }

    /// Saves the generation just created and the best gene.
    fn save_generation(&mut self) {
        logger::LOG(logger::VerbosityLevel::HIGH, ">> Saving generation data.");
        self.generation_historic.push(Vec::clone(&self.generation));

        let mut best_fitness: f64 = self.best_gene.get_fitness();

        for gene in self.generation.iter() {
            if gene.get_fitness() > best_fitness {
                self.best_gene = *gene;
                best_fitness = gene.get_fitness();
            }
        }
        logger::LOG(
            logger::VerbosityLevel::LOW,
            format!("Best gene with fitness = {:?}", best_fitness).as_str(),
        );
    }

    /// Checks if the algorithm should stop or not.
    fn check_stop_criteria(&mut self) {
        if self.best_gene.get_fitness() >= self.fitness_goal {
            self.running = false;
            self.stop_criteria = StopCriteria::FitnessAchieved;
            logger::LOG(
                logger::VerbosityLevel::LOW,
                format!("Algorithm must stop because of {:?}", self.stop_criteria).as_str(),
            );
        }

        if self.current_iteration >= self.iterations {
            self.running = false;
            self.stop_criteria = StopCriteria::MaxIterations;
            logger::LOG(
                logger::VerbosityLevel::LOW,
                format!("Algorithm must stop because of {:?}", self.stop_criteria).as_str(),
            );
        }
    }

    /// Sets the population size.
    ///
    /// # Notes
    ///
    /// If the population size is greather than the actual size, the generation ir resized filling the empty values with new genes of value `T`.
    pub fn population_size(mut self, population_size: usize) -> Self {
        if population_size >= self.population_size {
            self.generation
                .extend((0..population_size - self.generation.len()).map(|_| T::init()));
        } else {
            self.generation.resize(population_size, T::init());
        }
        self.population_size = population_size;
        self
    }

    /// Sets the max number of iterations the algorithm will perform.
    ///
    /// # Notes
    ///
    /// If the max number passed is lower than the current generation, the algorithm will end.
    pub fn iterations(mut self, iterations: u32) -> Self {
        if iterations <= self.current_iteration {
            panic!("Number of iterations is not greater  than the actual iteration.");
        }

        self.iterations = iterations;
        self
    }

    /// Sets the mutation rate.
    pub fn mutation_rate(mut self, mutation_rate: f32) -> Self {
        if mutation_rate > 1.0 || mutation_rate < 0.0 {
            panic!("Mutation rate not in rage between 0.0 and 1.0");
        }

        self.mutation_rate = mutation_rate;
        self
    }

    /// Sets the selection rate.
    pub fn selection_rate(mut self, selection_rate: f32) -> Self {
        if selection_rate > 1.0 || selection_rate < 0.0 {
            panic!("Selection rate not in rage between 0.0 and 1.0");
        }

        self.selection_rate = selection_rate;
        self
    }

    /// Sets the selection algorithm.
    pub fn selection_algorithm(mut self, selection_algorithm: Box<dyn Selection>) -> Self {
        self.selection_algorithm = selection_algorithm;
        self
    }

    /// Sets the fitness goal to reach and stop the algorithm.
    pub fn fitness_goal(mut self, fitness_goal: f64) -> Self {
        self.fitness_goal = fitness_goal;
        self
    }

    /// Returns the population size.
    pub fn get_population_size(&self) -> usize {
        self.population_size
    }

    /// Returns the max iterations.
    pub fn get_iterations(&self) -> u32 {
        self.iterations
    }

    /// Returns the current iteration.
    pub fn get_current_iteration(&self) -> u32 {
        self.current_iteration
    }

    /// Returns the current generation.
    pub fn get_generation(&self) -> Vec<T> {
        self.generation.clone()
    }

    /// Returns all the generations the algorithm passed.
    pub fn get_generation_historic(&self) -> Vec<Vec<T>> {
        self.generation_historic.clone()
    }

    /// Returns the mutation rate.
    pub fn get_mutation_rate(&self) -> f32 {
        self.mutation_rate
    }

    /// Returns the selection rate.
    pub fn get_selection_rate(&self) -> f32 {
        self.selection_rate
    }

    pub fn get_fitness_goal(&self) -> f64 {
        self.fitness_goal
    }

    /// Returns the best gene in all the generations.
    pub fn get_best_gene(&self) -> T {
        self.best_gene
    }

    /// Returns if the algorithm is currently running
    pub fn is_running(&self) -> bool {
        self.running
    }

    pub fn get_stop_criteria(&self) -> StopCriteria {
        self.stop_criteria
    }
}

/// Default trait implementation for GeneticAlgorithm.
impl<T: Gene + Copy> Default for GeneticAlgorithm<T> {
    fn default() -> Self {
        Self::new()
    }
}

/// Display trait implementation for GeneticAlgorithm.
impl<T: Gene + Copy> fmt::Display for GeneticAlgorithm<T> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
             "{{\n\tpopulation_size: {},\n\titerations: {},\n\tcurrent_iteration: {},\n\tselection_rate: {},\n\tmutation_rate: {},\n\tfitness_goal: {:?},\n\tstop_criteria: {:?},\n\tbest_gene_fitness: {}\n}}",
            self.population_size,
            self.iterations,
            self.current_iteration,
            self.selection_rate,
            self.mutation_rate,
            self.fitness_goal,
            self.stop_criteria,
            self.best_gene.get_fitness()
        )
    }
}

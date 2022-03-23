//! This module contains the definition of a Gene trait.

/// This trait defines how a Gene has to be implemented.
pub trait Gene {
    /// Initialize the parameters of the gene. It is used inside the `easy_ga::GeneticAlgorithm`.
    ///
    /// # Returns
    ///
    /// * `gene` as `Gene` - The Gene generated.
    fn init() -> Self;

    /// Do the logic to calculate the fitness of the gene and return the value
    ///
    /// # Returns
    ///
    /// * `fitness_value` as `f64` - The fitness value of this gene.
    fn calculate_fitness(&mut self) -> f64;

    /// Does the crossover logic to mix the gene with another one pass by parameter and returns the generated gene.
    ///
    /// # Parameters
    ///
    /// * `other` as `Gene` - The other gene to do the crossover.
    ///
    /// # Returns
    ///
    /// * `gene` as `Gene` - The gene result of doing the crossover between `self` and `other`.
    fn crossover(&self, other: &Self) -> Self;

    /// Mutates the gene to alter its values.
    fn mutate(&mut self);

    /// Returns the fitness of the gene to avoid calculate it everytime we want to check the value.
    ///
    /// # Returns
    ///
    /// * `fitness_value` as `f64` - The fitness value of the gene.
    fn get_fitness(&self) -> f64;
}

//! Library to implement genetic algorithms

pub mod gene;
pub mod genetic_algorithm;
pub mod selection;

pub use gene::Gene;
pub use genetic_algorithm::GeneticAlgorithm;
pub use selection::*;

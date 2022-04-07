//! Library to implement genetic algorithms

#[macro_use]
extern crate lazy_static;

pub mod gene;
pub mod genetic_algorithm;
mod logger;
pub mod samples;
pub mod selection;

pub use gene::Gene;
pub use genetic_algorithm::GeneticAlgorithm;
pub use logger::LOG_verbosity;
pub use logger::LOG_verbosity_type;
pub use logger::VerbosityLevel;
pub use logger::VerbosityType;
pub use selection::*;

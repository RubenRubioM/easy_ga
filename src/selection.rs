//! This module define the trait and implementation of selection algorithms

use rand::distributions::Standard;
use rand::Rng;
use std::cmp;
use std::collections::HashSet;

pub trait Selection {
    /// Select the gene index to pass to the next generation.
    ///
    /// # Arguments
    ///
    /// * `fitnesses` as `&Vec<f64>` - Vector with the fitnesses values of our generation.
    ///
    /// # Returns
    ///
    /// * `index` as `usize` - The index in our generation array to avance to the next generation.
    ///
    /// # Examples
    ///
    /// let v: Vec<f64> = [10.5, 20.0, 100.0];
    /// let idx: usize = Selection::select(&v);
    /// println!("Value {} is selected", v[idx]);
    fn select(&self, fitnesses: &Vec<f64>) -> usize;
}

/// Default selection algorithms supported.
#[derive(Debug)]
pub enum SelectionAlgorithms {
    Roulette,
    Tournament(usize),
}

impl Selection for SelectionAlgorithms {
    //FIXME: This should return a vector and not an index because this way we avoid a lot of calculations
    fn select(&self, fitnesses: &Vec<f64>) -> usize {
        let mut rng = rand::thread_rng();
        let mut winner_idx: usize = 0;

        match self {
            SelectionAlgorithms::Roulette => {
                let total_fitness: f64 = fitnesses.iter().sum();
                let mut probabilities: Vec<f64> = Vec::with_capacity(fitnesses.len());

                for fitness_value in fitnesses.iter() {
                    probabilities.push(fitness_value / total_fitness);
                }

                let value: f64 = rng.sample(Standard);
                let mut fitness_accum = 0.0;

                for (idx, probability) in probabilities.iter().enumerate() {
                    if value <= fitness_accum + probability {
                        return idx;
                    }
                    fitness_accum += probability;
                }
            }

            SelectionAlgorithms::Tournament(members) => {
                // We use a HashSet to avoid duplicated values.
                let tournament_size = cmp::min(*members, fitnesses.len());
                let mut set: HashSet<usize> = HashSet::with_capacity(tournament_size); // In case the population is lower than the number of members.

                while set.len() < tournament_size {
                    let idx = rng.gen_range(0..fitnesses.len());
                    set.insert(idx);
                }

                let mut best_fitness = f64::MIN;
                for idx in set {
                    if fitnesses[idx] > best_fitness {
                        best_fitness = fitnesses[idx];
                        winner_idx = idx;
                    }
                }
            }
        }
        winner_idx
    }
}

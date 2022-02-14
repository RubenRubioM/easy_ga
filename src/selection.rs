//! This module define the trait and implementation of selection algorithms

use std::collections::HashSet;
use rand::Rng;

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
    fn select(&self, fitnesses: Vec<f64>) -> usize;
}

pub enum SelectionAlgorithms {
    Roulette(usize),
    Tournament(usize),
}

impl Selection for SelectionAlgorithms {
    fn select(&self, fitnesses: Vec<f64>) -> usize {
        let mut rng = rand::thread_rng();
        let mut winner_idx: usize = 0;

        match self {
            SelectionAlgorithms::Roulette(sections) => {
                // TODO: Implement roulette algorithm.
                todo!();
            }

            SelectionAlgorithms::Tournament(members) => {
                // We use a HashSet to avoid duplicated values.
                let mut set: HashSet<usize> = HashSet::with_capacity(*members);

                while set.len() < *members {
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
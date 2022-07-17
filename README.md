# Easy_GA
[![Current Crates.io Version](https://img.shields.io/crates/v/easy_ga.svg)](https://crates.io/crates/easy_ga)

Easy_GA is a genetic algorithm library made for Rust projects. It provides full customization for your own genotypes definitions and a geneticAlgorithm implementation to wrap all the common logic within a genetic algorithm.

All the changes will be updated in the [CHANGELOG.md](CHANGELOG.md)

## Features
- `trait Gene`: Definition to implement for your custom genotypes.
- `trait Selection`: Definition for your custom selection algorithms.
    * `Roulette`: Selection algorithm already implemented.
    * `Tournament`: Selection algorithm implementation with `n` members on it.
    * `Random`: Selection algorithm already implemented.
    * `Stochastic`: Selection algorithm already implemented.
- `GeneticAlgorithm`: The main class to wrap the business logic in the genetic algorithm execution.

## Usage

In your `Cargo.tml` you have to add the `Easy_GA` dependency
```rust
[dependencies]
easy_ga = "*"
```
---
Now I will show you a basic example of `Easy_GA` that you can find on [main.rs](src/main.rs)

Files to include in order to use features:

```rust
use easy_ga::Gene; // For defining our own gene.
use easy_ga::GeneticAlgorithm; // To create a GeneticAlgorithm.
use easy_ga::SelectionAlgorithms; // To specity a concrete SelectionAlgorithm.
```
---
Definition of a custom Gene implementing `easy_ga::Gene` trait:

```rust
#[derive(Clone, Copy)]
struct MyGene {
    // Fields.
    fitness: f64 // Recomended to avoid recalculate fitness on `get_fitness`
}

impl Gene for MyGene {
    fn init() -> Self {
        // Gene constructor.
    }

    fn calculate_fitness(&mut self) -> f64 {
        // Fitness function.
    }

    fn crossover(&self, other: &Self) -> Self {
        // Crossover implementation.
    }

    fn mutate(&mut self) {
        // Mutation implementation.
    }

    fn get_fitness(&self) -> f64 {
        // Returns the fitness
    }
}
```
At this moment, we need to implement the `Clone` & `Copy` traits for our `Gene`. I will try to avoid that in a future versions.

---

Initialization of our `GeneticAlgorithm`:

```rust
let genetic_algorithm = GeneticAlgorithm::<MyGene>::new()
            .population_size(20)
            .iterations(50)
            .mutation_rate(0.10)
            .selection_rate(0.90)
            .selection_algorithm(Box::new(SelectionAlgorithms::Tournament(10)))
            .fitness_goal(100.0)
            .init().unwrap();
```
We have other ways to initializate our `GeneticAlgorithm` such as `GeneticAlgorithm::new_with_values` if we don't want the chain calling method.

---

Now that we have defined our genotype and have initializate our `GeneticAlgorhtm` we have 2 ways of running it:

- __`GeneticAlgorithm::run`__: This method runs the algorithm until the end and returns a tuple with (`Gene`, `StopCriteria`) that represents the best `Gene` in the execution and the reason to stop the execution.
```rust
let (gene, stop_criteria) = genetic_algorithm.run();
```
- __Iteration by iteration__: We have the posibilty of running the algorithm generation by generation and make modification while the execution is running.
```rust
while genetic_algorithm.is_running() {
    let new_generation: &Vec<MyGene> = genetic_algorithm.next_iteration();
}
```
---
## Logger

The logger is a very usefull tool to measure and retrieve some data from the execution. By default the logger is disabled, you can enable it this way:

```rust
use easy_ga::VerbosityLevel; // Verbosity level {DISABLED, LOW, MID, HIGH}
use easy_ga::VerbosityType; // Verbosity type {LOG, SAVE, LOG_AND_SAVE}
use easy_ga::LOG_verbosity; // Sets the verbosity level.
use easy_ga::LOG_verbosity_type; // Sets the verbosity type.

LOG_verbosity(VerbosityLevel::LOW); // VerbosityLevel::DISABLED by default
LOG_verbosity_type(VerbosityType::LOG_AND_SAVE); // VerbosityType::LOG by default
```

- **VerbosityLevel:**
  - **DISABLED**: The logs are disabled.
  - **LOW**: Only very usefull information.
  - **MID**: Maybe not to desired information but also usefull.
  - **HIGH**: All logs are avaliable including tracing logs.

- **VerbosityType:**
  - **LOG**: Only terminal logs.
  - **SAVE**: Saves the logs into [target/easy_ga/logs/](target/easy_ga/logs/).
  - **SAVE_AND_LOG**: Both.

---

## Benchmarking

Benchmarking was added in the version `1.1.0` and you can run them donwloading the repository and running `cargo bench` from the command-line. The benchmarks are placed inside the [benches/](benches/) folder. 

---

## Next steps

This is a personal side project mainly for me so any further implementations will be done in my spare time as a good way to teach me more about Rust.

- Multithreading
- Add verbosity for debugging ✅
- More unit testing and system testing
- New default `Selection` algorithms
- CSV and JSON result export
- Fix some quality of life problems with references and chain calling
- Add benchmarks ✅

## License

Easy_GA is licensed under Mozilla Public License 2.0.

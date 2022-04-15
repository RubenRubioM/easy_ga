# [1.2.0]

* JSON summary exporting for `GeneticAlgorithm`.
* CSV summary exporting for `GeneticAlgorithm`.
* GeneticAlgorithm::run() don't call init() anymore, instead it panics if it is not initializated.

# [1.1.0]

* Added benchmarks. You can run benchmarks using `cargo bench`. The benches are all inside `benches/`.
* Added `logger` feature to get information about the execution.
* Added `logger::VerbosityLevel` to filter the output.
* Added `logger::VerbosityType` to choose between only LOG, SAVE and LOG_AND_SAVE. 
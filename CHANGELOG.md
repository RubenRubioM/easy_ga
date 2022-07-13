# [1.2.0]

* Added new `SelectionAlgorithm::Random`.
* Added new `SelectionAlgorithm::Stochastic`.
* `SelectionAlgorithm` now derive `PartialEq`, `Eq`, `PartialOrd` and `Ord`.
* Added unit testing for `Selection` trait.

# [1.1.0]

* Added benchmarks. You can run benchmarks using `cargo bench`. The benches are all inside `benches/`.
* Added `logger` feature to get information about the execution.
* Added `logger::VerbosityLevel` to filter the output.
* Added `logger::VerbosityType` to choose between only LOG, SAVE and LOG_AND_SAVE. 
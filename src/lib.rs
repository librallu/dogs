//! Discrete Optimization Global Search framework

// useful additional warnings if docs are missing, or crates imported but unused, etc.
#![warn(missing_debug_implementations)]
#![warn(missing_docs)]
#![warn(trivial_casts, trivial_numeric_casts)]
#![warn(unsafe_code)]
#![warn(unused_extern_crates)]
#![warn(variant_size_differences)]

// not sure if already by default in clippy
#![warn(clippy::similar_names)]
#![warn(clippy::print_stdout)]
#![warn(clippy::use_debug)]
#![warn(clippy::shadow_unrelated)]
#![warn(clippy::shadow_same)]
#![warn(clippy::shadow_reuse)]

// checks integer arithmetic in the project
// #![warn(clippy::integer_arithmetic)]

// these flags can be useful, but will indicate normal behavior
// #![warn(clippy::cast_possible_truncation)]
// #![warn(clippy::cast_possible_wrap)]
// #![warn(clippy::cast_precision_loss)]
// #![warn(clippy::cast_sign_loss)]

// files
/**
SearchAlgorithm trait definition + StoppingCriterion trait with some useful stopping criteria
*/ 
pub mod search_algorithm;

/**
Implementation of the search manager. Keeps the best-known objective and state of the algorithm.
*/
pub mod search_manager;

/**
Various search space related traits (SearchSpace, GuidedSpace, *etc.*)
*/
pub mod search_space;

/**
Implements the metric logger. Allows the algorithm to display logs of its performance through time.
*/
pub mod metric_logger;

/**
Search space decorator traits + procedural macros
*/
pub mod search_decorator;


// directories

/**
includes tree search algorithms
*/
pub mod tree_search;

/**
includes genetic algorithms
*/
pub mod genetic;

/**
includes local search algorithms
*/
pub mod local_search;

/**
various useful data-structures for search algorithms
*/
pub mod data_structures;


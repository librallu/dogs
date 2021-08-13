use std::time::SystemTime;
use std::cell::RefCell;
use std::rc::Rc;

use serde_json::json;

use crate::search_manager::SearchManager;

/**
 * Stopping criterion trait
 * Can be time limit, number of iterations, or else
 */
pub trait StoppingCriterion:Clone {
    /**
    true iff the search should finish.
    */
    fn is_finished(&self) -> bool;
}

/**
stopping criterion that never stops
*/
#[derive(Debug, Clone, Default)]
pub struct NeverStoppingCriterion {}

impl StoppingCriterion for NeverStoppingCriterion {
    fn is_finished(&self) -> bool { false }
}

/**
 * stops the search after a given amount of time searching
 */
#[derive(Debug, Clone)]
pub struct TimeStoppingCriterion {
    /// starting time 
    t_start: SystemTime,
    /// maximum time after the beginning
    t_max: f32,
}

impl TimeStoppingCriterion {
    /** stops after t_max time
    t_max: number of seconds the algorithm is allowed to run.
    */
    pub fn new(t_max:f32) -> Self {
        Self {
            t_start: SystemTime::now(),
            t_max,
        }
    }
}


impl StoppingCriterion for TimeStoppingCriterion {
    fn is_finished(&self) -> bool {
        self.t_start.elapsed().unwrap().as_secs_f32() >= self.t_max
    }
}

/**
 * A search algorithm has a "run" method that runs until a stopping_criterion is reached
 */
pub trait SearchAlgorithm<N, B> {
    /**
     * runs until the stopping_criterion is reached
     */
    fn run<SC:StoppingCriterion>(&mut self, stopping_criterion:SC);

    /**
        Gets the search manager of the algorithm.
        It allows to get the best found solution and its value, etc.
    */
    fn get_manager(&mut self) -> &mut SearchManager<N,B>;

    /**
     * returns true if the optimal value is found (thus we can stop the search). False by default
     */
    fn is_optimal(&self) -> bool { false }

    /**
    provides various statistics of events that occured during the search.
    */
    fn json_statistics(&self, json:&mut serde_json::Value) { json["is_optimal"] = json!(self.is_optimal()) }
}

/**
 indicates that the algorithm can be created using an integer parameter d
 (for instance beam search, MBA*, etc.)
 useful for iterative beam search, iterative MBA*, etc.
 */
pub trait BuildableWithInteger<Space> {
    /**
    constructor taking an integer as a parameter.
    */
    fn create_with_integer(s:Rc<RefCell<Space>>, d:usize) -> Self;
}
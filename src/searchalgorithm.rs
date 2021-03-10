use std::time::{SystemTime};
use std::cell::RefCell;
use std::rc::Rc;

use crate::searchmanager::SearchManager;

/**
 * Stopping criterion trait
 * Can be time limit, number of iterations, or else
 */
pub trait StoppingCriterion:Clone {
    fn is_finished(&self) -> bool;
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
    pub fn new(t_max:f32) -> Self {
        Self {
            t_start: SystemTime::now(),
            t_max: t_max,
        }
    }
}

impl StoppingCriterion for TimeStoppingCriterion {
    fn is_finished(&self) -> bool {
        return self.t_start.elapsed().unwrap().as_secs_f32() >= self.t_max;
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

    fn get_manager(&mut self) -> &mut SearchManager<N,B>;

    /**
     * returns true if the optimal value is found (thus we can stop the search)
     */
    fn is_optimal(&self) -> bool {
        return false;
    }
}

/**
 * can be created using a parameter d (for instance beam search, MBA*, etc.)
 */
pub trait BuildableWithInteger<Space> {
    fn create_with_integer(s:Rc<RefCell<Space>>, d:usize) -> Self;
}
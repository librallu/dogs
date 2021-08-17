/**
 * implements an iterative scheme. It helps to implement search algorithms
 * (for instance IterativeBeamSearch).
 */
use std::cell::RefCell;
use std::rc::{Weak,Rc};
use std::fmt::Display;
use std::marker::PhantomData;

use crate::search_space::SearchSpace;
use crate::search_manager::SearchManager;
use crate::metric_logger::{Metric, MetricLogger};
use crate::search_algorithm::{BuildableWithInteger, StoppingCriterion, SearchAlgorithm};

/**
An iterative search repetively builds a search algorithm that can be constructed using an integer
it progressively increases the number that represents the search effort.
*/
#[derive(Debug)]
pub struct IterativeSearch<N, B, Algo, Tree> {
    /// search manager of the iterative search
    pub manager: SearchManager<N, B>,
    space: Rc<RefCell<Tree>>,
    dinit: f64,
    growth: f64,
    logger: Weak<MetricLogger>,
    logging_id_msg: Option<usize>,
    is_optimal: bool,
    algo_phantom: PhantomData<Algo>,
}


impl<N:Clone, B: PartialOrd + Display + Copy, Algo, Tree> IterativeSearch<N, B, Algo, Tree> {
    /** constructs an iterated search from the search space, and the integer series defined
    as the initial value (dinit) and the geometric growth factor (growth).
    */
    pub fn new(space: Rc<RefCell<Tree>>, dinit: f64, growth: f64) -> Self {
        Self {
            manager: SearchManager::default(),
            space,
            dinit,
            growth,
            logger: Weak::new(),
            logging_id_msg: None,
            is_optimal: false,
            algo_phantom: PhantomData,
        }
    }

    /**
    binds the iterative search to a logger (to display the iteration number)
    */
    pub fn bind_logger(mut self, logger_ref:Weak<MetricLogger>) -> Self {
        if let Some(logger) = logger_ref.upgrade() {
            // adds headers to the logger
            let tmp = logger.register_headers([
                format!("{:<15}","Iter"),
            ].to_vec());
            self.logging_id_msg = Some(tmp[0]);
        }
        // registers the logger
        self.logger = logger_ref;
        self
    }
}

impl<N, B, Algo, Tree> SearchAlgorithm<N,B> for IterativeSearch<N, B, Algo, Tree>
where
    N:Clone,
    B:PartialOrd+Display+Copy,
    Algo:SearchAlgorithm<N, B>+BuildableWithInteger<Tree>,
    Tree:SearchSpace<N,B>,
{

    fn run<SC:StoppingCriterion>(&mut self, stopping_criterion:SC) {
        let mut d = self.dinit;
        while !stopping_criterion.is_finished() && !self.is_optimal() {
            self.space.borrow_mut().restart(format!("Iter D={}", d));
            // updates logger and display statistics
            if let Some(logger) = self.logger.upgrade() {
                if let Some(id) = self.logging_id_msg {
                    logger.update_metric(id, Metric::Text(format!("start D={}", d)));
                    logger.request_logging();
                    logger.update_metric(id, Metric::Text("".to_string()));
                }
            }
            let mut ts:Algo = Algo::create_with_integer(self.space.clone(), d as usize);
            // initializes the underlying beam search with best known solution
            self.manager.give_best(&mut ts.get_manager());
            ts.run(stopping_criterion.clone());
            ts.get_manager().give_best(&mut self.manager);
            // gets best
            d = (d * self.growth).ceil();
            if ts.is_optimal() {
                self.is_optimal = true;
                break
            }
        }
        self.space.borrow_mut().stop_search("".to_string());
    }

    fn is_optimal(&self) -> bool { self.is_optimal }

    fn get_manager(&mut self) -> &mut SearchManager<N,B> { &mut self.manager }

}


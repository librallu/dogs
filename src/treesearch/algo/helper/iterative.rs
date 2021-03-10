/**
 * implements an iterative scheme. It helps to implement search algorithms
 * (for instance IterativeBeamSearch).
 */
use std::cell::RefCell;
use std::rc::{Weak,Rc};
use std::fmt::Display;
use std::marker::PhantomData;

use crate::searchspace::SearchSpace;
use crate::searchmanager::SearchManager;
use crate::metriclogger::{Metric, MetricLogger};
use crate::searchalgorithm::{BuildableWithInteger, StoppingCriterion, SearchAlgorithm};


pub struct IterativeSearch<N, B, Algo, Sol, Tree> {
    pub manager: SearchManager<N, B>,
    space: Rc<RefCell<Tree>>,
    dinit: f64,
    growth: f64,
    logger: Weak<MetricLogger>,
    logging_id_msg: Option<usize>,
    is_optimal_: bool,
    algo_phantom: PhantomData<Algo>,
    sol_phantom: PhantomData<Sol>,
}


impl<N:Clone, B: PartialOrd + Display + Copy, Algo, Sol, Tree> IterativeSearch<N, B, Algo, Sol, Tree> {
    pub fn new(space: Rc<RefCell<Tree>>, dinit: f64, growth: f64) -> Self {
        Self {
            manager: SearchManager::new(),
            space: space,
            dinit: dinit,
            growth: growth,
            logger: Weak::new(),
            logging_id_msg: None,
            is_optimal_: false,
            algo_phantom: PhantomData,
            sol_phantom: PhantomData,
        }
    }

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
        return self;
    }
}

impl<N, B, Algo, Sol, Tree> SearchAlgorithm<N,B> for IterativeSearch<N, B, Algo, Sol, Tree>
where
    N:Clone,
    B:PartialOrd+Display+Copy,
    Algo:SearchAlgorithm<N, B>+BuildableWithInteger<Tree>,
    Tree:SearchSpace<N,Sol>,
{

    fn run<SC:StoppingCriterion>(&mut self, stopping_criterion:SC) {
        let mut d = self.dinit;
        while !stopping_criterion.is_finished() {
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
            d = ((d as f64) * self.growth).ceil();
            if ts.is_optimal() {
                self.is_optimal_ = true;
                break
            }
        }
        self.space.borrow_mut().stop_search("".to_string());
    }

    fn is_optimal(&self) -> bool {
        return self.is_optimal_;
    }

    fn get_manager(&mut self) -> &mut SearchManager<N,B> {
        return &mut self.manager;
    }

}


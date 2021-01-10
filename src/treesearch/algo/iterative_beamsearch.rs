use std::cmp::Ord;
use std::fmt::Display;
use std::rc::Weak;

use crate::treesearch::algo::beamsearch::BeamSearch;

use crate::searchmanager::SearchManager;
use crate::searchspace::{SearchSpace, GuidedSpace, SearchTree, TotalChildrenExpansion};
use crate::metriclogger::{Metric, MetricLogger};

pub struct IterativeBeamSearch<'a, Tree, N, B> {
    pub manager: SearchManager<N, B>,
    space: &'a mut Tree,
    dinit: usize,
    growth: f64,
    logger: Weak<MetricLogger>,
    logging_id_msg: Option<usize>,
    heuristic_pruning_done: bool,
}

impl<Tree, N:Clone, B: PartialOrd + Display + Copy> IterativeBeamSearch<'_, Tree, N, B> {
    pub fn new<'a>( space: &'a mut Tree, dinit: usize, growth: f64 ) -> IterativeBeamSearch<'a, Tree, N, B> {
        IterativeBeamSearch {
            manager: SearchManager::new(),
            space: space,
            dinit: dinit,
            growth: growth,
            logger: Weak::new(),
            logging_id_msg: None,
            heuristic_pruning_done: true,
        }
    }

    pub fn bind_logger(mut self, logger_ref:Weak<MetricLogger>) -> Self {
        if let Some(logger) = logger_ref.upgrade() {
            // adds headers to the logger
            let tmp = logger.register_headers([
                format!("{:<15}","IBS"),
            ].to_vec());
            self.logging_id_msg = Some(tmp[0]);
        }
        // registers the logger
        self.logger = logger_ref;
        return self;
    }

    pub fn run<S, G:Ord>(&mut self, stopping_criterion: impl Fn(&SearchManager<N, B>) -> bool + Copy)
    where
        Tree: SearchSpace<N,S>+GuidedSpace<N,G>+SearchTree<N, B>+TotalChildrenExpansion<N>,
    {
        let mut d = self.dinit;
        while stopping_criterion(&self.manager) {
            self.space.restart(format!("BEAM D={}", d));
            // updates logger and display statistics
            if let Some(logger) = self.logger.upgrade() {
                if let Some(id) = self.logging_id_msg {
                    logger.update_metric(id, Metric::Text(format!("start D={}", d)));
                    logger.request_logging();
                    logger.update_metric(id, Metric::Text("".to_string()));

                }
            }
            let mut ts = BeamSearch::new(self.space, d);
            // initializes the underlying beam search with best known solution
            self.manager.give_best(&mut ts.manager);
            ts.run(stopping_criterion);
            ts.manager.give_best(&mut self.manager);
            // gets best
            d = ((d as f64) * self.growth) as usize;
            if ! ts.is_heuristic_pruning_done() {
                self.heuristic_pruning_done = false;
                break
            }
        }
        self.space.stop_search("".to_string());
    }

    pub fn is_heuristic_pruning_done(&self) -> bool {
        return self.heuristic_pruning_done;
    }
}

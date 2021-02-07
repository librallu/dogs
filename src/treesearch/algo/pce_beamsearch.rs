use min_max_heap::MinMaxHeap;
use std::cmp::{Ord, PartialOrd};
use std::fmt::Display;
use std::rc::Weak;

use crate::searchmanager::SearchManager;
use crate::searchspace::{SearchSpace, GuidedSpace, SearchTree, PartialChildrenExpansion};
use crate::metriclogger::{Metric, MetricLogger};

use crate::treesearch::algo::helper::guided_node::GuidedNode;

pub struct PCEBeamSearch<'a, Tree, N, B> {
    pub manager: SearchManager<N, B>,
    space: &'a mut Tree,
    d: usize,
    heuristic_pruning_done: bool,
}

impl<'a, Tree, N:Clone, B:PartialOrd+Copy> PCEBeamSearch<'a, Tree, N, B> {
    pub fn new(space: &'a mut Tree, d: usize) -> Self {
        Self {
            manager: SearchManager::new(),
            space: space,
            d: d,
            heuristic_pruning_done: false,
        }
    }

    pub fn is_heuristic_pruning_done(&self) -> bool {
        return self.heuristic_pruning_done;
    }

    /**
     * BeamSearch version that takes advantage of the PartialChildrenExpansion.
     * Moreover, it takes advantage of having the beam search children sorted
     * At each level of the beam search, all parents are expanded once and kept with their last
     * children value. The algorithm expands the parent whose generated the smallest child.
     * The algorithm stops when no parent can expand a children that would be inserted into the
     * next level.
    */
    pub fn run<S, G: Ord+Clone>(&mut self, stopping_criterion: impl Fn(&SearchManager<N, B>) -> bool)
    where
        Tree: SearchSpace<N,S>+GuidedSpace<N,G>+SearchTree<N, B>+PartialChildrenExpansion<N>,
    {
        let mut beam = MinMaxHeap::with_capacity(self.d);
        let root = self.space.root();
        let g_root = self.space.guide(&root);
        self.heuristic_pruning_done = false;
        beam.push(GuidedNode::new(root, g_root));
        // for each level of the tree
        while stopping_criterion(&self.manager) && !beam.is_empty() {
            let mut next_beam = MinMaxHeap::with_capacity(self.d);
            while !beam.is_empty() && stopping_criterion(&self.manager) {
                // extract a parent node
                let mut n = beam.pop_min().unwrap().node;
                // check if goal
                if self.space.goal(&n) {
                    // compare with best
                    let v = self.space.bound(&n);
                    if self.manager.is_better(v) {
                        self.space.handle_new_best(&n);
                        self.manager.update_best(n, v);
                    }
                    continue;
                }
                // generate one child
                match self.space.get_next_child(&mut n) {
                    None => {},  // if no children, do nothing (discard the node)
                    Some(c) => {  // if a child, try to insert it
                        // check if goal
                        if self.space.goal(&c) {
                            // compare with best
                            let v = self.space.bound(&c);
                            if self.manager.is_better(v) {
                                self.space.handle_new_best(&c);
                                self.manager.update_best(c, v);
                            }
                            continue;
                        }
                        // add c to next_beam
                        let c_guide = self.space.guide(&c);
                        if next_beam.len() < self.d {
                            next_beam.push(GuidedNode::new(c, c_guide.clone()));
                            beam.push(GuidedNode::new(n, c_guide));
                        } else {
                            self.heuristic_pruning_done = true;
                            // pop max and insert child
                            if next_beam.peek_max().unwrap().guide > c_guide {
                                next_beam.push_pop_max(GuidedNode::new(c, c_guide.clone()));
                                beam.push(GuidedNode::new(n, c_guide.clone()));
                            }
                            
                        }
                    }
                }
            }
            beam = next_beam;
        }
        self.space.stop_search("".to_string());
    }
}


pub struct IterativePCEBeamSearch<'a, Tree, N, B> {
    pub manager: SearchManager<N, B>,
    space: &'a mut Tree,
    dinit: usize,
    growth: f64,
    logger: Weak<MetricLogger>,
    logging_id_msg: Option<usize>,
    heuristic_pruning_done: bool,
}


impl<'a, Tree, N:Clone, B: PartialOrd + Display + Copy> IterativePCEBeamSearch<'a, Tree, N, B> {
    pub fn new( space: &'a mut Tree, dinit: usize, growth: f64) -> Self {
        Self {
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

    pub fn run<S, G:Ord+Clone>(&mut self, stopping_criterion: impl Fn(&SearchManager<N, B>) -> bool + Copy)
    where
        Tree: SearchSpace<N,S>+GuidedSpace<N,G>+SearchTree<N, B>+PartialChildrenExpansion<N>,
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
            let mut ts = PCEBeamSearch::new(self.space, d);
            // initializes the underlying beam search with best known solution
            self.manager.give_best(&mut ts.manager);
            ts.run(stopping_criterion);
            ts.manager.give_best(&mut self.manager);
            // gets best
            d = ((d as f64) * self.growth).ceil() as usize;
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

use min_max_heap::MinMaxHeap;
use std::cmp::{Ord, PartialOrd};
use std::fmt::Display;
use std::rc::Weak;

use crate::searchmanager::SearchManager;
use crate::searchspace::{SearchSpace, GuidedSpace, SearchTree, TotalChildrenExpansion, ParetoDominanceSpace};
use crate::metriclogger::{Metric, MetricLogger};

use crate::treesearch::algo::helper::guided_node::GuidedNode;

#[derive(Clone)]
pub enum BeamDominanceScheme {
    Fixed(usize)
}

pub struct BeamSearchDom<'a, Tree, N, B> {
    pub manager: SearchManager<N, B>,
    space: &'a mut Tree,
    d: usize,
    heuristic_pruning_done: bool,
    beam_dominance_scheme: BeamDominanceScheme,
}

impl<'a, Tree, N:Clone, B:PartialOrd+Copy> BeamSearchDom<'a, Tree, N, B> {
    pub fn new(space: &'a mut Tree, d: usize, dom_scheme:BeamDominanceScheme) -> Self {
        Self {
            manager: SearchManager::new(),
            space: space,
            d: d,
            heuristic_pruning_done: false,
            beam_dominance_scheme: dom_scheme
        }
    }

    pub fn is_heuristic_pruning_done(&self) -> bool {
        return self.heuristic_pruning_done;
    }

    pub fn run<S, G: Ord+Clone>(&mut self, stopping_criterion: impl Fn(&SearchManager<N, B>) -> bool)
    where
        Tree: SearchSpace<N,S>+GuidedSpace<N,G>+SearchTree<N, B>+TotalChildrenExpansion<N>+ParetoDominanceSpace<N>,
    {
        let mut beam = MinMaxHeap::with_capacity(self.d);
        let root = self.space.root();
        let g_root = self.space.guide(&root);
        self.heuristic_pruning_done = false;
        beam.push(GuidedNode::new(root, g_root));
        while stopping_criterion(&self.manager) && !beam.is_empty() {
            let mut next_beam = MinMaxHeap::with_capacity(self.d);
            let mut elites:MinMaxHeap<GuidedNode<N,G>> = MinMaxHeap::new();
            while !beam.is_empty() && stopping_criterion(&self.manager) {
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
                let mut children = self.space.children(&mut n);
                while !children.is_empty() {
                    let c = children.pop().unwrap();
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
                    // check if n is dominated by the elite set
                    let mut is_dominated = false;
                    for e in elites.iter() {
                        if self.space.dominates(&e.node, &c) {
                            is_dominated = true;
                            continue;
                        }
                    }
                    if !is_dominated {
                        let c_guide = self.space.guide(&c);
                        // possibly: add n to the elite set
                        let elite_max_size:usize = match self.beam_dominance_scheme {
                            BeamDominanceScheme::Fixed(e) => e
                        };
                        if elites.len() < elite_max_size {
                            elites.push(GuidedNode::new(c.clone(), c_guide.clone()));
                        } else {
                            elites.push_pop_max(GuidedNode::new(c.clone(), c_guide.clone()));
                        }
                         // compute guide to feed the GuidedNode while inserting into next_beam
                        if next_beam.len() < self.d {
                            next_beam.push(GuidedNode::new(c, c_guide));
                        } else {
                            self.heuristic_pruning_done = true;
                            // pop max and insert child
                            next_beam.push_pop_max(GuidedNode::new(c, c_guide));
                        }
                    }
                }
            }
            beam = next_beam;
        }
        self.space.stop_search("".to_string());
    }
}

pub struct IterativeBeamSearchDom<'a, Tree, N, B> {
    pub manager: SearchManager<N, B>,
    space: &'a mut Tree,
    dinit: usize,
    growth: f64,
    logger: Weak<MetricLogger>,
    logging_id_msg: Option<usize>,
    heuristic_pruning_done: bool,
    beam_dominance_scheme: BeamDominanceScheme,
}


impl<'a, Tree, N:Clone, B: PartialOrd + Display + Copy> IterativeBeamSearchDom<'a, Tree, N, B> {
    pub fn new( space: &'a mut Tree, dinit: usize, growth: f64, dom:BeamDominanceScheme ) -> Self {
        Self {
            manager: SearchManager::new(),
            space: space,
            dinit: dinit,
            growth: growth,
            logger: Weak::new(),
            logging_id_msg: None,
            heuristic_pruning_done: true,
            beam_dominance_scheme: dom,
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
        Tree: SearchSpace<N,S>+GuidedSpace<N,G>+SearchTree<N, B>+TotalChildrenExpansion<N>+ParetoDominanceSpace<N>,
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
            let mut ts = BeamSearchDom::new(self.space, d, self.beam_dominance_scheme.clone());
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

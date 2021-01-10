use min_max_heap::MinMaxHeap;
use std::cmp::{Ord, PartialOrd};

use crate::searchmanager::SearchManager;
use crate::searchspace::{SearchSpace, GuidedSpace, SearchTree, TotalChildrenExpansion};

use crate::treesearch::algo::helper::guided_node::GuidedNode;

pub struct BeamSearch<'a, Tree, N, B> {
    pub manager: SearchManager<N, B>,
    space: &'a mut Tree,
    d: usize,
    heuristic_pruning_done: bool,
}

impl<'a, Tree, N:Clone, B:PartialOrd+Copy> BeamSearch<'a, Tree, N, B> {
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

    pub fn run<S, G: Ord>(&mut self, stopping_criterion: impl Fn(&SearchManager<N, B>) -> bool)
    where
        Tree: SearchSpace<N,S>+GuidedSpace<N,G>+SearchTree<N, B>+TotalChildrenExpansion<N>,
    {
        let mut beam = MinMaxHeap::with_capacity(self.d);
        let root = self.space.root();
        let g_root = self.space.guide(&root);
        self.heuristic_pruning_done = false;
        beam.push(GuidedNode::new(root, g_root));
        while stopping_criterion(&self.manager) && !beam.is_empty() {
            let mut next_beam = MinMaxHeap::with_capacity(self.d);
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
                    let c_guide = self.space.guide(&c); // compute guide to feed the GuidedNode while inserting into next_beam
                    if next_beam.len() < self.d {
                        next_beam.push(GuidedNode::new(c, c_guide));
                    } else {
                        self.heuristic_pruning_done = true;
                        // pop max and insert child
                        next_beam.push_pop_max(GuidedNode::new(c, c_guide));
                    }
                }
            }
            beam = next_beam;
        }
        self.space.stop_search("".to_string());
    }
}

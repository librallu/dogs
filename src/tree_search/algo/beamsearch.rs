use min_max_heap::MinMaxHeap;
use std::cmp::{Ord, PartialOrd};
use std::marker::PhantomData;
use std::cell::RefCell;
use std::rc::Rc;
use std::fmt::Display;

use crate::search_manager::SearchManager;
use crate::search_algorithm::{BuildableWithInteger, SearchAlgorithm, StoppingCriterion};
use crate::search_space::{SearchSpace, GuidedSpace, TotalNeighborGeneration};

use crate::tree_search::algo::helper::guided_node::GuidedNode;
use crate::tree_search::algo::helper::iterative::IterativeSearch;

pub struct BeamSearch<N, B, G, Tree> {
    pub manager: SearchManager<N, B>,
    tree: Rc<RefCell<Tree>>,
    d: usize,
    heuristic_pruning_done: bool,
    g: PhantomData<G>,}

impl<N:Clone, B:PartialOrd+Copy, G, Tree> BeamSearch<N, B, G, Tree> {
    pub fn new(tree: Rc<RefCell<Tree>>, d: usize) -> Self {
        Self {
            manager: SearchManager::new(),
            tree: tree,
            d: d,
            heuristic_pruning_done: false,
            g: PhantomData,
        }
    }
}


impl<'a, N, B, G:Ord, Tree> SearchAlgorithm<N, B> for BeamSearch<N, B, G, Tree>
where
    N: Clone,
    B: PartialOrd+Copy,
    Tree: SearchSpace<N,B> + GuidedSpace<N,G> + TotalNeighborGeneration<N>,
{
    /**
     * runs until the stopping_criterion is reached
     */
    fn run<SC:StoppingCriterion>(&mut self, stopping_criterion:SC)
    where SC:StoppingCriterion,  {
        let mut space = self.tree.borrow_mut();
        let mut beam = MinMaxHeap::with_capacity(self.d);
        let root = space.initial();
        let g_root = space.guide(&root);
        self.heuristic_pruning_done = false;
        beam.push(GuidedNode::new(root, g_root));
        while !stopping_criterion.is_finished() && !beam.is_empty() {
            let mut next_beam = MinMaxHeap::with_capacity(self.d);
            while !beam.is_empty() && !stopping_criterion.is_finished() {
                let mut n = beam.pop_min().unwrap().node;
                // check if goal
                if space.goal(&n) {
                    // compare with best
                    let v = space.bound(&n);
                    if self.manager.is_better(v) {
                        let n2 = space.handle_new_best(n);
                        let b2 = space.bound(&n2);
                        n = n2.clone();
                        self.manager.update_best(n2, b2);
                    }
                }
                let mut children = space.neighbors(&mut n);
                while !children.is_empty() {
                    let c = children.pop().unwrap();
                    // check if goal
                    if space.goal(&c) {
                        // compare with best
                        let v = space.bound(&c);
                        if self.manager.is_better(v) {
                            let c2 = space.handle_new_best(c);
                            let b2 = space.bound(&c2);
                            self.manager.update_best(c2, b2);
                        }
                        continue;
                    }
                    let c_guide = space.guide(&c); // compute guide to feed the GuidedNode while inserting into next_beam
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
        space.stop_search("".to_string());
    }

    fn get_manager(&mut self) -> &mut SearchManager<N, B> {
        return &mut self.manager;
    }

    /**
     * returns true if the optimal value is found (thus we can stop the search)
     */
    fn is_optimal(&self) -> bool {
        return !self.heuristic_pruning_done;
    }
}

impl<N, B, G, Tree> BuildableWithInteger<Tree> for BeamSearch<N, B, G, Tree>
where N:Clone, B:PartialOrd+Copy {
    fn create_with_integer(tree: Rc<RefCell<Tree>>, d:usize) -> Self {
        Self::new(tree, d)
    }
}

/**
 * creates an iterative beam search algorithm
 */
pub fn create_iterative_beam_search<N, B, G, Tree>(space:Rc<RefCell<Tree>>, d_init:f64, growth:f64)
-> IterativeSearch<N, B, BeamSearch<N, B, G, Tree>, Tree>
where N:Clone, B:Copy+PartialOrd+Display {
    return IterativeSearch::new(space, d_init, growth);
}
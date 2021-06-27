use min_max_heap::MinMaxHeap;
use std::cmp::{Ord, PartialOrd};
use std::marker::PhantomData;
use std::fmt::Display;
use std::cell::RefCell;
use std::rc::Rc;

use crate::search_manager::SearchManager;
use crate::search_space::{SearchSpace, GuidedSpace, PartialNeighborGeneration};
use crate::search_algorithm::{BuildableWithInteger, SearchAlgorithm, StoppingCriterion};

use crate::tree_search::algo::helper::guided_node::GuidedNode;
use crate::tree_search::algo::helper::iterative::IterativeSearch;

/**
implements a partial expansion beam search
*/
#[derive(Debug)]
pub struct PEBeamSearch<N, B, G, Tree> {
    /// search manager
    pub manager: SearchManager<N, B>,
    space: Rc<RefCell<Tree>>,
    d: usize,
    heuristic_pruning_done: bool,
    g: PhantomData<G>,
}


impl<N:Clone, B:PartialOrd+Copy, G, Tree> PEBeamSearch<N, B, G, Tree> {
    /** builds the algorithm using the search space and the beam width */
    pub fn new(space: Rc<RefCell<Tree>>, d: usize) -> Self {
        Self {
            manager: SearchManager::default(),
            space,
            d,
            heuristic_pruning_done: false,
            g: PhantomData,
        }
    }
}


impl<'a, N, B, G:Ord+Clone, Tree> SearchAlgorithm<N, B> for PEBeamSearch<N, B, G, Tree>
where
    N: Clone,
    B: PartialOrd+Copy,
    Tree: SearchSpace<N,B> + GuidedSpace<N,G> + PartialNeighborGeneration<N>,
{
    /**
     * BeamSearch version that takes advantage of the PartialChildrenExpansion.
     * Moreover, it takes advantage of having the beam search children sorted
     * At each level of the beam search, all parents are expanded once and kept with their last
     * children value. The algorithm expands the parent whose generated the smallest child.
     * The algorithm stops when no parent can expand a children that would be inserted into the
     * next level.
    */
    fn run<SC:StoppingCriterion>(&mut self, stopping_criterion:SC)
    where SC:StoppingCriterion,  {
        let mut space = self.space.borrow_mut();
        let mut beam = MinMaxHeap::with_capacity(self.d);
        let root = space.initial();
        let g_root = space.guide(&root);
        self.heuristic_pruning_done = false;
        beam.push(GuidedNode::new(root, g_root));
        // for each level of the tree
        while !stopping_criterion.is_finished() && !beam.is_empty() {
            let mut next_beam = MinMaxHeap::with_capacity(self.d);
            while !beam.is_empty() && !stopping_criterion.is_finished() {
                // extract a parent node
                let mut n = beam.pop_min().unwrap().node;
                // check if goal
                if space.goal(&n) {
                    // compare with best
                    let v = space.bound(&n);
                    if self.manager.is_better(v) {
                        let n2 = space.handle_new_best(n);
                        let b2 = space.bound(&n2);
                        self.manager.update_best(n2, b2);
                    }
                    continue;
                }
                // generate one child
                match space.next_neighbor(&mut n) {
                    None => {},  // if no children, do nothing (discard the node)
                    Some(c) => {  // if a child, try to insert it
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
                        // add c to next_beam
                        let c_guide = space.guide(&c);
                        if next_beam.len() < self.d {
                            next_beam.push(GuidedNode::new(c, c_guide.clone()));
                            beam.push(GuidedNode::new(n, c_guide.clone()));
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
        space.stop_search("".to_string());
    }


    fn get_manager(&mut self) -> &mut SearchManager<N, B> { &mut self.manager }

    /**
     * returns true if the optimal value is found (thus we can stop the search)
     */
    fn is_optimal(&self) -> bool { !self.heuristic_pruning_done }
}


impl<N, B, G, Tree> BuildableWithInteger<Tree> for PEBeamSearch<N, B, G, Tree>
where N:Clone, B:PartialOrd+Copy {
    fn create_with_integer(space: Rc<RefCell<Tree>>, d:usize) -> Self {
        Self::new(space, d)
    }
}

/**
 * creates an iterative beam search algorithm
 */
pub fn create_iterative_pce_beam_search<N, B, G, Tree>(space:Rc<RefCell<Tree>>, d_init:f64, growth:f64)
-> IterativeSearch<N, B, PEBeamSearch<N, B, G, Tree>, Tree>
where N:Clone, B:Copy+PartialOrd+Display {
    IterativeSearch::new(space, d_init, growth)
}


    
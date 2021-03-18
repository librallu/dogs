use min_max_heap::MinMaxHeap;
use std::cmp::{Ord, PartialOrd};
use std::marker::PhantomData;
use std::fmt::Display;
use std::cell::RefCell;
use std::rc::Rc;

use crate::searchmanager::SearchManager;
use crate::searchspace::{SearchSpace, GuidedSpace, SearchTree, PartialChildrenExpansion};
use crate::searchalgorithm::{BuildableWithInteger, SearchAlgorithm, StoppingCriterion};

use crate::treesearch::algo::helper::guided_node::GuidedNode;
use crate::treesearch::algo::helper::iterative::IterativeSearch;

pub struct PCEBeamSearch<N, B, G, Sol, Tree> {
    pub manager: SearchManager<N, B>,
    tree: Rc<RefCell<Tree>>,
    d: usize,
    heuristic_pruning_done: bool,
    g: PhantomData<G>,
    sol: PhantomData<Sol>,
}


impl<N:Clone, B:PartialOrd+Copy, G, Sol, Tree> PCEBeamSearch<N, B, G, Sol, Tree> {
    pub fn new(tree: Rc<RefCell<Tree>>, d: usize) -> Self {
        Self {
            manager: SearchManager::new(),
            tree: tree,
            d: d,
            heuristic_pruning_done: false,
            g: PhantomData,
            sol: PhantomData,
        }
    }
}


impl<'a, N, B, G:Ord+Clone, Sol, Tree> SearchAlgorithm<N, B> for PCEBeamSearch<N, B, G, Sol, Tree>
where
    N: Clone,
    B: PartialOrd+Copy,
    Tree: SearchSpace<N,Sol> + GuidedSpace<N,G> + SearchTree<N,B> + PartialChildrenExpansion<N>,
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
        let mut space = self.tree.borrow_mut();
        let mut beam = MinMaxHeap::with_capacity(self.d);
        let root = space.root();
        let g_root = space.guide(&root);
        self.heuristic_pruning_done = false;
        beam.push(GuidedNode::new(root, g_root));
        // for each level of the tree
        let mut i = 0;
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
                        space.handle_new_best(&n);
                        self.manager.update_best(n, v);
                    }
                    continue;
                }
                // generate one child
                match space.get_next_child(&mut n) {
                    None => {},  // if no children, do nothing (discard the node)
                    Some(c) => {  // if a child, try to insert it
                        // check if goal
                        if space.goal(&c) {
                            // compare with best
                            let v = space.bound(&c);
                            if self.manager.is_better(v) {
                                space.handle_new_best(&c);
                                self.manager.update_best(c, v);
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


impl<N, B, G, Sol, Tree> BuildableWithInteger<Tree> for PCEBeamSearch<N, B, G, Sol, Tree>
where N:Clone, B:PartialOrd+Copy {
    fn create_with_integer(tree: Rc<RefCell<Tree>>, d:usize) -> Self {
        Self::new(tree, d)
    }
}

/**
 * creates an iterative beam search algorithm
 */
pub fn create_iterative_pce_beam_search<N, B, G, Sol, Tree>(space:Rc<RefCell<Tree>>, d_init:f64, growth:f64)
-> IterativeSearch<N, B, PCEBeamSearch<N, B, G, Sol, Tree>, Sol, Tree>
where N:Clone, B:Copy+PartialOrd+Display {
    return IterativeSearch::new(space, d_init, growth);
}


    
use min_max_heap::MinMaxHeap;
use std::cmp::{Ord, PartialOrd};
use std::marker::PhantomData;
use std::cell::RefCell;
use std::rc::Rc;

use crate::search_manager::SearchManager;
use crate::search_algorithm::{StoppingCriterion, SearchAlgorithm};
use crate::tree_search::algo::helper::guided_node::GuidedNode;
use crate::search_space::{
    SearchSpace, GuidedSpace, TotalNeighborGeneration, ParetoDominanceSpace
};

/**
 * implements beam dominance schemes. i.e. the policy to keep track of "elite" nodes that will be
 * used to eliminate dominated nodes.
 */
#[derive(Debug, Clone)]
pub enum BeamDominanceScheme {
    /// keep a maximum number of elite nodes
    Fixed(usize)
}

/** beam search with pareto-dominance scheme */
#[derive(Debug)]
pub struct BeamSearchDom<N, B, G, Space> {
    manager: SearchManager<N, B>,
    space: Rc<RefCell<Space>>,
    d: usize,
    heuristic_pruning_done: bool,
    beam_dominance_scheme: BeamDominanceScheme,
    g: PhantomData<G>,
}

impl<Space, N:Clone, B:PartialOrd+Copy, G:Ord> BeamSearchDom<N, B, G, Space> {
    /** builds the beam search using the search space, the beam width, and the dominance scheme */
    pub fn new(space: Rc<RefCell<Space>>, d: usize, dom_scheme:BeamDominanceScheme) -> Self {
        Self {
            manager: SearchManager::default(),
            space,
            d,
            heuristic_pruning_done: false,
            beam_dominance_scheme: dom_scheme,
            g: PhantomData,
        }
    }
}

impl<'a, N, B, G, Space> SearchAlgorithm<N, B> for BeamSearchDom<N, B, G, Space>
where
    N: Clone,
    B: PartialOrd+Copy,
    G: Ord+Clone,
    Space: SearchSpace<N,B> + GuidedSpace<N,G> + TotalNeighborGeneration<N> + ParetoDominanceSpace<N>,
{
    fn run<SC:StoppingCriterion>(&mut self, stopping_criterion:SC) {
        let mut space = self.space.borrow_mut();
        let mut beam = MinMaxHeap::with_capacity(self.d);
        let root = space.initial();
        let g_root = space.guide(&root);
        self.heuristic_pruning_done = false;
        beam.push(GuidedNode::new(root, g_root));
        while !stopping_criterion.is_finished() && !beam.is_empty() {
            let mut next_beam = MinMaxHeap::with_capacity(self.d);
            let mut elites:MinMaxHeap<GuidedNode<N,G>> = MinMaxHeap::new();
            while !beam.is_empty() && !stopping_criterion.is_finished() {
                let mut n = beam.pop_min().unwrap().node;
                // check if goal
                if space.goal(&n) {
                    // compare with best
                    let v = space.bound(&n);
                    if self.manager.is_better(v) {
                        let n2 = space.handle_new_best(n);
                        n = n2.clone();
                        let b2 = space.bound(&n2);
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
                    // check if n is dominated by the elite set
                    let mut is_dominated = false;
                    for e in elites.iter() {
                        if space.dominates(&e.node, &c) {
                            is_dominated = true;
                            continue;
                        }
                    }
                    if !is_dominated {
                        let c_guide = space.guide(&c);
                        // possibly: add n to the elite set
                        let BeamDominanceScheme::Fixed(elite_max_size) = self.beam_dominance_scheme;
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
        space.stop_search("".to_string());
    }

    fn get_manager(&mut self) -> &mut SearchManager<N, B> { &mut self.manager }

    /**
     * returns true if the optimal value is found (thus we can stop the search)
     */
    fn is_optimal(&self) -> bool { !self.heuristic_pruning_done }
}
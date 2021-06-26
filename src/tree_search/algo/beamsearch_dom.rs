use min_max_heap::MinMaxHeap;
use std::cmp::{Ord, PartialOrd};

use crate::search_manager::SearchManager;
use crate::search_space::{SearchSpace, GuidedSpace, TotalNeighborGeneration, ParetoDominanceSpace};

use crate::tree_search::algo::helper::guided_node::GuidedNode;

/**
 * implements beam dominance schemes. i.e. the policy to keep track of "elite" nodes that will be
 * used to eliminate dominated nodes.
 * TODO: replace with the new beam search format
 */
#[derive(Debug, Clone)]
pub enum BeamDominanceScheme {
    /// keep a maximum number of elite nodes
    Fixed(usize)
}

/** beam search with pareto-dominance scheme */
#[derive(Debug)]
pub struct BeamSearchDom<'a, Tree, N, B> {
    /// search manager
    pub manager: SearchManager<N, B>,
    space: &'a mut Tree,
    d: usize,
    heuristic_pruning_done: bool,
    beam_dominance_scheme: BeamDominanceScheme,
}

impl<'a, Tree, N:Clone, B:PartialOrd+Copy> BeamSearchDom<'a, Tree, N, B> {
    /** builds the beam search using the search space, the beam width, and the dominance scheme */
    pub fn new(space: &'a mut Tree, d: usize, dom_scheme:BeamDominanceScheme) -> Self {
        Self {
            manager: SearchManager::default(),
            space,
            d,
            heuristic_pruning_done: false,
            beam_dominance_scheme: dom_scheme
        }
    }

    pub fn is_heuristic_pruning_done(&self) -> bool { self.heuristic_pruning_done }

    pub fn run<G: Ord+Clone>(&mut self, stopping_criterion: impl Fn(&SearchManager<N, B>) -> bool)
    where
        Tree: SearchSpace<N,B>+GuidedSpace<N,G>+TotalNeighborGeneration<N>+ParetoDominanceSpace<N>,
    {
        let mut beam = MinMaxHeap::with_capacity(self.d);
        let root = self.space.initial();
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
                        let n2 = self.space.handle_new_best(n);
                        n = n2.clone();
                        let b2 = self.space.bound(&n2);
                        self.manager.update_best(n2, b2);
                    }
                }
                let mut children = self.space.neighbors(&mut n);
                while !children.is_empty() {
                    let c = children.pop().unwrap();
                    // check if goal
                    if self.space.goal(&c) {
                        // compare with best
                        let v = self.space.bound(&c);
                        if self.manager.is_better(v) {
                            let c2 = self.space.handle_new_best(c);
                            let b2 = self.space.bound(&c2);
                            self.manager.update_best(c2, b2);
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
        self.space.stop_search("".to_string());
    }
}
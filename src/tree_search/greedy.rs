use std::cmp::{Ord, PartialOrd};
use std::marker::PhantomData;
use std::cell::RefCell;
use std::rc::Rc;

use rand::prelude::{SliceRandom, ThreadRng};

use crate::search_manager::SearchManager;
use crate::search_space::{SearchSpace, GuidedSpace, TotalNeighborGeneration};
use crate::search_algorithm::{SearchAlgorithm, StoppingCriterion};

/**
implements a partial expansion greedy algorithm
*/
#[derive(Debug)]
pub struct Greedy<N, B, G, Tree> {
    /// search manager
    pub manager: SearchManager<N, B>,
    tree: Rc<RefCell<Tree>>,
    g: PhantomData<G>,
    rng: ThreadRng,
}

impl<N:Clone, B:PartialOrd+Copy, G, Tree> Greedy<N, B, G, Tree> {
    /** builds the partial expansion greedy using the search space */
    pub fn new(tree: Rc<RefCell<Tree>>) -> Self {
        Self {
            manager: SearchManager::default(),
            tree,
            g: PhantomData,
            rng: rand::thread_rng(),
        }
    }
}

impl<'a, N, B, G:Ord+Clone, Tree> SearchAlgorithm<N, B> for Greedy<N, B, G, Tree>
where
    N: Clone,
    B: PartialOrd+Copy,
    Tree: SearchSpace<N,B> + GuidedSpace<N,G> + TotalNeighborGeneration<N>,
{

    fn run<SC:StoppingCriterion>(&mut self, stopping_criterion:SC) where SC:StoppingCriterion {
        let mut space = self.tree.borrow_mut();
        let mut n = space.initial();
        while !stopping_criterion.is_finished() {
            // check if goal
            let is_goal = space.goal(&n);
            if is_goal {
                // compare with best
                let v = space.bound(&n);
                if self.manager.is_better(v) {
                    let n2 = space.handle_new_best(n.clone());
                    self.manager.update_best(n2.clone(), space.bound(&n2));
                }
                // break;
            }
            // get the neighbor with minimum guide (break ties randomly)
            let mut best_val = None;
            let mut bests = Vec::new();
            for neigh in space.neighbors(&mut n) {
                match &best_val {
                    None => {
                        best_val = Some(space.guide(&neigh));
                        bests.clear();
                        bests.push(neigh);
                    },
                    Some(v) => {
                        let g = space.guide(&neigh);
                        match g.cmp(v) {
                            std::cmp::Ordering::Less => {
                                bests.clear();
                                bests.push(neigh);
                                best_val = Some(g);
                            },
                            std::cmp::Ordering::Equal => {
                                bests.push(neigh);
                            },
                            std::cmp::Ordering::Greater => {},
                        };
                    }
                }
            }
            if bests.is_empty() {
                break;
            } else {
                n = bests.choose(&mut self.rng).unwrap().clone();
            }
            // match space.neighbors(&mut n).iter().min_by_key(|neigh| space.guide(neigh)) {
            //     None => { break; } // no more neighbors (stop),
            //     Some(neigh) => { n = neigh.clone(); }
            // }
        }
        space.stop_search("".to_string());
    }


    fn get_manager(&mut self) -> &mut SearchManager<N, B> { &mut self.manager }

    /**
     * returns true if the optimal value is found (thus we can stop the search).
     * For this greedy, we set it to always false (it is not destined to prove optimality)
     */
    fn is_optimal(&self) -> bool { false }
}
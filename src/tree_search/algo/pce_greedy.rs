use std::cmp::{Ord, PartialOrd};
use std::marker::PhantomData;
use std::cell::RefCell;
use std::rc::Rc;

use crate::search_manager::SearchManager;
use crate::search_space::{SearchSpace, GuidedSpace, PartialNeighborGeneration};
use crate::search_algorithm::{SearchAlgorithm, StoppingCriterion};

pub struct PCEGreedy<N, B, G, Tree> {
    pub manager: SearchManager<N, B>,
    tree: Rc<RefCell<Tree>>,
    g: PhantomData<G>,
}

impl<N:Clone, B:PartialOrd+Copy, G, Tree> PCEGreedy<N, B, G, Tree> {
    pub fn new(tree: Rc<RefCell<Tree>>) -> Self {
        Self {
            manager: SearchManager::default(),
            tree,
            g: PhantomData,
        }
    }
}

impl<'a, N, B, G:Ord+Clone, Tree> SearchAlgorithm<N, B> for PCEGreedy<N, B, G, Tree>
where
    N: Clone,
    B: PartialOrd+Copy,
    Tree: SearchSpace<N,B> + GuidedSpace<N,G> + PartialNeighborGeneration<N>,
{

    fn run<SC:StoppingCriterion>(&mut self, stopping_criterion:SC)
    where SC:StoppingCriterion,  {
        let mut space = self.tree.borrow_mut();
        let mut n = space.initial();
        while !stopping_criterion.is_finished() {
            // check if goal
            let is_goal = space.goal(&n);
            if is_goal {
                // compare with best
                let v = space.bound(&n);
                if self.manager.is_better(v) {
                    let n2 = space.handle_new_best(n);
                    self.manager.update_best(n2.clone(), space.bound(&n2));
                }
                break;
            }
            // if not, explore children until a feasible children is found
            let tmp;
            match space.next_neighbor(&mut n) {
                None => { break; },  // stop if no more children
                Some(c) => { tmp = c; }
            }
            n = tmp;
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
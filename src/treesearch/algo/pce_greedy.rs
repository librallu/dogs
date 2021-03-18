use std::cmp::{Ord, PartialOrd};
use std::marker::PhantomData;
use std::cell::RefCell;
use std::rc::Rc;

use crate::searchmanager::SearchManager;
use crate::searchspace::{SearchSpace, GuidedSpace, SearchTree, PartialChildrenExpansion};
use crate::searchalgorithm::{SearchAlgorithm, StoppingCriterion};

pub struct PCEGreedy<N, B, G, Sol, Tree> {
    pub manager: SearchManager<N, B>,
    tree: Rc<RefCell<Tree>>,
    g: PhantomData<G>,
    sol: PhantomData<Sol>,
}

impl<N:Clone, B:PartialOrd+Copy, G, Sol, Tree> PCEGreedy<N, B, G, Sol, Tree> {
    pub fn new(tree: Rc<RefCell<Tree>>) -> Self {
        Self {
            manager: SearchManager::new(),
            tree: tree,
            g: PhantomData,
            sol: PhantomData,
        }
    }
}

impl<'a, N, B, G:Ord+Clone, Sol, Tree> SearchAlgorithm<N, B> for PCEGreedy<N, B, G, Sol, Tree>
where
    N: Clone,
    B: PartialOrd+Copy,
    Tree: SearchSpace<N,Sol> + GuidedSpace<N,G> + SearchTree<N,B> + PartialChildrenExpansion<N>,
{

    fn run<SC:StoppingCriterion>(&mut self, stopping_criterion:SC)
    where SC:StoppingCriterion,  {
        let mut space = self.tree.borrow_mut();
        let mut n = space.root();
        while !stopping_criterion.is_finished() {
            // check if goal
            let is_goal = space.goal(&n);
            if is_goal {
                // compare with best
                let v = space.bound(&n);
                if self.manager.is_better(v) {
                    space.handle_new_best(&n);
                    self.manager.update_best(n.clone(), v);
                }
                break;
            }
            // if not, explore children until a feasible children is found
            let tmp;
            match space.get_next_child(&mut n) {
                None => { break; },  // stop if no more children
                Some(c) => { tmp = c; }
            }
            n = tmp;
        }
        space.stop_search("".to_string());
    }


    fn get_manager(&mut self) -> &mut SearchManager<N, B> { return &mut self.manager; }

    /**
     * returns true if the optimal value is found (thus we can stop the search).
     * For this greedy, we set it to always false (it is not destined to prove optimality)
     */
    fn is_optimal(&self) -> bool { return false; }
}
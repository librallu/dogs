use std::collections::LinkedList;
use std::marker::PhantomData;
use std::cell::RefCell;
use std::rc::Rc;


use crate::search_manager::SearchManager;
use crate::search_algorithm::{StoppingCriterion, SearchAlgorithm};
use crate::search_space::{SearchSpace, GuidedSpace, TotalNeighborGeneration};

/** defines a Depth First Search structure */
#[derive(Debug)]
pub struct DepthFirstSearch<N, B, G, Space> {
    manager: SearchManager<N, B>,
    space: Rc<RefCell<Space>>,
    g: PhantomData<G>,
    optimal_found: bool,
}

impl<Space, N:Clone, B:PartialOrd+Copy, G:Ord> DepthFirstSearch<N, B, G, Space> {
    /**
    creates a best first search given a search space.
    */
    pub fn new(space: Rc<RefCell<Space>>) -> Self {
        Self {
            manager: SearchManager::default(),
            space,
            g: PhantomData,
            optimal_found: false,
        }
    }
}


impl<'a, N, B, G, Space> SearchAlgorithm<N, B> for DepthFirstSearch<N, B, G, Space>
where
    N: Clone,
    B: PartialOrd+Copy,
    G: Ord+Clone,
    Space: SearchSpace<N,B> + GuidedSpace<N,G> + TotalNeighborGeneration<N>,
{
    fn run<SC:StoppingCriterion>(&mut self, stopping_criterion:SC) {
        let mut space = self.space.borrow_mut();
        let mut stack = LinkedList::new();
        stack.push_back(space.initial());
        while !stopping_criterion.is_finished() && !stack.is_empty() {
            let mut n = stack.pop_front().unwrap();
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
            // if not, add all its children
            let mut children = space.neighbors(&mut n);
            children.sort_by_key(|e| space.guide(e));
            while !children.is_empty() {
                stack.push_front(children.pop().unwrap());
            }
        }
        space.stop_search("".to_string());
    }

    fn get_manager(&mut self) -> &mut SearchManager<N, B> { &mut self.manager }

    /**
     * returns true if the optimal value is found (thus we can stop the search)
     */
    fn is_optimal(&self) -> bool { self.optimal_found }
}

use std::collections::LinkedList;

use crate::search_manager::SearchManager;
use crate::search_space::{SearchSpace, GuidedSpace, TotalNeighborGeneration};

// TODO: new format
pub struct Dfs<'a, Tree, N, B> {
    pub manager: SearchManager<N, B>,
    space: &'a mut Tree,
}

impl<'a, Tree, N:Clone, B: PartialOrd + Copy> Dfs<'a, Tree, N, B> {
    pub fn new(space: &'a mut Tree) -> Dfs<Tree, N, B> {
        Self {
            manager: SearchManager::default(),
            space,
        }
    }

    pub fn run<G>(&mut self, stopping_criterion: impl Fn(&SearchManager<N, B>) -> bool)
    where
        Tree: SearchSpace<N,B>+GuidedSpace<N,G>+TotalNeighborGeneration<N>,
        G: Ord
    {
        let mut stack = LinkedList::new();
        stack.push_back(self.space.initial());
        while stopping_criterion(&self.manager) && !stack.is_empty() {
            let mut n = stack.pop_front().unwrap();
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
            // if not, add all its children
            let mut children = self.space.neighbors(&mut n);
            children.sort_by_key(|e| self.space.guide(e));
            while !children.is_empty() {
                stack.push_front(children.pop().unwrap());
            }
        }
        self.space.stop_search("".to_string());
    }
}

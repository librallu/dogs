use std::collections::LinkedList;

use crate::searchmanager::SearchManager;
use crate::searchspace::{SearchSpace, GuidedSpace, SearchTree, TotalChildrenExpansion};

pub struct DFS<'a, Tree, N, B> {
    pub manager: SearchManager<N, B>,
    space: &'a mut Tree,
}

impl<'a, Tree, N:Clone, B: PartialOrd + Copy> DFS<'a, Tree, N, B> {
    pub fn new(space: &'a mut Tree) -> DFS<Tree, N, B> {
        Self {
            manager: SearchManager::new(),
            space: space,
        }
    }

    pub fn run<S, G>(&mut self, stopping_criterion: impl Fn(&SearchManager<N, B>) -> bool)
    where
        Tree: SearchSpace<N,S>+GuidedSpace<N,G>+SearchTree<N, B>+TotalChildrenExpansion<N>,
        G: Ord
    {
        let mut stack = LinkedList::new();
        stack.push_back(self.space.root());
        while stopping_criterion(&self.manager) && !stack.is_empty() {
            let mut n = stack.pop_front().unwrap();
            // check if goal
            if self.space.goal(&n) {
                // compare with best
                let v = self.space.bound(&n);
                if self.manager.is_better(v) {
                    self.space.handle_new_best(&n);
                    self.manager.update_best(n, v);
                }
            } else {
                // if not, add all its children
                let mut children = self.space.children(&mut n);
                children.sort_by_key(|e| self.space.guide(e));
                while !children.is_empty() {
                    stack.push_front(children.pop().unwrap());
                }
            }
        }
        self.space.stop_search("".to_string());
    }
}

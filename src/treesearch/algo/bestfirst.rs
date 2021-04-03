use std::collections::BinaryHeap;
use std::cmp::Reverse;

use crate::searchmanager::SearchManager;
use crate::searchspace::{SearchSpace, GuidedSpace, SearchTree, TotalChildrenExpansion};
use crate::treesearch::algo::helper::guided_node::GuidedNode;

pub struct BestFirst<'a, Tree, N, B> {
    pub manager: SearchManager<N, B>,
    space: &'a mut Tree,
}

impl<'a, Tree, N:Clone, B: PartialOrd + Copy> BestFirst<'a, Tree, N, B> {
    pub fn new(space: &'a mut Tree) -> Self {
        Self {
            manager: SearchManager::new(),
            space: space,
        }
    }

    pub fn run<S, G>(&mut self, stopping_criterion: impl Fn(&SearchManager<N, B>) -> bool)
    where Tree: SearchSpace<N,S>+GuidedSpace<N,G>+SearchTree<N, B>+TotalChildrenExpansion<N>, G: Ord {
        let mut pq = BinaryHeap::new();
        let root = self.space.root();
        let g_root = self.space.guide(&root);
        pq.push(Reverse(GuidedNode::new(root, g_root)));
        while stopping_criterion(&self.manager) && !pq.is_empty() {
            let mut n = pq.pop().unwrap().0.node;
            // check if goal
            if self.space.goal(&n) {
                // compare with best
                let v = self.space.bound(&n);
                if self.manager.is_better(v) {
                    let n2 = self.space.handle_new_best(n);
                    let b2 = self.space.bound(&n2);
                    self.manager.update_best(n2, b2);
                }
            } else {
                // if not, add all its children
                let mut children = self.space.children(&mut n);
                while !children.is_empty() {
                    let c = children.pop().unwrap();
                    let g_c = self.space.guide(&c);
                    pq.push(Reverse(GuidedNode::new(c, g_c)));
                }
            }
        }
        self.space.stop_search("".to_string());
    }
}

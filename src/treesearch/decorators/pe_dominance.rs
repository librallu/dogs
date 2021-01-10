use std::cmp::PartialOrd;
use std::collections::hash_map::Entry;
use std::hash::Hash;

extern crate fxhash;
use fxhash::FxHashMap;

extern crate human_format;

use crate::searchspace::{SearchSpace, GuidedSpace, PrefixEquivalenceTree, SearchTree, TotalChildrenExpansion};

struct DominanceInfo<B> {
    val: B,
    iter: u32,
}

/// Prefix Equivalence Dominance Decorator
pub struct PEDominanceTsDecorator<Tree, PE, B> {
    s: Tree,
    store: FxHashMap<PE, DominanceInfo<B>>,
    current_iter: u32,
    nb_prunings: u64,
    nb_gets: u64,
    nb_updates: u64,
}

impl<N,G,Tree,PE,B> GuidedSpace<N,G> for PEDominanceTsDecorator<Tree, PE, B>
where Tree:GuidedSpace<N,G>
{
    fn guide(&mut self, n: &N) -> G {
        return self.s.guide(n);
    }
}

impl<N,Sol,Tree,PE,B> SearchSpace<N,Sol> for PEDominanceTsDecorator<Tree, PE, B>
where Tree:SearchSpace<N,Sol>, B:serde::Serialize
{
    fn solution(&mut self, node: &N) -> Sol {
        return self.s.solution(node);
    }

    fn restart(&mut self, msg: String) {
        self.current_iter += 1;
        self.s.restart(msg);
    }

    fn handle_new_best(&mut self, n: &N) {
        self.s.handle_new_best(n);
    }

    fn stop_search(&mut self, _msg: String) {
        self.s.stop_search(_msg);
    }

    fn display_statistics(&self) {
        let format = |e| human_format::Formatter::new().with_decimals(1).format(e);
        println!("{:>25}{:>15}", "nb elts", format(self.store.len() as f64));
        println!("{:>25}{:>15}", "nb gets", format(self.nb_gets as f64));
        println!("{:>25}{:>15}", "nb pruned", format(self.nb_prunings as f64));
        println!("{:>25}{:>15}", "nb updates", format(self.nb_updates as f64));
        println!();
        self.s.display_statistics();
    }

    fn export_statistics(&self, json:&mut serde_json::Value) {
        json["pe_nb_elts"] = serde_json::json!(self.store.len());
        json["pe_nb_gets"] = serde_json::json!(self.nb_gets);
        json["pe_nb_pruned"] = serde_json::json!(self.nb_prunings);
        json["pe_nb_updates"] = serde_json::json!(self.nb_updates);
        self.s.export_statistics(json);
    }
}

impl<N, Tree, PE, B> TotalChildrenExpansion<N> for PEDominanceTsDecorator<Tree, PE, B>
where 
    Tree: TotalChildrenExpansion<N>+PrefixEquivalenceTree<N, B, PE>,
    PE: Eq + Hash,
    B: PartialOrd
{
    fn children(&mut self, node: &mut N) -> Vec<N> {
        // check if current node is dominated, otherwise, return children of underlying node
        let pe = self.s.get_pe(node);
        let prefix_bound = self.s.prefix_bound(node);
        self.nb_gets += 1;
        match self.store.entry(pe) {
            Entry::Occupied(o) => {
                // if the prefix equivalence exists in the database
                let info = o.into_mut();
                if info.val < prefix_bound
                    || (info.val == prefix_bound && info.iter == self.current_iter)
                {
                    self.nb_prunings += 1;
                    return Vec::new(); // if node dominated
                } else {
                    // otherwise, add it to the database (update)
                    info.val = prefix_bound;
                    info.iter = self.current_iter;
                    self.nb_updates += 1;
                    return self.s.children(node);
                }
            }
            Entry::Vacant(v) => {
                // otherwise, add it to the database (new entry)
                v.insert(DominanceInfo {
                    val: prefix_bound,
                    iter: self.current_iter,
                });
                return self.s.children(node);
            }
        }
    }
}

impl<N, B, Tree, PE> SearchTree<N, B> for PEDominanceTsDecorator<Tree, PE, B>
where
    Tree: SearchTree<N, B>,
{
    fn root(&mut self) -> N {
        return self.s.root();
    }

    fn bound(&mut self, node: &N) -> B {
        return self.s.bound(node);
    }

    fn goal(&mut self, node: &N) -> bool {
        return self.s.goal(node);
    }
}

impl<Tree, PE, B> PEDominanceTsDecorator<Tree, PE, B> {
    pub fn unwrap(&self) -> &Tree {
        return &self.s;
    }

    pub fn new(s: Tree) -> PEDominanceTsDecorator<Tree, PE, B> {
        PEDominanceTsDecorator {
            s: s,
            store: FxHashMap::default(),
            current_iter: 0,
            nb_prunings: 0,
            nb_gets: 0,
            nb_updates: 0,
        }
    }
}

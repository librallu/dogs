use std::cmp::PartialOrd;
use std::collections::hash_map::Entry;
use std::hash::Hash;

extern crate fxhash;
use fxhash::FxHashMap;

extern crate human_format;

use crate::searchspace::{SearchSpace, GuidedSpace, PrefixEquivalenceTree, SearchTree, TotalChildrenExpansion, ParetoDominanceSpace, PartialChildrenExpansion};


#[derive(Debug)]
pub struct DominanceInfo<B> {
    val: B,
    iter: u32,
}

#[derive(Debug)]
pub struct DominanceStore<PE, B> {
    name: String,
    store: FxHashMap<PE, DominanceInfo<B>>,
    nb_gets: usize,
    nb_dominations: usize,
    nb_updates: usize,
}

impl<PE, B> DominanceStore<PE, B> where PE:Eq+Hash, B:PartialOrd {
    pub fn new(name:String) -> Self {
        Self {
            name: name,
            store: FxHashMap::default(),
            nb_gets: 0,
            nb_dominations: 0,
            nb_updates: 0
        }
    }

    pub fn is_dominated_or_add(&mut self, pe:PE, b:B, iter:u32) -> bool {
        self.nb_gets += 1;
        match self.store.entry(pe) {
            Entry::Occupied(o) => {
                // if the prefix equivalence exists in the database
                let info = o.into_mut();
                if info.val < b
                    || (info.val == b && info.iter == iter)
                {
                    self.nb_dominations += 1;
                    return true; // if node dominated
                } else {
                    // otherwise, add it to the database (update)
                    info.val = b;
                    info.iter = iter;
                    self.nb_updates += 1;
                    return false;
                }
            }
            Entry::Vacant(v) => {
                // otherwise, add it to the database (new entry)
                v.insert(DominanceInfo {
                    val: b,
                    iter: iter,
                });
                return false;
            }
        }
    }

    pub fn display_statistics(&self) {
        let format = |e| human_format::Formatter::new().with_decimals(1).format(e);
        println!("{} dominances:", self.name);
        println!("{:>25}{:>15}", "nb elts", format(self.store.len() as f64));
        println!("{:>25}{:>15}", "nb gets", format(self.nb_gets as f64));
        println!("{:>25}{:>15}", "nb pruned", format(self.nb_dominations as f64));
        println!("{:>25}{:>15}", "nb updates", format(self.nb_updates as f64));
    }

    pub fn export_statistics(&self, json:&mut serde_json::Value) {
        json["pe_nb_elts"] = serde_json::json!(self.store.len());
        json["pe_nb_gets"] = serde_json::json!(self.nb_gets);
        json["pe_nb_pruned"] = serde_json::json!(self.nb_dominations);
        json["pe_nb_updates"] = serde_json::json!(self.nb_updates);
    }
}



/// Prefix Equivalence Dominance Decorator
pub struct PEDominanceTsDecorator<Tree, PE, B> {
    s: Tree,
    current_iter: u32,
    store: DominanceStore<PE, B>,
}

impl<N,G,Tree,PE,B> GuidedSpace<N,G> for PEDominanceTsDecorator<Tree, PE, B>
where Tree:GuidedSpace<N,G>
{
    fn guide(&mut self, n: &N) -> G {
        return self.s.guide(n);
    }
}

impl<N,Sol,Tree,PE,B> SearchSpace<N,Sol> for PEDominanceTsDecorator<Tree, PE, B>
where Tree:SearchSpace<N,Sol>, B:serde::Serialize+PartialOrd, PE:Hash+Eq
{
    fn solution(&mut self, node: &N) -> Sol {
        return self.s.solution(node);
    }

    fn restart(&mut self, msg: String) {
        self.current_iter += 1;
        self.s.restart(msg);
    }

    fn handle_new_best(&mut self, n: N) -> N {
        self.s.handle_new_best(n)
    }

    fn stop_search(&mut self, _msg: String) {
        self.s.stop_search(_msg);
    }

    fn display_statistics(&self) {
        self.store.display_statistics();
        println!();
        self.s.display_statistics();
    }

    fn export_statistics(&self, json:&mut serde_json::Value) {
        self.store.export_statistics(json);
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
        let bound = self.s.prefix_bound(node);
        if self.store.is_dominated_or_add(pe, bound, self.current_iter) {
            return Vec::new();
        } else {
            return self.s.children(node);
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

impl<Tree, PE, B> PEDominanceTsDecorator<Tree, PE, B> where PE:Hash+Eq, B:PartialOrd {
    pub fn unwrap(&self) -> &Tree {
        return &self.s;
    }

    pub fn new(s: Tree) -> Self {
        Self {
            s: s,
            store: DominanceStore::new("".to_string()),
            current_iter: 0,
        }
    }
}

impl<N,Tree,PE,B> ParetoDominanceSpace<N> for PEDominanceTsDecorator<Tree, PE, B>
where Tree: ParetoDominanceSpace<N>
{
    fn dominates(&self, a:&N, b:&N) -> bool {
        return self.s.dominates(a,b);
    }
}

impl<N,Tree,PE,B> PartialChildrenExpansion<N> for PEDominanceTsDecorator<Tree, PE, B>
where
    Tree: PartialChildrenExpansion<N>+PrefixEquivalenceTree<N, B, PE>,
    PE: Eq + Hash,
    B: PartialOrd
{
    fn get_next_child(&mut self, node: &mut N) -> Option<N> {
        match self.s.get_next_child(node) {
            None => { return None; },
            Some(c) => {
                // checks if c is dominated
                let pe = self.s.get_pe(&c);
                let prefix_bound = self.s.prefix_bound(&c);
                if self.store.is_dominated_or_add(pe, prefix_bound, self.current_iter) {
                    return self.get_next_child(node); // if node dominated, try another one
                } else {
                    return Some(c);
                }
            }
        }
    }
}
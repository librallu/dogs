use std::cmp::PartialOrd;
use std::collections::hash_map::Entry;
use std::hash::Hash;

use fxhash::FxHashMap;

use crate::search_space::{SearchSpace, GuidedSpace, TotalNeighborGeneration, PartialNeighborGeneration, Identifiable, ParetoDominanceSpace, ToSolution};


/**
implements a dominance information used to represent a previous state (g-cost and iter number)
*/
#[derive(Debug)]
pub struct DominanceInfo<B> {
    val: B,
    iter: u32,
}

/**
stores dominances and counts the number of gets/dominations/updates
*/
#[derive(Debug)]
pub struct DominanceStore<Id, B> {
    store: FxHashMap<Id, DominanceInfo<B>>,
    nb_gets: usize,
    nb_dominations: usize,
    nb_updates: usize,
}

impl<Id, B> Default for DominanceStore<Id, B> where Id:Eq+Hash, B:PartialOrd {
    /** builds the dominance store  */
    fn default() -> Self {
        Self {
            store: FxHashMap::default(),
            nb_gets: 0,
            nb_dominations: 0,
            nb_updates: 0
        }
    }
}

impl<Id, B> DominanceStore<Id, B> where Id:Eq+Hash, B:PartialOrd {
    /**
    returns true if the information is dominated, or insert it and returns false.
    */
    pub fn is_dominated_or_add(&mut self, pe:Id, b:B, iter:u32) -> bool {
        self.nb_gets += 1;
        match self.store.entry(pe) {
            Entry::Occupied(o) => {
                // if the prefix equivalence exists in the database
                let info = o.into_mut();
                if info.val < b
                    || (info.val == b && info.iter == iter)
                {
                    self.nb_dominations += 1;
                    true // if node dominated
                } else {
                    // otherwise, add it to the database (update)
                    info.val = b;
                    info.iter = iter;
                    self.nb_updates += 1;
                    false
                }
            }
            Entry::Vacant(v) => {
                // otherwise, add it to the database (new entry)
                v.insert(DominanceInfo {
                    val: b,
                    iter,
                });
                false
            }
        }
    }

    /** displays statistics of the search */
    pub fn display_statistics(&self) {
        let format = |e| human_format::Formatter::new().with_decimals(1).format(e);
        println!("{:>25}{:>15}", "nb elts", format(self.store.len() as f64));
        println!("{:>25}{:>15}", "nb gets", format(self.nb_gets as f64));
        println!("{:>25}{:>15}", "nb pruned", format(self.nb_dominations as f64));
        println!("{:>25}{:>15}", "nb updates", format(self.nb_updates as f64));
    }

    /** exports statistics to a JSON format */
    pub fn export_statistics(&self, json:&mut serde_json::Value) {
        json["pe_nb_elts"] = serde_json::json!(self.store.len());
        json["pe_nb_gets"] = serde_json::json!(self.nb_gets);
        json["pe_nb_pruned"] = serde_json::json!(self.nb_dominations);
        json["pe_nb_updates"] = serde_json::json!(self.nb_updates);
    }
}



/// Prefix Equivalence Dominance Decorator
#[derive(Debug)]
pub struct GcostDominanceTsDecorator<Space, Id, B> {
    s: Space,
    current_iter: u32,
    store: DominanceStore<Id, B>,
}

impl<N,G,Space,Id,B> GuidedSpace<N,G> for GcostDominanceTsDecorator<Space, Id, B>
where Space:GuidedSpace<N,G>
{
    fn guide(&mut self, n: &N) -> G { self.s.guide(n) }
}

impl<N,Sol,Space,Id,B> ToSolution<N,Sol> for GcostDominanceTsDecorator<Space, Id, B>
where Space:ToSolution<N,Sol> {
    fn solution(&mut self, node: &mut N) -> Sol { self.s.solution(node) }
}

impl<N,Space,Id,B> SearchSpace<N,B> for GcostDominanceTsDecorator<Space, Id, B>
where Space:SearchSpace<N,B>, B:serde::Serialize+PartialOrd, Id:Hash+Eq
{
    fn initial(&mut self) -> N { self.s.initial() }

    fn bound(&mut self, node: &N) -> B { self.s.bound(node) }

    fn g_cost(&mut self, node: &N) -> B { self.s.g_cost(node) }

    fn goal(&mut self, node: &N) -> bool { self.s.goal(node) }

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

impl<N, Space, Id, B> TotalNeighborGeneration<N> for GcostDominanceTsDecorator<Space, Id, B>
where 
    Space: TotalNeighborGeneration<N>+Identifiable<N, Id>+SearchSpace<N,B>,
    Id: Eq + Hash,
    B: PartialOrd
{
    fn neighbors(&mut self, node: &mut N) -> Vec<N> {
        // check if current node is dominated, otherwise, return neighbors of underlying node
        let pe = self.s.id(node);
        let bound = self.s.g_cost(node);
        if self.store.is_dominated_or_add(pe, bound, self.current_iter) { Vec::new() }
        else { self.s.neighbors(node) }
    }
}


impl<Space, Id, B> GcostDominanceTsDecorator<Space, Id, B> where Id:Hash+Eq, B:PartialOrd {
    /** unwraps itself */
    pub fn unwrap(&self) -> &Space { &self.s }

    /** builds the decorator around a search space */
    pub fn new(s: Space) -> Self {
        Self {
            s,
            store: DominanceStore::default(),
            current_iter: 0,
        }
    }
}

impl<N,Space,Id,B> ParetoDominanceSpace<N> for GcostDominanceTsDecorator<Space, Id, B>
where Space: ParetoDominanceSpace<N>
{
    fn dominates(&self, a:&N, b:&N) -> bool { self.s.dominates(a,b) }
}

impl<N,Space,Id,B> PartialNeighborGeneration<N> for GcostDominanceTsDecorator<Space, Id, B>
where
    Space: PartialNeighborGeneration<N>+Identifiable<N, Id>+SearchSpace<N,B>,
    Id: Eq + Hash,
    B: PartialOrd
{
    fn next_neighbor(&mut self, node: &mut N) -> Option<N> {
        match self.s.next_neighbor(node) {
            None => { None },
            Some(c) => {
                // checks if c is dominated
                let id = self.s.id(&c);
                let prefix_bound = self.s.g_cost(&c);
                if self.store.is_dominated_or_add(id, prefix_bound, self.current_iter) {
                    self.next_neighbor(node) // if node dominated, try another one
                } else { Some(c) }
            }
        }
    }
}
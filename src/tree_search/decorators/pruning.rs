use serde_json::json;

use crate::search_space::{
    SearchSpace,
    GuidedSpace,
    TotalNeighborGeneration,
    PartialNeighborGeneration,
    Identifiable,
    ParetoDominanceSpace,
    ToSolution
};
use crate::search_decorator::SearchSpaceDecorator;

/**
pruning decorator: stores the best known solution and counts the number of prunings for statistics.
*/
#[derive(Debug)]
pub struct PruningDecorator<Space, B> {
    s: Space,
    best_val: Option<B>,
    nb_prunings: u64
}

impl<N,G,Space,B> GuidedSpace<N,G> for PruningDecorator<Space, B> 
where Space:GuidedSpace<N,G>
{
    fn guide(&mut self, n: &N) -> G { self.s.guide(n) }
}

impl<N,Sol,Space,B> ToSolution<N,Sol> for PruningDecorator<Space, B>
where Space:ToSolution<N,Sol> {
    fn solution(&mut self, node: &mut N) -> Sol { self.s.solution(node) }
}

impl<N,Space,B> SearchSpace<N,B> for PruningDecorator<Space,B>
where Space:SearchSpace<N,B>, B:serde::Serialize+PartialOrd+Clone
{
    fn initial(&mut self) -> N { self.s.initial() }

    fn bound(&mut self, n: &N) -> B { self.s.bound(n) }

    fn g_cost(&mut self, n: &N) -> B { self.s.g_cost(n) }

    fn goal(&mut self, n: &N) -> bool {
        let res = self.s.goal(n);
        if res {
            let eval_n = self.s.bound(n);
            match self.best_val.clone() {  // check if the best-known should be updated
                None => { self.best_val = Some(eval_n) }
                Some(v) => {
                    if eval_n < v {
                        self.best_val = Some(eval_n);
                    } 
                }
            }
        }
        res
    }

    fn restart(&mut self, msg: String) {
        self.s.restart(msg);
    }

    fn handle_new_best(&mut self, n: N) -> N {
        self.s.handle_new_best(n)
    }

    fn stop_search(&mut self, _msg: String) {
        self.s.stop_search(_msg);
    }

    fn display_statistics(&self) {
        println!("{:>25}{:>15}", "nb pruned", self.nb_prunings);
        println!();
        self.s.display_statistics();
    }

    fn json_statistics(&self, json:&mut serde_json::Value) {
        json["nb_pruned"] = serde_json::json!(self.nb_prunings);
        match &self.best_val {
            None => {},
            Some(v) => { json["primal_bound"] = json!(v); }
        }
        self.s.json_statistics(json);
    }
}


impl<N, Space, B> TotalNeighborGeneration<N> for PruningDecorator<Space,B>
where 
    Space: TotalNeighborGeneration<N> + SearchSpace<N, B>,
    B: PartialOrd+Copy,
{

    fn neighbors(&mut self, n: &mut N) -> Vec<N> {
        match self.best_val {
            None => { self.s.neighbors(n) }
            Some(best_v) => {
                let mut res = Vec::new();
                let mut children = self.s.neighbors(n);
                let children_size = children.len();
                while !children.is_empty() {
                    let child = children.pop().unwrap();
                    if self.s.bound(&child) < best_v {
                        res.push(child);
                    }
                }
                self.nb_prunings += children_size as u64 - res.len() as u64;
                res
            }
        }
    }
}

impl<Space, B> SearchSpaceDecorator<Space> for PruningDecorator<Space, B> {
    fn unwrap(&self) -> &Space { &self.s }
}


impl<Space, B> PruningDecorator<Space, B> {
    /** builds the decorator around a search space */
    pub fn new(s: Space) -> Self {
        Self {s, best_val: None, nb_prunings: 0}
    }
}

impl<N, B, Id, Space> Identifiable<N, Id> for PruningDecorator<Space, B>
where
    Space: Identifiable<N, Id>,
{
    fn id(&self, n: &mut N) -> Id { self.s.id(n) }
}


impl<N,Space,B> ParetoDominanceSpace<N> for PruningDecorator<Space, B>
where Space: ParetoDominanceSpace<N>
{
    fn dominates(&self, a:&N, b:&N) -> bool { self.s.dominates(a,b) }
}


impl<N, Space, B> PartialNeighborGeneration<N> for PruningDecorator<Space,B>
where 
    Space: PartialNeighborGeneration<N>+SearchSpace<N,B>,
    B: PartialOrd+Copy,
{
    fn next_neighbor(&mut self, node: &mut N) -> Option<N> {
        match self.s.next_neighbor(node) {
            None => { None },
            Some(c) => {
                match self.best_val {
                    None => { Some(c) }
                    Some(best_v) => {
                        if self.s.bound(&c) < best_v { Some(c) }
                        else {
                            self.nb_prunings += 1;
                            self.next_neighbor(node)
                        }
                    }
                }
                
            }
        }
    }
}